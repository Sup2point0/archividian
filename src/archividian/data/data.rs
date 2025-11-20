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

    pub fn export_to_text(&self, cli: &Cli) -> String
    {
        let file_data = self.files
            .clone()
            .into_iter()
            .map(|file| file.export_oneline(&cli))
            .collect::<Vec<String>>();

        format!(
"# Archividian Export

Exported on **{}**
**{}** files archived

| Created    | Modified   | Path |
| :--------- | :--------- | :--- |
",
            chrono::Utc::now().format("%Y %B %d at %H:%M.%S UTC"),
            file_data.len(),
        )
            .to_string() + &file_data.join("\n")
    }

    pub fn export_to_file(&self, cli: &Cli) -> anyhow::Result<()>
    {
        let load = self.export_to_text(&cli);
        let out = fs::write(&cli.export_to, load.as_bytes());
        Ok(out?)
    }
}
