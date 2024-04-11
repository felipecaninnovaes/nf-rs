#![allow(dead_code, unused_imports)]
#[derive(Debug)]
pub struct Dir {
    pub dir_files: Option<Vec<String>>,
    pub files_extensions: Option<String>,
}

type Path = str;
type Extension = str;

impl Dir {
    pub fn read_dir(path: &Path, extension: &Extension) -> Dir {
        // TODO: Remover os espaços em branco.
        let folder = std::fs::read_dir(path).expect("Error reading folder");
        let mut folder_files = Vec::new();
        for file in folder {
            let file = file.unwrap();
            let file_name = file
                .file_name()
                .into_string()
                .expect("Error reading file name");
            match file_name.ends_with(extension) {
                true => folder_files.push(format!("{}/{}", path, file_name)),
                false => continue,
            }
        }
        folder_files.sort();
        match folder_files.is_empty() {
            true => Dir {
                dir_files: None,
                files_extensions: None,
            },
            false => Dir {
                dir_files: Some(folder_files),
                files_extensions: Some(extension.to_string()),
            },
        }
    }
}
