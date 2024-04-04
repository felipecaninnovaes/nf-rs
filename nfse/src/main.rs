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
struct Valores {
    aliquota: String,
    desconto_incondicionado: String,
    desconto_condicionado: String,
    outras_retencoes: String,
    valor_cofins: String,
    valor_csll: String,
    valor_deducoes: String,
    valor_inss: String,
    valor_ir: String,
    valor_iss: String,
    valor_pis: String,
    valor_servicos: String,
}

#[derive(Deserialize, Debug)]
struct Nfse {
    dados_nfse: DadosNfse,
    prestador: Prestador,
    tomador: Tomador,
    valores: Valores,
}

#[derive(Deserialize, Debug)]
struct Config {
    layout: String,
    identifier: Identifier,
    dados_nfse: Vec<Data>,
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

impl std::fmt::Display for DadosNfse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "numero_da_nota: {}\ncompetencia: {}\ncodigo_do_municipio: {}\ncodigo_tributacao_mucipio: {}\nexigibilidade_iss: {}\niss_retido: {}\nitem_lista_servico: {}\nmunicipio_incidencia: {}\nresponsavel_retencao: {}",
            self.numero_da_nota, self.competencia, self.codigo_do_municipio, self.codigo_tributacao_mucipio, self.exigibilidade_iss, self.iss_retido, self.item_lista_servico, self.municipio_incidencia, self.responsavel_retencao
        )
    }
}

impl std::fmt::Display for Prestador {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "cnpj: {}\ninscricao_municipal: {}\nnome_fantasia: {}\nrazao_social: {}\nbairro: {}\ncep: {}\ncodigo_municipio: {}\ncodigo_pais: {}\ncomplemento: {}\nendereco: {}\nnumero: {}\nuf: {}\nemail: {}\ntelefone: {}",
            self.cnpj, self.inscricao_municipal, self.nome_fantasia, self.razao_social, self.bairro, self.cep, self.codigo_municipio, self.codigo_pais, self.complemento, self.endereco, self.numero, self.uf, self.email, self.telefone
        )
    }
}

impl std::fmt::Display for Tomador {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "cnpj: {}\ncpf: {}\nrazao_social: {}\nbairro: {}\ncep: {}\ncodigo_municipio: {}\nendereco: {}\nnumero: {}\nuf: {}\nemail: {}\ntelefone: {}",
            self.cnpj, self.cpf, self.razao_social, self.bairro, self.cep, self.codigo_municipio, self.endereco, self.numero, self.uf, self.email, self.telefone
        )
    }
}

impl std::fmt::Display for Valores {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "aliquota: {}\ndesconto_incondicionado: {}\ndesconto_condicionado: {}\noutras_retencoes: {}\nvalor_cofins: {}\nvalor_csll: {}\nvalor_deducoes: {}\nvalor_inss: {}\nvalor_ir: {}\nvalor_iss: {}\nvalor_pis: {}\nvalor_servicos: {}",
            self.aliquota, self.desconto_incondicionado, self.desconto_condicionado, self.outras_retencoes, self.valor_cofins, self.valor_csll, self.valor_deducoes, self.valor_inss, self.valor_ir, self.valor_iss, self.valor_pis, self.valor_servicos
        )
    }
}

impl std::fmt::Display for Nfse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\ndados_nfse: {}\n\nprestador: {}\n\ntomador: {}\n\nvalores: {}",
            self.dados_nfse, self.prestador, self.tomador, self.valores
        )
    }
}

fn get_config(path: &str) -> Config {
    let config = std::fs::read_to_string(path).unwrap();
    let config: Config = serde_json::from_str(&config).unwrap();
    config
}

fn read_dir(path: &str, extension: &str) -> Vec<String> {
    let folder = std::fs::read_dir(path).unwrap();
    let mut folder_files = Vec::new();
    for file in folder {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        match file_name.ends_with(extension) {
            true => folder_files.push(file_name),
            false => continue,
        }
    }
    folder_files
}

fn check_layout(nfse_json: &serde_json::Value, layout_folder: &str) -> Result<Config, ()> {
    let layout_files = read_dir(layout_folder, ".json");
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

fn get_field(data: &Data, nfse_json: &serde_json::Value) -> Option<String> {
    let binding = data.fieldtype.replace(' ', "");
    let data_path = binding.split(',').collect::<Vec<&str>>();
    let mut field_value = nfse_json;
    for data_field in data_path {
        let field = &field_value[data_field];
        field_value = field;
    }
    match field_value.is_null() {
        true => None,
        false => Some(field_value[&data.field].to_string()),
    }
}

fn parse_in_nfse(nfse_json: &serde_json::Value, config: Config) {
    let dados_nfse: DadosNfse = {
        DadosNfse {
            numero_da_nota: get_field(&config.dados_nfse[0], nfse_json).unwrap(),
            competencia: get_field(&config.dados_nfse[1], nfse_json).unwrap(),
            codigo_do_municipio: get_field(&config.dados_nfse[2], nfse_json).unwrap(),
            codigo_tributacao_mucipio: get_field(&config.dados_nfse[3], nfse_json).unwrap(),
            exigibilidade_iss: get_field(&config.dados_nfse[4], nfse_json).unwrap(),
            iss_retido: get_field(&config.dados_nfse[5], nfse_json).unwrap(),
            item_lista_servico: get_field(&config.dados_nfse[6], nfse_json).unwrap(),
            municipio_incidencia: get_field(&config.dados_nfse[7], nfse_json).unwrap(),
            responsavel_retencao: get_field(&config.dados_nfse[8], nfse_json).unwrap(),
        }
    };

    let prestador: Prestador = {
        Prestador {
            cnpj: get_field(&config.prestador[0], nfse_json).unwrap(),
            inscricao_municipal: get_field(&config.prestador[1], nfse_json).unwrap(),
            nome_fantasia: get_field(&config.prestador[2], nfse_json).unwrap(),
            razao_social: get_field(&config.prestador[3], nfse_json).unwrap(),
            bairro: get_field(&config.prestador[4], nfse_json).unwrap(),
            cep: get_field(&config.prestador[5], nfse_json).unwrap(),
            codigo_municipio: get_field(&config.prestador[6], nfse_json).unwrap(),
            codigo_pais: get_field(&config.prestador[7], nfse_json).unwrap(),
            complemento: get_field(&config.prestador[8], nfse_json).unwrap(),
            endereco: get_field(&config.prestador[9], nfse_json).unwrap(),
            numero: get_field(&config.prestador[10], nfse_json).unwrap(),
            uf: get_field(&config.prestador[11], nfse_json).unwrap(),
            email: get_field(&config.prestador[12], nfse_json).unwrap(),
            telefone: get_field(&config.prestador[13], nfse_json).unwrap(),
        }
    };

    let tomador: Tomador = {
        Tomador {
            cnpj: get_field(&config.tomador[0], nfse_json).unwrap(),
            cpf: get_field(&config.tomador[1], nfse_json).unwrap(),
            razao_social: get_field(&config.tomador[2], nfse_json).unwrap(),
            bairro: get_field(&config.tomador[3], nfse_json).unwrap(),
            cep: get_field(&config.tomador[4], nfse_json).unwrap(),
            codigo_municipio: get_field(&config.tomador[5], nfse_json).unwrap(),
            endereco: get_field(&config.tomador[6], nfse_json).unwrap(),
            numero: get_field(&config.tomador[7], nfse_json).unwrap(),
            uf: get_field(&config.tomador[8], nfse_json).unwrap(),
            email: get_field(&config.tomador[9], nfse_json).unwrap(),
            telefone: get_field(&config.tomador[10], nfse_json).unwrap(),
        }
    };

    let valores: Valores = {
        Valores {
            aliquota: get_field(&config.valores[0], nfse_json).unwrap(),
            desconto_incondicionado: get_field(&config.valores[1], nfse_json).unwrap(),
            desconto_condicionado: get_field(&config.valores[2], nfse_json).unwrap(),
            outras_retencoes: get_field(&config.valores[3], nfse_json).unwrap(),
            valor_cofins: get_field(&config.valores[4], nfse_json).unwrap(),
            valor_csll: get_field(&config.valores[5], nfse_json).unwrap(),
            valor_deducoes: get_field(&config.valores[6], nfse_json).unwrap(),
            valor_inss: get_field(&config.valores[7], nfse_json).unwrap(),
            valor_ir: get_field(&config.valores[8], nfse_json).unwrap(),
            valor_iss: get_field(&config.valores[9], nfse_json).unwrap(),
            valor_pis: get_field(&config.valores[10], nfse_json).unwrap(),
            valor_servicos: get_field(&config.valores[11], nfse_json).unwrap(),
        }
    };

    let nfse = Nfse {
        dados_nfse,
        prestador,
        tomador,
        valores,
    };

    println!("{}", nfse);
}

pub fn get_nfse(nfse_layout_folder_path: &str, nfse_json_path: &str) {
    let nfse_json = to_json_from_file(nfse_json_path).unwrap();
    let config = check_layout(&nfse_json, nfse_layout_folder_path);
    match config {
        Ok(config) => parse_in_nfse(&nfse_json, config),
        Err(_) => println!("layout not found"),
    };
}

fn main() {
    let nfse_layout_folder_path = "/home/felipecn/Desktop/PROJECTS/nf-rs/nfse/src/layouts";
    let nfse_json_path = "/home/felipecn/Desktop/PROJECTS/nf-rs/nfse/src/models/model_nfse_catanduva_01_normal_com_rps.xml";
    get_nfse(nfse_layout_folder_path, nfse_json_path);
}
