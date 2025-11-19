use super::structs::*;


pub fn run(args: Vec<String>) -> anyhow::Result<()>
{
    let config = Config::from_args(args)?;

    println!("{:?}", config);

    let files: Vec<ArchivedFile> =
        find_files(&config)
        .map(ArchivedFile::from)
        .filter_map(anyhow::Result::ok)
        .collect()
    ;

    println!("{:?}", files);

    Ok(())

    // let archive = ArchiveData::of(files);
    // archive.export_to_file(&config)
}


fn find_files(config: &Config) -> impl Iterator<Item = walkdir::DirEntry>
{
    walkdir::WalkDir::new(&config.root)
        .into_iter()
        .filter_map(Result::ok)
}
