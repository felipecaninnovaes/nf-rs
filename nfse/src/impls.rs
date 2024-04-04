use std::fmt::{Display, Formatter, Result};

use crate::structs::{Config, DadosNfse, Data, Identifier, Nfse, Prestador, Tomador, Valores};

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "layout: {}\nidentifier: {}\ndata: {:?}",
            self.layout, self.identifier.key, self.prestador
        )
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
            "cnpj: {}\ninscricao_municipal: {}\nnome_fantasia: {}\nrazao_social: {}\nbairro: {}\ncep: {}\ncodigo_municipio: {}\ncodigo_pais: {}\ncomplemento: {}\nendereco: {}\nnumero: {}\nuf: {}\nemail: {}\ntelefone: {}",
            self.cnpj, self.inscricao_municipal, self.nome_fantasia, self.razao_social, self.bairro, self.cep, self.codigo_municipio, self.codigo_pais, self.complemento, self.endereco, self.numero, self.uf, self.email, self.telefone
        )
    }
}

impl Display for Tomador {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "cnpj: {}\ncpf: {}\nrazao_social: {}\nbairro: {}\ncep: {}\ncodigo_municipio: {}\nendereco: {}\nnumero: {}\nuf: {}\nemail: {}\ntelefone: {}",
            self.cnpj, self.cpf, self.razao_social, self.bairro, self.cep, self.codigo_municipio, self.endereco, self.numero, self.uf, self.email, self.telefone
        )
    }
}

impl Display for Valores {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "aliquota: {}\ndesconto_incondicionado: {}\ndesconto_condicionado: {}\noutras_retencoes: {}\nvalor_cofins: {}\nvalor_csll: {}\nvalor_deducoes: {}\nvalor_inss: {}\nvalor_ir: {}\nvalor_iss: {}\nvalor_pis: {}\nvalor_servicos: {}",
            self.aliquota, self.desconto_incondicionado, self.desconto_condicionado, self.outras_retencoes, self.valor_cofins, self.valor_csll, self.valor_deducoes, self.valor_inss, self.valor_ir, self.valor_iss, self.valor_pis, self.valor_servicos
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
