use std::fs;
use std::io;

pub fn list_folder(folder: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    for input in fs::read_dir(folder)? {
        let input = input?;
        if input.metadata()?.is_file() {
            let path = input.path();
            if path.extension() == Some(std::ffi::OsStr::new("xml")) {
                files.push(path.display().to_string());
            }
        }
    }

    Ok(files)
}

pub fn remove_file(caminho: &str) -> std::io::Result<()> {
    fs::remove_file(caminho)
}