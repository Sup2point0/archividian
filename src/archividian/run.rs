use crate::*;


pub fn run(args: Vec<String>) -> anyhow::Result<()>
{
    let config = Config::from_args(args)?;

    let found = archividian::find_files(&config);
    
    let files: Vec<ArchivedFile> =
        found
        .map(ArchivedFile::from)
        .filter_map(anyhow::Result::ok)
        .collect()
    ;

    let archive = ArchiveData::of(files.into_iter());
    println!("{}", archive.export_to_text(&config));
    // archive.export_to_file(&config)

    Ok(())
}
