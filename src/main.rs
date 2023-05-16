use jrsonnet_evaluator::{
    apply_tla, error::Result as JrResult, gc::GcHashMap, trace::PathResolver, FileImportResolver,
    State,
};
use jrsonnet_parser::IStr;
use jrsonnet_stdlib::{ContextInitializer, YamlFormat};

pub fn eval() -> JrResult<()> {
    let manifest_format = Box::new(YamlFormat::cli(2, true));
    let state = State::default();
    state.set_import_resolver(FileImportResolver::default());

    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    ctx.add_ext_str("bar".into(), "bar-value".into());
    state.set_context_initializer(ctx);

    let mut tla = GcHashMap::<IStr, IStr>::new();
    tla.insert("foo".into(), "foo-value".into());

    let val = state.import("config.jsonnet")?;
    let val = apply_tla(state.clone(), &tla, val)?;
    let output = val.manifest(manifest_format)?;

    println!("{output}");

    Ok(())
}

pub fn main() -> JrResult<()> {
    eval()?;

    Ok(())
}
