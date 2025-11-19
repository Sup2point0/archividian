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
    pub fn from(entry: walkdir::DirEntry, config: &Config) -> anyhow::Result<ArchivedFile>
    {
        let name =
            entry.file_name()
            .to_string_lossy().to_string();

        let path_rel =
            entry.path()
            .strip_prefix(config.root_dir.parent().unwrap_or(&config.root_dir))?
            .to_string_lossy().to_string();
        
        let date_created = entry.metadata()?.created()?;

        anyhow::Ok(
            Self { name, path_rel, date_created }
        )
    }

    pub fn export_oneline(&self, _config: &Config) -> String
    {
        format!(
            "{} -- {}",
            self.date_created.duration_since(time::SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs().to_string(),
            self.path_rel,
        )
    }
}
