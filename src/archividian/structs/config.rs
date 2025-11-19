use std::{
    env,
    path,
};

use chrono::prelude::*;


const EXPORT_EXTENSION: &str = "txt";

#[derive(Debug)]
pub struct Config
{
    pub root_dir: path::PathBuf,
    pub export_to: path::PathBuf,

    pub include_dotdirs: bool,
}

impl Config
{
    pub fn from_args(args: Vec<String>) -> anyhow::Result<Config>
    {
        let cwd = env::current_dir()?;

        let root_dir = {
            match args.get(1) {
                Some(route) => cwd.join(route),
                None        => cwd.clone()
            }
        };
        let export_to = {
            (match args.get(2) {
                Some(route) => cwd.join(route),
                None        => cwd.join(format!("archividian--{}", Utc::now().format("%Y-%m-%d--%H%M")))
            }
            ).with_extension(EXPORT_EXTENSION)
        };
        let include_dotdirs = args.contains(&"--include-dotdirs".to_string());

        Ok(
            Config { root_dir, export_to, include_dotdirs }
        )
    }
}
