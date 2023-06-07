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
    clippy::map_err_ignore,
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
use std::{path::PathBuf, process, time::Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RootArgs {
  #[command(subcommand)]
  command: Command,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
enum Command {
  Print(PrintArgs),
  Diff(DiffArgs),
}

#[derive(Args, Debug)]
struct Input {
  #[arg(long, value_name = "JOB_ID")]
  job_id: Option<String>,

  #[arg(long, value_name = "TAG")]
  imagetag: Option<String>,

  #[arg(long)]
  error_on_unknown_field: bool,

  #[arg(long)]
  unnested_job: bool,

  config: PathBuf,
}

#[derive(Args, Debug)]
struct PrintArgs {
  #[arg(long, value_enum, rename_all = "lower", default_value_t = Format::Json)]
  format: Format,

  #[command(flatten)]
  input: Input,
}

#[derive(Clone, ValueEnum, Debug)]
enum Format {
  Json,
  Yaml,
  Toml,
}

#[derive(Args, Debug)]
struct DiffArgs {
  #[command(flatten)]
  nomad: NomadArgs,

  #[command(flatten)]
  input: Input,
}

#[derive(Args, Debug)]
struct NomadArgs {
  #[arg(
    long,
    env = "NOMAD_ADDR",
    default_value = "http://127.0.0.1:4646",
    value_name = "URL"
  )]
  address: Option<String>,

  #[arg(long, env = "NOMAD_CACERT", value_name = "PATH")]
  ca_cert: Option<PathBuf>,

  #[arg(long, env = "NOMAD_CAPATH", value_name = "PATH")]
  ca_path: Option<PathBuf>,

  #[arg(long, env = "NOMAD_CLIENT_CERT", value_name = "PATH")]
  client_cert: Option<PathBuf>,

  #[arg(long, env = "NOMAD_CLIENT_KEY", value_name = "PATH")]
  client_key: Option<PathBuf>,

  #[arg(long, env = "NOMAD_TLS_SERVER_NAME", value_name = "SERVER_NAME")]
  tls_server_name: Option<String>,

  #[arg(long, env = "NOMAD_SKIP_VERIFY")]
  tls_skip_verify: bool,

  #[arg(long, env = "NOMAD_REGION", value_name = "REGION")]
  region: Option<String>,

  #[arg(
    long,
    env = "NOMAD_NAMESPACE",
    default_value = "default",
    value_name = "NAMESPACE"
  )]
  namespace: Option<String>,

  #[arg(long, env = "NOMAD_TOKEN", value_name = "TOKEN")]
  token: Option<String>,

  #[arg(long, env = "VAULT_TOKEN", value_name = "TOKEN")]
  vault_token: Option<String>,

  #[arg(long, value_name = "NAMESPACE")]
  vault_namespace: Option<String>,

  #[arg(long, env = "CONSUL_HTTP_TOKEN", value_name = "TOKEN")]
  consul_token: Option<String>,
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
      Err(
        Error::SerdeJrsonnet(error::Error::Evaluator(e)) | Error::Jrsonnet(e),
      ) => {
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
      Err(e) => {
        eprintln!("Error: {e}");
        process::exit(1);
      }
      r => r,
    },
    Command::Diff(diff_args) => diff(&diff_args),
  }
}

fn evaluate(input: &Input) -> Result<Vec<Jobspec>, Error> {
  let state = State::default();
  state.set_import_resolver(FileImportResolver::default());

  let ctx =
    ContextInitializer::new(state.clone(), PathResolver::new_cwd_fallback());
  if let Some(ref imagetag) = input.imagetag {
    ctx.add_ext_str("imagetag".into(), imagetag.into());
  }
  state.set_context_initializer(ctx);

  // let mut tla = GcHashMap::<IStr, IStr>::new();
  // tla.insert("foo".into(), "foo-value".into());

  let val = state.import(&input.config)?;
  // let val = apply_tla(state.clone(), &tla, val)?;

  // TODO: canonicalize
  // TODO: only eval requested job
  let mut jobspecs = Vec::<Jobspec>::new();
  match val {
    Val::Arr(ref jobs) => {
      for job in jobs.iter() {
        let val: Val = job?;
        if input.unnested_job {
          jobspecs.push(Jobspec {
            job: deserializer::from_val(&val, input.error_on_unknown_field)?,
          });
        } else {
          jobspecs
            .push(deserializer::from_val(&val, input.error_on_unknown_field)?);
        }
      }
    }
    Val::Obj(_) => {
      if input.unnested_job {
        jobspecs.push(Jobspec {
          job: deserializer::from_val(&val, input.error_on_unknown_field)?,
        });
      } else {
        jobspecs
          .push(deserializer::from_val(&val, input.error_on_unknown_field)?);
      }
    }
    _ => {
      return Err(Error::ExpectedJobOrArrayOfJobs);
    }
  }

  Ok(jobspecs)
}

fn print(args: &PrintArgs) -> Result<(), Error> {
  let jobspecs = evaluate(&args.input)?;

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

fn diff(args: &DiffArgs) -> Result<(), Error> {
  let jobspecs = evaluate(&args.input)?;

  let local_spec = jobspecs
    .iter()
    .find(|j| {
      args.input.job_id.is_none()
        || args.input.job_id.as_ref().unwrap()
          == j.job.id.as_ref().unwrap_or(&"*UNSET*".to_owned())
    })
    .ok_or(Error::NoJobsFound)?;
  let local_json = serde_json::to_value(&local_spec.job)?;
  let local_yaml = serde_yaml::to_string(&local_json)?;

  let remote_json = get_job(
    args.nomad.address.as_ref().unwrap(),
    local_spec.job.id.as_ref().unwrap(),
    local_spec.job.namespace.as_deref(),
  )?;
  let remote_yaml = serde_yaml::to_string(&remote_json)?;

  for chunk in diff::lines(&remote_yaml, &local_yaml) {
    match chunk {
      diff::Result::Left(l) => println!("-{}", l),
      diff::Result::Both(l, _) => println!(" {}", l),
      diff::Result::Right(r) => println!("+{}", r),
    }
  }

  Ok(())
}

fn get_job(
  base_url: &str,
  job_id: &str,
  namespace: Option<&str>,
) -> reqwest::Result<serde_json::Value> {
  let mut u = url::Url::parse(base_url).unwrap();
  u.path_segments_mut().unwrap().clear().extend(["v1", "job", job_id]);
  if let Some(ns) = namespace {
    u.set_query(Some(&format!("namespace={ns}")));
  }

  // TODO: TLS

  let client = reqwest::blocking::Client::builder()
    .connect_timeout(Duration::from_secs(10))
    .build()?;
  let body = client.get(u).send()?.json::<serde_json::Value>()?;
  Ok(body)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("no job(s) found")]
  NoJobsFound,

  #[error("expected job or array of jobs")]
  ExpectedJobOrArrayOfJobs,

  #[error("jrsonnet error: {0}")]
  Jrsonnet(#[from] jrsonnet_evaluator::Error),

  #[error("deserializer error: {0}")]
  SerdeJrsonnet(#[from] error::Error),

  #[error("serde_json error: {0}")]
  SerdeJson(#[from] serde_json::Error),

  #[error("serde_yaml error: {0}")]
  SerdeYaml(#[from] serde_yaml::Error),

  #[error("toml error: {0}")]
  Toml(#[from] toml::ser::Error),

  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
}
