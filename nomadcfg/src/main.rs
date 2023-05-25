use clap::Parser;
use jrsonnet_evaluator::{trace::PathResolver, FileImportResolver, State};
use jrsonnet_stdlib::ContextInitializer;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf, process};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(long, value_name = "FILE")]
    spec: PathBuf,

    #[arg(long, default_value = "latest", value_name = "TAG")]
    imagetag: String,

    #[arg(long)]
    yaml: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Jobspec {
    #[serde(rename = "Job")]
    job: nomadapi::types::Job,
}

pub fn main() -> Result<(), Box<dyn Error + 'static>> {
    let opts = Opts::parse();

    let state = State::default();
    state.set_import_resolver(FileImportResolver::default());

    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    ctx.add_ext_str("imagetag".into(), opts.imagetag.into());
    state.set_context_initializer(ctx);

    // let mut tla = GcHashMap::<IStr, IStr>::new();
    // tla.insert("foo".into(), "foo-value".into());

    let val = state.import(opts.spec)?;
    // let val = apply_tla(state.clone(), &tla, val)?;

    let spec: Jobspec = match serde_jrsonnet::from_val(&val) {
        Ok(spec) => spec,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };

    let output = if opts.yaml {
        serde_yaml::to_string(&spec)?
    } else {
        serde_json::to_string_pretty(&spec)?
    };

    println!("{output}");

    Ok(())
}
