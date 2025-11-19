use std::fs;

use crate::*;


pub struct ArchiveData
{
    files: Vec<ArchivedFile>,
}

impl ArchiveData
{
    pub fn of(files: impl Iterator<Item = ArchivedFile>) -> ArchiveData
    {
        ArchiveData {
            files: files.collect()
        }
    }

    pub fn export_to_text(&self, config: &Config) -> String
    {
        self.files
            .clone()
            .into_iter()
            .map(|file| file.export_oneline(&config))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn export_to_file(&self, config: &Config) -> anyhow::Result<()>
    {
        let load = self.export_to_text(&config);
        let out = fs::write(&config.export_to, load.as_bytes());
        Ok(out?)
    }
}
