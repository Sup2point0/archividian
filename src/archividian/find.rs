use crate::*;


pub fn find_files(config: &Config) -> impl Iterator<Item = walkdir::DirEntry>
{
    walkdir::WalkDir::new(&config.root)
        .into_iter()
        .filter_entry(|e| config.include_dotdirs || !is_dotdir(e))
        .filter_map(Result::ok)
}


fn is_dotdir(entry: &walkdir::DirEntry) -> bool
{
    entry
        .file_name()
        .to_str()
        .map(|e| e.starts_with("."))
        // .map(|e| entry.depth() == 0 || e.starts_with("."))
        .unwrap_or(false)
}
