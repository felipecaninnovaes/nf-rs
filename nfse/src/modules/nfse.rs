#![allow(unused_imports, dead_code)]
use serde::Deserialize;
use xml_json::to_json_from_file;

use super::structs::{
    Config, DadosNfse, Data, Endereco, Identifier, Nfse, Prestador, Tomador, Valores,
};

fn get_config(path: &str) -> Config {
    let config = std::fs::read_to_string(path).unwrap();
    let config: Config = serde_json::from_str(&config).unwrap();
    config
}

fn check_layout(nfse_json: &serde_json::Value, layout_folder: &str) -> Result<Config, ()> {
    let layout_files = utils::core::fs::Dir::read_dir(layout_folder, ".json");
    for layout in layout_files.dir_files.expect("error reading layout files") {
        let config = get_config(&layout);
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
    let binding = data.fieldpath.replace(' ', "");
    let data_path = binding.split('/').collect::<Vec<&str>>();
    let mut field_value = nfse_json;
    for data_field in data_path {
        let field = &field_value[data_field];
        field_value = field;
    }
    match field_value.is_null() {
        true => Some("Null".to_string()),
        false => Some(field_value[&data.field].to_string().replace('\"', "")),
    }
}

fn parse_in_nfse(nfse_json: &serde_json::Value, config: Config) -> Nfse {
    println!("{:?}", nfse_json);
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
            cnpj: get_field(&config.prestador_json.dados_prestador.cnpj, nfse_json).unwrap(),
            inscricao_municipal: get_field(
                &config.prestador_json.dados_prestador.inscricao_municipal,
                nfse_json,
            )
            .unwrap(),
            nome_fantasia: get_field(
                &config.prestador_json.dados_prestador.nome_fantasia,
                nfse_json,
            )
            .unwrap(),
            razao_social: get_field(
                &config.prestador_json.dados_prestador.razao_social,
                nfse_json,
            )
            .unwrap(),
            endereco: Endereco {
                bairro: get_field(&config.prestador_json.endereco.bairro, nfse_json).unwrap(),
                cep: get_field(&config.prestador_json.endereco.cep, nfse_json).unwrap(),
                codigo_municipio: get_field(
                    &config.prestador_json.endereco.codigo_municipio,
                    nfse_json,
                )
                .unwrap(),
                codigo_pais: get_field(&config.prestador_json.endereco.codigo_pais, nfse_json)
                    .unwrap(),
                complemento: get_field(&config.prestador_json.endereco.complemento, nfse_json)
                    .unwrap(),
                logradouro: get_field(&config.prestador_json.endereco.logradouro, nfse_json)
                    .unwrap(),
                numero: get_field(&config.prestador_json.endereco.numero, nfse_json).unwrap(),
                uf: get_field(&config.prestador_json.endereco.uf, nfse_json).unwrap(),
            },
            email: get_field(&config.prestador_json.contato.email, nfse_json).unwrap(),
            telefone: get_field(&config.prestador_json.contato.telefone, nfse_json).unwrap(),
        }
    };

    let tomador: Tomador = {
        Tomador {
            cnpj: get_field(&config.tomador_json.dados_tomador.cnpj, nfse_json).unwrap(),
            cpf: get_field(&config.tomador_json.dados_tomador.cpf, nfse_json).unwrap(),
            inscricao_municipal: get_field(
                &config.tomador_json.dados_tomador.inscricao_municipal,
                nfse_json,
            )
            .unwrap(),
            razao_social: get_field(&config.tomador_json.dados_tomador.razao_social, nfse_json)
                .unwrap(),
            endereco: Endereco {
                bairro: get_field(&config.tomador_json.endereco.bairro, nfse_json).unwrap(),
                cep: get_field(&config.tomador_json.endereco.cep, nfse_json).unwrap(),
                codigo_municipio: get_field(
                    &config.tomador_json.endereco.codigo_municipio,
                    nfse_json,
                )
                .unwrap(),
                codigo_pais: get_field(&config.tomador_json.endereco.codigo_pais, nfse_json)
                    .unwrap(),
                complemento: get_field(&config.tomador_json.endereco.complemento, nfse_json)
                    .unwrap(),
                logradouro: get_field(&config.tomador_json.endereco.logradouro, nfse_json).unwrap(),
                numero: get_field(&config.tomador_json.endereco.numero, nfse_json).unwrap(),
                uf: get_field(&config.tomador_json.endereco.uf, nfse_json).unwrap(),
            },
            email: get_field(&config.tomador_json.contato.email, nfse_json).unwrap(),
            telefone: get_field(&config.tomador_json.contato.telefone, nfse_json).unwrap(),
        }
    };

    let valores: Valores = {
        Valores {
            aliquota: get_field(&config.valores[0], nfse_json).unwrap(),
            base_calculo: get_field(&config.valores[1], nfse_json).unwrap(),
            desconto_incondicionado: get_field(&config.valores[2], nfse_json).unwrap(),
            desconto_condicionado: get_field(&config.valores[3], nfse_json).unwrap(),
            outras_retencoes: get_field(&config.valores[4], nfse_json).unwrap(),
            valor_cofins: get_field(&config.valores[5], nfse_json).unwrap(),
            valor_csll: get_field(&config.valores[6], nfse_json).unwrap(),
            valor_deducoes: get_field(&config.valores[7], nfse_json).unwrap(),
            valor_inss: get_field(&config.valores[8], nfse_json).unwrap(),
            valor_ir: get_field(&config.valores[9], nfse_json).unwrap(),
            valor_iss: get_field(&config.valores[10], nfse_json).unwrap(),
            valor_pis: get_field(&config.valores[11], nfse_json).unwrap(),
            valor_servicos: get_field(&config.valores[12], nfse_json).unwrap(),
        }
    };

    Nfse {
        dados_nfse,
        prestador,
        tomador,
        valores,
    }
}

pub fn get_nfse(nfse_layout_folder_path: &str, nfse_json_path: &str) -> Result<Nfse, String> {
    let nfse_json = to_json_from_file(nfse_json_path).unwrap();
    let config = check_layout(&nfse_json, nfse_layout_folder_path);
    match config {
        Ok(config) => Ok(parse_in_nfse(&nfse_json, config)),
        Err(_) => Err("layout not found".to_string()),
    }
}