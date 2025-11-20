use std::{fmt::Debug};
use chrono::prelude::*;

use crate::*;


#[derive(Debug, Clone)]
pub struct ArchivedFile
{
    name: String,
    path_rel: String,
    date_created: chrono::DateTime<Utc>,
    date_edited: chrono::DateTime<Utc>,
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
            .strip_prefix(&cli.relative_to)?
            .to_string_lossy().to_string();
        
        let date_created = entry.metadata()?.created()?.into();
        let date_edited = entry.metadata()?.modified()?.into();

        anyhow::Ok(
            Self { name, path_rel, date_created, date_edited }
        )
    }

    pub fn export_oneline(&self, _cli: &Cli) -> String
    {
        format!(
            "| {} | {} | `{}` |",
            self.date_created.format("%Y/%m/%d"),
            self.date_edited.format("%Y/%m/%d"),
            self.path_rel,
        )
    }
}
