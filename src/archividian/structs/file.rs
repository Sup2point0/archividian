use std::{fmt::Debug, time};

use crate::*;


#[derive(Debug, Clone)]
pub struct ArchivedFile
{
    name: String,
    date_created: time::SystemTime,
}

impl ArchivedFile
{
    pub fn from(entry: walkdir::DirEntry) -> anyhow::Result<ArchivedFile>
    {
        let name = entry.file_name().to_string_lossy().to_string();
        let date_created = entry.metadata()?.created()?;

        anyhow::Ok(Self { name, date_created })
    }

    pub fn export_oneline(&self, _config: &Config) -> String
    {
        unimplemented!()
    }
}
