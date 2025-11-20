use crate::*;


pub fn find_files(cli: &Cli) -> impl Iterator<Item = walkdir::DirEntry>
{
    walkdir::WalkDir::new(&cli.root_dir)
        .into_iter()
        .filter_entry(|e| check_dir(e, cli))
        .filter_map(Result::ok)
}


fn check_dir(entry: &walkdir::DirEntry, cli: &Cli) -> bool
{
    if let Some(name) = entry.file_name().to_str() {
        !is_autogen(name)
        && (cli.include_dotdirs || !is_dotdir(name))
    }
    else {
        false
    }
}

fn is_autogen(name: &str) -> bool
{
    [
        ".git",
        "__pycache__", "node_modules", "target", "dist-newstyle"
    ].contains(&name)
}

fn is_dotdir(name: &str) -> bool
{
    name.starts_with(".")
}
