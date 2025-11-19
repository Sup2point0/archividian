use std::{
    env,
    path,
};


const EXPORT_EXTENSION: &str = "txt";

#[derive(Debug)]
pub struct Config
{
    pub root: path::PathBuf,
    pub export_to: path::PathBuf,
}

impl Config
{
    pub fn from_args(args: Vec<String>) -> anyhow::Result<Config>
    {
        let cwd = env::current_dir()?;

        Ok(Config {
            root: {
                match args.get(0) {
                    Some(route) => cwd.join(route),
                    None        => cwd.clone()
                }
            },
            export_to: {
                (match args.get(1) {
                    Some(route) => cwd.join(route),
                    None        => cwd.join("export")
                }
                ).join(EXPORT_EXTENSION)
            }
        })
    }
}
