use std::{
    env,
    path::{self, PathBuf},
};
use chrono::prelude::*;

use clap;


#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli
{
    #[arg(value_parser = set_root, default_value = get_root().into_os_string(),
        help = "Root directory from which to start finding files"
    )]
    pub root_dir: path::PathBuf,

    #[arg(short = 'o', long = "export",
        default_value = get_export().into_os_string(),
        help = "File path to export data to"
    )]
    pub export_to: path::PathBuf,

    #[arg(short = 'c', long = "config",
        help = "Path to `arv.json` config file"
    )]
    pub config_file: Option<path::PathBuf>,

    #[arg(long = "relative-to",
    default_value = get_rel().into_os_string(),
        help = "Base path which dirs are dispayed relative to"
    )]
    pub relative_to: path::PathBuf,

    #[arg(short = 'd', long = "dotdirs",
        help = "Include dirs starting with `.` (`.github/`, `.vscode/`, etc.)"
    )]
    pub include_dotdirs: bool,

    #[arg(long = "default-ignore",
        help = "Ignores a sensible default set of dirs (`.git/`, `node_modules/`, etc.)"
    )]
    pub default_ignore: bool,
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

fn get_rel() -> path::PathBuf
{
    let root = get_root();
    match root.parent() {
        None => root,
        Some(path) => PathBuf::from(path)
    }
}
