use std::{
    env,
    fs,
    path::{self, PathBuf},
};
use chrono::prelude::*;

use clap;
use serde_json as json;


#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli
{
    #[arg(value_parser = Self::set_root,
        default_value = Self::get_root().into_os_string(),
        help = "Root directory from which to start finding files"
    )]
    pub root_dir: path::PathBuf,

    #[arg(short = 'o', long = "export",
        default_value = Self::get_export().into_os_string(),
        help = "File path to export data to"
    )]
    pub export_to: path::PathBuf,

    #[arg(short = 'c', long = "config",
        help = "Path to `arv.json` config file"
    )]
    pub config_file: Option<path::PathBuf>,

    #[arg(long = "relative-to",
        default_value = Self::get_rel().into_os_string(),
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

impl Cli
{
    pub fn config(&mut self) -> anyhow::Result<()>
    {
        if let Some(path) = &self.config_file {
            let data = Self::get_config(path.clone())?;
            self.set_config(data);
        }

        Ok(())
    }

    pub fn get_config(path: path::PathBuf) -> anyhow::Result<json::Value>
    {
        let text = fs::read_to_string(path)?;
        let data = json::from_str(&text)?;
        Ok(data)
    }

    pub fn set_config(&mut self, data: json::Value) -> ()
    {
        if let json::Value::String(path) = data["root-dir"].clone() {
            self.root_dir = if path.starts_with("C:") {
                PathBuf::from(path)
            } else {
                self.root_dir.join(path)
            }
        }

        if let json::Value::String(path) = data["--export"].clone() {
            self.export_to = if path.starts_with("C:") {
                PathBuf::from(path)
            } else {
                self.root_dir.join(path)
            }
        }

        if let json::Value::String(path) = data["--relative-to"].clone() {
            self.relative_to = if path.starts_with("C:") {
                PathBuf::from(path)
            } else {
                self.root_dir.join(path)
            }
        }

        if let json::Value::Bool(state) = data["--dotdirs"] {
            self.include_dotdirs = state;
        }

        if let json::Value::Bool(state) = data["--default-ignore"] {
            self.default_ignore = state;
        }
    }
}

impl Cli
{
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
        Self::get_root().join(timestamp)
    }

    fn get_rel() -> path::PathBuf
    {
        let root = Self::get_root();
        match root.parent() {
            None => root,
            Some(path) => PathBuf::from(path)
        }
    }
}
