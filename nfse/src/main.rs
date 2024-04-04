#![allow(unused_imports, dead_code)]
use serde::Deserialize;
use xml_json::to_json_from_file;

#[derive(Deserialize, Debug)]
struct DadosNfse {
    numero_da_nota: String,
    competencia: String,
    codigo_do_municipio: String,
    codigo_tributacao_mucipio: String,
    exigibilidade_iss: String,
    iss_retido: String,
    item_lista_servico: String,
    municipio_incidencia: String,
    responsavel_retencao: String,
}

#[derive(Deserialize, Debug)]
struct Prestador {
    cnpj: String,
    inscricao_municipal: String,
    nome_fantasia: String,
    razao_social: String,
    bairro: String,
    cep: String,
    codigo_municipio: String,
    codigo_pais: String,
    complemento: String,
    endereco: String,
    numero: String,
    uf: String,
    email: String,
    telefone: String,
}

#[derive(Deserialize, Debug)]
struct Tomador {
    cnpj: String,
    cpf: String,
    razao_social: String,
    bairro: String,
    cep: String,
    codigo_municipio: String,
    endereco: String,
    numero: String,
    uf: String,
    email: String,
    telefone: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    layout: String,
    identifier: Identifier,
    prestador: Vec<Data>,
    tomador: Vec<Data>,
    valores: Vec<Data>,
}

#[derive(Deserialize, Debug)]
struct Identifier {
    key: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    field: String,
    fieldtype: String,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "layout: {}\nidentifier: {}\ndata: {:?}",
            self.layout, self.identifier.key, self.prestador
        )
    }
}

fn get_config(path: &str) -> Config {
    let config = std::fs::read_to_string(path).unwrap();
    let config: Config = serde_json::from_str(&config).unwrap();
    config
}

fn read_dir(path: &str) -> Vec<String> {
    let folder = std::fs::read_dir(path).unwrap();
    let mut layout_files = Vec::new();
    // filder only .json files
    for file in folder {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        if file_name.ends_with(".json") {
            layout_files.push(file_name);
        }
    }
    layout_files
}

async fn check_layout(nfse_json: &serde_json::Value, layout_folder: &str) -> Result<Config, ()> {
    let layout_files = read_dir(layout_folder);
    for layout in layout_files {
        let layout_path = format!("{}/{}", layout_folder, layout);
        let config = get_config(&layout_path);
        let identifier_value = &nfse_json[&config.identifier.key][&config.identifier.value];
        match identifier_value.is_null() {
            true => continue,
            false => {
                println!("layout found: {}", config.layout);
                return Ok(config);
            }
        };
    }
    Err(())
}

fn get_filed(data: &Data, nfse_json: serde_json::Value) -> Option<String> {
    let binding = data.fieldtype.replace(' ', "");
    let data_path = binding.split(',').collect::<Vec<&str>>();
    let mut field_value = nfse_json;
    for data_field in data_path {
        let field = &field_value[data_field];
        field_value = field.clone();
    }
    match field_value.is_null() {
        true => None,
        false => Some(field_value[&data.field].to_string()),
    }
}
#[tokio::main]
async fn main() {
    let nfse_json = to_json_from_file(
        "/home/felipecn/Desktop/PROJECTS/nf-rs/nfse/src/models/model_nfse_catanduva_01_normal_com_rps.xml",
    )
    .unwrap();
    let config = check_layout(
        &nfse_json,
        "/home/felipecn/Desktop/PROJECTS/nf-rs/nfse/src/layouts",
    )
    .await;
    // println!("{:?}", config);
    let result = get_filed(&config.unwrap().valores[0], nfse_json).unwrap();
    println!("{}", result);
}
