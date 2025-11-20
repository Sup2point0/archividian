use std::{
    fmt::Debug,
    time,
};

use crate::*;


#[derive(Debug, Clone)]
pub struct ArchivedFile
{
    name: String,
    path_rel: String,
    date_created: time::SystemTime,
}

impl ArchivedFile
{
    pub fn from(entry: walkdir::DirEntry, cli: &Cli) -> anyhow::Result<ArchivedFile>
    {
        let name =
            entry.file_name()
            .to_string_lossy().to_string();

        let path_rel =
            entry.path()
            .strip_prefix(cli.root_dir.parent().unwrap_or(&cli.root_dir))?
            .to_string_lossy().to_string();
        
        let date_created = entry.metadata()?.created()?;

        anyhow::Ok(
            Self { name, path_rel, date_created }
        )
    }

    pub fn export_oneline(&self, _cli: &Cli) -> String
    {
        format!(
            "{} -- {}",
            self.date_created.duration_since(time::SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs().to_string(),
            self.path_rel,
        )
    }
}
