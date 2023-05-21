use clap::Parser;
use jrsonnet_evaluator::{trace::PathResolver, FileImportResolver, State};
use jrsonnet_stdlib::ContextInitializer;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(short, long, value_name = "FILE")]
    spec: PathBuf,
}

pub fn main() -> Result<(), Box<dyn Error + 'static>> {
    let opts = Opts::parse();

    let state = State::default();
    state.set_import_resolver(FileImportResolver::default());

    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    // ctx.add_ext_str("bar".into(), "bar-value".into());
    state.set_context_initializer(ctx);

    // let mut tla = GcHashMap::<IStr, IStr>::new();
    // tla.insert("foo".into(), "foo-value".into());

    let val = state.import(opts.spec)?;
    // let val = apply_tla(state.clone(), &tla, val)?;

    let spec: nomadapi::types::Job = serde_jrsonnet::from_val(&val)?;

    let output = serde_yaml::to_string(&spec)?;

    println!("{output}");

    Ok(())
}
