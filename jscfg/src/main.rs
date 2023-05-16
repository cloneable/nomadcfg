use clap::Parser;
use jrsonnet_evaluator::{
    apply_tla, error::Result as JrResult, gc::GcHashMap, trace::PathResolver, FileImportResolver,
    State,
};
use jrsonnet_parser::IStr;
use jrsonnet_stdlib::{ContextInitializer, YamlFormat};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    schema: Option<PathBuf>,
}

pub fn main() -> JrResult<()> {
    let opts = Opts::parse();

    let yaml = eval(opts.config)?;

    println!("{yaml}");

    Ok(())
}

pub fn eval<P: AsRef<Path>>(path: P) -> JrResult<String> {
    let manifest_format = Box::new(YamlFormat::cli(2, true));
    let state = State::default();
    state.set_import_resolver(FileImportResolver::default());

    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    // ctx.add_ext_str("bar".into(), "bar-value".into());
    state.set_context_initializer(ctx);

    // let mut tla = GcHashMap::<IStr, IStr>::new();
    // tla.insert("foo".into(), "foo-value".into());

    let val = state.import(path)?;
    // let val = apply_tla(state.clone(), &tla, val)?;
    val.manifest(manifest_format)
}
