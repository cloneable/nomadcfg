use clap::Parser;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::PathBuf};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MyStruct {
    #[serde(rename = "myNumber")]
    pub my_int: i32,
    pub my_bool: bool,
    #[serde(default)]
    pub my_nullable_enum: Option<MyEnum>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum MyEnum {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(long)]
    print_config_schema: bool,

    #[arg(long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let opts = Opts::parse();

    if opts.print_config_schema {
        let schema = schema_for!(MyStruct);
        println!("{}", serde_json::to_string_pretty(&schema)?);
        return Ok(());
    }

    if let Some(config) = opts.config {
        let config: MyStruct = serde_yaml::from_reader(File::open(config)?)?;
        println!("{config:?}");
    }

    Ok(())
}
