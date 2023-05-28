mod deserializer;
mod error;
mod nomadapi;

use clap::{Parser, ValueEnum};
use jrsonnet_evaluator::{
    trace::{ExplainingFormat, PathResolver},
    FileImportResolver, State, Val,
};
use jrsonnet_stdlib::ContextInitializer;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(long, value_name = "FILE")]
    spec: PathBuf,

    #[arg(long, value_name = "JOB ID")]
    job_id: Option<String>,

    #[arg(long, default_value = "latest", value_name = "TAG")]
    imagetag: String,

    #[arg(long, value_enum, rename_all = "lower", default_value_t = Format::Json)]
    format: Format,

    #[arg(long)]
    error_on_unknown_field: bool,

    #[arg(long)]
    unnested_job: bool,
}

#[derive(Clone, ValueEnum)]
enum Format {
    Json,
    Yaml,
    Toml,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Jobspec {
    #[serde(rename = "Job")]
    job: nomadapi::types::Job,
}

pub fn main() -> Result<(), Error> {
    let opts = Opts::parse();

    match run(opts) {
        Err(Error::SerdeJrsonnet(error::Error::Evaluator(e))) | Err(Error::Jrsonnet(e)) => {
            use jrsonnet_evaluator::trace::TraceFormat;

            let trace = Box::new(ExplainingFormat {
                resolver: PathResolver::new_cwd_fallback(),
                max_trace: 10,
            });
            let mut out = String::new();
            trace.write_trace(&mut out, &e).expect("format error");
            eprintln!("{out}");
            process::exit(1);
        }
        r => r,
    }
}

fn run(opts: Opts) -> Result<(), Error> {
    let state = State::default();
    state.set_import_resolver(FileImportResolver::default());

    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    ctx.add_ext_str("imagetag".into(), opts.imagetag.into());
    state.set_context_initializer(ctx);

    // let mut tla = GcHashMap::<IStr, IStr>::new();
    // tla.insert("foo".into(), "foo-value".into());

    let val = state.import(opts.spec)?;
    // let val = apply_tla(state.clone(), &tla, val)?;

    // TODO: only eval requested job
    let mut jobspecs = Vec::<Jobspec>::new();
    match val {
        Val::Arr(ref jobs) => {
            for job in jobs.iter() {
                let val: Val = job?;
                if opts.unnested_job {
                    jobspecs.push(Jobspec {
                        job: deserializer::from_val(&val, opts.error_on_unknown_field)?,
                    });
                } else {
                    jobspecs.push(deserializer::from_val(&val, opts.error_on_unknown_field)?);
                }
            }
        }
        Val::Obj(_) => {
            if opts.unnested_job {
                jobspecs.push(Jobspec {
                    job: deserializer::from_val(&val, opts.error_on_unknown_field)?,
                });
            } else {
                jobspecs.push(deserializer::from_val(&val, opts.error_on_unknown_field)?);
            }
        }
        _ => {
            return Err(Error::ExpectedJobOrArrayOfJobs);
        }
    }

    let mut found = false;
    for spec in &jobspecs {
        if opts.job_id.is_none()
            || opts.job_id.as_ref().unwrap()
                == spec.job.id.as_ref().unwrap_or(&"*UNSET*".to_owned())
        {
            found = true;
            let output = match opts.format {
                Format::Json => serde_json::to_string_pretty(&spec)?,
                Format::Yaml => serde_yaml::to_string(&spec)?,
                Format::Toml => toml::to_string_pretty(&spec)?,
            };
            println!("{output}");
        }
    }

    if !found {
        eprintln!("No job(s) found.");
        process::exit(1);
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no job(s) found")]
    NoJobsFound,

    #[error("expected job or array of jobs")]
    ExpectedJobOrArrayOfJobs,

    #[error("jrsonnet error: {0}")]
    Jrsonnet(#[from] jrsonnet_evaluator::Error),

    #[error("serde_jrsonnet error: {0}")]
    SerdeJrsonnet(#[from] error::Error),

    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("serde_yaml error: {0}")]
    SerdeYaml(#[from] serde_yaml::Error),

    #[error("toml error: {0}")]
    Toml(#[from] toml::ser::Error),
}
