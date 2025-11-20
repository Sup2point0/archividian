use std::{
    env,
    path,
};
use chrono::prelude::*;

use clap;


#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli
{
    #[arg(value_parser = set_root, default_value = get_root().into_os_string())]
    pub root_dir: path::PathBuf,

    #[arg(short = 'o', long = "export", default_value = get_export().into_os_string())]
    pub export_to: path::PathBuf,

    #[arg(short = 'c', long = "config")]
    pub config_file: Option<path::PathBuf>,

    #[arg(short = 'd', long = "dotdirs")]
    pub include_dotdirs: bool,
}

fn get_root() -> path::PathBuf
{
    env::current_dir().unwrap()
}

fn set_root(s: &str) -> anyhow::Result<path::PathBuf>
{
    Ok(env::current_dir()?.join(s))
}

fn get_export() -> path::PathBuf
{
    let timestamp = format!("archividian--{}.md", Utc::now().format("%Y-%m-%d--%H%M"));
    get_root().join(timestamp)
}
