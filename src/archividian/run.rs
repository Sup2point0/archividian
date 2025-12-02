use crate::*;

use clap::Parser;


pub fn run() -> anyhow::Result<()>
{
    let mut cli = Cli::parse();
    cli.init()?;

    let found = walk::find_files(&cli);
    
    let files: Vec<ArchivedFile> =
        found
        .map(|f| ArchivedFile::from(f, &cli))
        .filter_map(anyhow::Result::ok)
        .collect()
    ;

    let archive = ArchiveData::of(files.into_iter());
    archive.export_to_file(&cli)?;

    Ok(())
}
