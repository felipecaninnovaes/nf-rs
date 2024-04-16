pub mod modules;

#[cfg(test)]
mod tests {
    use crate::modules::{self, structs::*};

    #[test]
    fn get_nfse() {
        let result: Nfse = Nfse {
            dados_nfse: DadosNfse {
                numero_da_nota: "6983".to_string(),
                competencia: "2024-04-03".to_string(),
                codigo_do_municipio: "3511102".to_string(),
                codigo_tributacao_mucipio: "0000140000001".to_string(),
                exigibilidade_iss: "1".to_string(),
                iss_retido: "2".to_string(),
                item_lista_servico: "14.01".to_string(),
                municipio_incidencia: "3511102".to_string(),
                responsavel_retencao: "1".to_string(),
            },
            prestador: Prestador {
                cnpj: "99999999999999".to_string(),
                inscricao_municipal: "9999999999".to_string(),
                nome_fantasia: "NOME FANTASIA PRESTADOR".to_string(),
                razao_social: "RAZAO SOCIAL PRETADOR".to_string(),
                endereco: Endereco {
                    bairro: "BAIRRO PRESTADOR".to_string(),
                    cep: "99999999".to_string(),
                    codigo_municipio: "3511102".to_string(),
                    codigo_pais: "1058".to_string(),
                    complemento: "COMPLEMENTO PRESTADOR".to_string(),
                    logradouro: "LOGRADOURO PRESTADOR".to_string(),
                    numero: "999".to_string(),
                    uf: "SP".to_string(),
                },
                email: "prestador@email.com.br".to_string(),
                telefone: "99999999999".to_string(),
            },
            tomador: Tomador {
                cnpj: "99999999999999".to_string(),
                cpf: "null".to_string(),
                inscricao_municipal: "9999999999".to_string(),
                razao_social: "RASAO SOCIAL TOMADOR".to_string(),
                endereco: Endereco {
                    bairro: "BAIRRO TOMADOR".to_string(),
                    cep: "99999999".to_string(),
                    codigo_pais: "Null".to_string(),
                    complemento: "Null".to_string(),
                    codigo_municipio: "3529302".to_string(),
                    logradouro: "LOGRADOURO TOMADOR".to_string(),
                    numero: "999".to_string(),
                    uf: "SP".to_string(),
                },
                email: "tomador@email.com.br".to_string(),
                telefone: "99999999999".to_string(),
            },
            valores: Valores {
                aliquota: "3.9986".to_string(),
                base_calculo: "2775.80".to_string(),
                desconto_incondicionado: "0.00".to_string(),
                desconto_condicionado: "0.00".to_string(),
                outras_retencoes: "0.00".to_string(),
                valor_cofins: "0.00".to_string(),
                valor_csll: "0.00".to_string(),
                valor_deducoes: "0.00".to_string(),
                valor_inss: "0.00".to_string(),
                valor_ir: "0.00".to_string(),
                valor_iss: "110.99".to_string(),
                valor_pis: "0.00".to_string(),
                valor_servicos: "2775.80".to_string(),
            },
        };

        let nfse_layout_folder_path = "layouts";
        let nfse_json_path = "models/model_nfse_catanduva_01_normal_com_rps.xml";
        let nfse_catanduva = modules::nfse::get_nfse(nfse_layout_folder_path, nfse_json_path)
            .expect("error getting nfse");
        assert_eq!(result, nfse_catanduva);
    }
}
