use jrsonnet_evaluator::{trace::PathResolver, FileImportResolver, State};
use jrsonnet_stdlib::ContextInitializer;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Object {
    mybool: Option<bool>,
    mystr: String,
    myfloat: Option<i8>,
    moo: char,
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let state = State::default();
    state.set_import_resolver(FileImportResolver::default());

    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    state.set_context_initializer(ctx);

    let val = state.import("examples/simple.jsonnet")?;

    let obj: Object = serde_jrsonnet::from_val(&val)?;

    println!("{obj:?}");

    Ok(())
}
