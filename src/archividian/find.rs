use crate::*;


pub fn find_files(config: &Config) -> impl Iterator<Item = walkdir::DirEntry>
{
    walkdir::WalkDir::new(&config.root_dir)
        .into_iter()
        .filter_entry(|e| check_dir(e, config))
        .filter_map(Result::ok)
}


fn check_dir(entry: &walkdir::DirEntry, config: &Config) -> bool
{
    if let Some(name) = entry.file_name().to_str() {
        !is_autogen(name)
        && (config.include_dotdirs || !is_dotdir(name))
    }
    else {
        false
    }
}

fn is_autogen(name: &str) -> bool
{
    [
        ".git",
        "__pycache__", "node_modules", "target", "dist_newstyle"
    ].contains(&name)
}

fn is_dotdir(name: &str) -> bool
{
    name.starts_with(".")
}
