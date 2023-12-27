use std::fs;
use std::io;

pub fn listar_arquivos(diretorio: &str) -> io::Result<Vec<String>> {
    let mut arquivos = Vec::new();

    for entrada in fs::read_dir(diretorio)? {
        let entrada = entrada?;
        if entrada.metadata()?.is_file() {
            arquivos.push(entrada.path().display().to_string());
        }
    }

    Ok(arquivos)
}