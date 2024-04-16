use std::fmt::{Display, Formatter, Result};

use crate::modules::structs::*;

impl Display for Data {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "field: {}\nfieldpath: {}", self.field, self.fieldpath)
    }
}

impl Display for ContatoJson {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "email: {}\ntelefone: {}", self.email, self.telefone)
    }
}

impl Display for EnderecoJson {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "bairro: {}\ncep: {}\ncodigo_municipio: {}\ncodigo_pais: {}\ncomplemento: {}\nlogradouro: {}\nnumero: {}\nuf: {}",
            self.bairro, self.cep, self.codigo_municipio, self.codigo_pais, self.complemento, self.logradouro, self.numero, self.uf
        )
    }
}

impl Display for DadosTomador {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "cnpj: {}\ncpf: {}\nrazao_social: {}",
            self.cnpj, self.cpf, self.razao_social
        )
    }
}

impl Display for DadosPrestador {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "cnpj: {}\ninscricao_municipal: {}\nnome_fantasia: {}\nrazao_social: {}",
            self.cnpj, self.inscricao_municipal, self.nome_fantasia, self.razao_social
        )
    }
}

impl Display for PrestadorJson {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "dados_prestador: {}\nendereco: {}\ncontato: {}",
            self.dados_prestador, self.endereco, self.contato
        )
    }
}

impl Display for TomadorJson {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "dados_tomador: {}\nendereco: {}\ncontato: {}",
            self.dados_tomador, self.endereco, self.contato
        )
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "layout: {}\nidentifier: {}\nprestador_json: {}\ntomador_json: {}\ndados_nfse: {:?}\nvalores: {:?}",
            self.layout, self.identifier, self.prestador_json, self.tomador_json, self.dados_nfse, self.valores
        )
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "key: {}\nvalue: {}", self.key, self.value)
    }
}

impl Display for DadosNfse {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "numero_da_nota: {}\ncompetencia: {}\ncodigo_do_municipio: {}\ncodigo_tributacao_mucipio: {}\nexigibilidade_iss: {}\niss_retido: {}\nitem_lista_servico: {}\nmunicipio_incidencia: {}\nresponsavel_retencao: {}",
            self.numero_da_nota, self.competencia, self.codigo_do_municipio, self.codigo_tributacao_mucipio, self.exigibilidade_iss, self.iss_retido, self.item_lista_servico, self.municipio_incidencia, self.responsavel_retencao
        )
    }
}

impl Display for Prestador {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "cnpj: {}\ninscricao_municipal: {}\nnome_fantasia: {}\nrazao_social: {}\nendereco: {}\nemail: {}\ntelefone: {}",
            self.cnpj, self.inscricao_municipal, self.nome_fantasia, self.razao_social, self.endereco, self.email, self.telefone
        )
    }
}

impl Display for Endereco {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "bairro: {}\ncep: {}\ncodigo_municipio: {}\ncodigo_pais: {}\ncomplemento: {}\nlogradouro: {}\nnumero: {}\nuf: {}",
            self.bairro, self.cep, self.codigo_municipio, self.codigo_pais, self.complemento, self.logradouro, self.numero, self.uf
        )
    }
}

impl Display for Tomador {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "cnpj: {}\ncpf: {}\nincricao_municipal: {}\nrazao_social: {}\nendereco: {}\nemail: {}\ntelefone: {}",
            self.cnpj, self.cpf, self.inscricao_municipal, self.razao_social, self.endereco, self.email, self.telefone
        )
    }
}
impl Display for Valores {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "aliquota: {}\nbase_calculo: {}\ndesconto_incondicionado: {}\ndesconto_condicionado: {}\noutras_retencoes: {}\nvalor_cofins: {}\nvalor_csll: {}\nvalor_deducoes: {}\nvalor_inss: {}\nvalor_ir: {}\nvalor_iss: {}\nvalor_pis: {}\nvalor_servicos: {}",
            self.aliquota, self.base_calculo, self.desconto_incondicionado, self.desconto_condicionado, self.outras_retencoes, self.valor_cofins, self.valor_csll, self.valor_deducoes, self.valor_inss, self.valor_ir, self.valor_iss, self.valor_pis, self.valor_servicos
        )
    }
}

impl Display for Nfse {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "\ndados_nfse: {}\n\nprestador: {}\n\ntomador: {}\n\nvalores: {}",
            self.dados_nfse, self.prestador, self.tomador, self.valores
        )
    }
}
