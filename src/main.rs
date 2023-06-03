#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    // clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    // clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    deprecated,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]

mod deserializer;
mod error;
mod nomadapi;

use clap::{Args, Parser, Subcommand, ValueEnum};
use jrsonnet_evaluator::{
    trace::{ExplainingFormat, PathResolver},
    FileImportResolver, State, Val,
};
use jrsonnet_stdlib::ContextInitializer;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RootArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Print(PrintArgs),
}

#[derive(Args)]
struct Input {
    config: PathBuf,

    #[arg(long, value_name = "JOB ID")]
    job_id: Option<String>,

    #[arg(long, default_value = "latest", value_name = "TAG")]
    imagetag: String,

    #[arg(long)]
    error_on_unknown_field: bool,

    #[arg(long)]
    unnested_job: bool,
}

#[derive(Args)]
struct PrintArgs {
    #[command(flatten)]
    input: Input,

    #[arg(long, value_enum, rename_all = "lower", default_value_t = Format::Json)]
    format: Format,
}

#[derive(Clone, ValueEnum)]
enum Format {
    Json,
    Yaml,
    Toml,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Jobspec {
    #[serde(rename(deserialize = "job", serialize = "Job"))]
    job: nomadapi::Job,
}

pub fn main() -> Result<(), Error> {
    let args = RootArgs::parse();

    match args.command {
        Command::Print(print_args) => match print(&print_args) {
            Err(Error::SerdeJrsonnet(error::Error::Evaluator(e)) | Error::Jrsonnet(e)) => {
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
        },
    }
}

fn print(args: &PrintArgs) -> Result<(), Error> {
    let state = State::default();
    state.set_import_resolver(FileImportResolver::default());

    let ctx = ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
    ctx.add_ext_str("imagetag".into(), args.input.imagetag.clone().into());
    state.set_context_initializer(ctx);

    // let mut tla = GcHashMap::<IStr, IStr>::new();
    // tla.insert("foo".into(), "foo-value".into());

    let val = state.import(&args.input.config)?;
    // let val = apply_tla(state.clone(), &tla, val)?;

    // TODO: only eval requested job
    let mut jobspecs = Vec::<Jobspec>::new();
    match val {
        Val::Arr(ref jobs) => {
            for job in jobs.iter() {
                let val: Val = job?;
                if args.input.unnested_job {
                    jobspecs.push(Jobspec {
                        job: deserializer::from_val(&val, args.input.error_on_unknown_field)?,
                    });
                } else {
                    jobspecs.push(deserializer::from_val(
                        &val,
                        args.input.error_on_unknown_field,
                    )?);
                }
            }
        }
        Val::Obj(_) => {
            if args.input.unnested_job {
                jobspecs.push(Jobspec {
                    job: deserializer::from_val(&val, args.input.error_on_unknown_field)?,
                });
            } else {
                jobspecs.push(deserializer::from_val(
                    &val,
                    args.input.error_on_unknown_field,
                )?);
            }
        }
        _ => {
            return Err(Error::ExpectedJobOrArrayOfJobs);
        }
    }

    let mut found = false;
    for spec in &jobspecs {
        if args.input.job_id.is_none()
            || args.input.job_id.as_ref().unwrap()
                == spec.job.id.as_ref().unwrap_or(&"*UNSET*".to_owned())
        {
            found = true;
            let output = match args.format {
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
