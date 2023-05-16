use clap::Parser;
use jrsonnet_evaluator::{manifest::JsonFormat, trace::PathResolver, FileImportResolver, State};
use jrsonnet_stdlib::ContextInitializer;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use valico::json_schema::{self, ValidationState};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    schema: Option<PathBuf>,
}

pub fn main() -> Result<(), Box<dyn Error + 'static>> {
    let opts = Opts::parse();

    let json = evaluate(opts.config)?;

    if let Some(schema) = opts.schema {
        let state = validate(schema, &json)?;
        if !state.is_strictly_valid() {
            eprintln!("Errors: {:?}", state.errors);
        }
    }

    println!("{json}");

    Ok(())
}

fn evaluate<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error + 'static>> {
    let manifest_format = Box::new(JsonFormat::cli(2, true));
    let state = State::default();
    state.set_import_resolver(FileImportResolver::default());

    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    // ctx.add_ext_str("bar".into(), "bar-value".into());
    state.set_context_initializer(ctx);

    // let mut tla = GcHashMap::<IStr, IStr>::new();
    // tla.insert("foo".into(), "foo-value".into());

    let val = state.import(path)?;
    // let val = apply_tla(state.clone(), &tla, val)?;
    let output = val.manifest(manifest_format)?;

    Ok(output)
}

fn validate<P: AsRef<Path>>(
    schema: P,
    config: &str,
) -> Result<ValidationState, Box<dyn Error + 'static>> {
    let schema = serde_json::from_reader(File::open(schema)?)?;
    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(schema, true)?;

    let config: serde_json::Value = serde_json::from_str(config)?;

    Ok(schema.validate(&config))
}
