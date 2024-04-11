use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DadosNfse {
    pub numero_da_nota: String,
    pub competencia: String,
    pub codigo_do_municipio: String,
    pub codigo_tributacao_mucipio: String,
    pub exigibilidade_iss: String,
    pub iss_retido: String,
    pub item_lista_servico: String,
    pub municipio_incidencia: String,
    pub responsavel_retencao: String,
}

#[derive(Deserialize, Debug)]
pub struct Prestador {
    pub cnpj: String,
    pub inscricao_municipal: String,
    pub nome_fantasia: String,
    pub razao_social: String,
    pub bairro: String,
    pub cep: String,
    pub codigo_municipio: String,
    pub codigo_pais: String,
    pub complemento: String,
    pub endereco: String,
    pub numero: String,
    pub uf: String,
    pub email: String,
    pub telefone: String,
}

#[derive(Deserialize, Debug)]
pub struct Tomador {
    pub cnpj: String,
    pub cpf: String,
    pub razao_social: String,
    pub bairro: String,
    pub cep: String,
    pub codigo_municipio: String,
    pub endereco: String,
    pub numero: String,
    pub uf: String,
    pub email: String,
    pub telefone: String,
}

#[derive(Deserialize, Debug)]
pub struct Valores {
    pub aliquota: String,
    pub base_calculo: String,
    pub desconto_incondicionado: String,
    pub desconto_condicionado: String,
    pub outras_retencoes: String,
    pub valor_cofins: String,
    pub valor_csll: String,
    pub valor_deducoes: String,
    pub valor_inss: String,
    pub valor_ir: String,
    pub valor_iss: String,
    pub valor_pis: String,
    pub valor_servicos: String,
}

#[derive(Deserialize, Debug)]
pub struct Nfse {
    pub dados_nfse: DadosNfse,
    pub prestador: Prestador,
    pub tomador: Tomador,
    pub valores: Valores,
}

#[derive(Deserialize, Debug)]
pub struct DadosPrestador {
    pub cnpj: Data,
    pub inscricao_municipal: Data,
    pub nome_fantasia: Data,
    pub razao_social: Data,
}

#[derive(Deserialize, Debug)]
pub struct PrestadorJson {
    pub dados_prestador: DadosPrestador,
    pub endereco: EnderecoJson,
    pub contato: ContatoJson,
}

#[derive(Deserialize, Debug)]
pub struct TomadorJson {
    pub dados_tomador: DadosTomador,
    pub endereco: EnderecoJson,
    pub contato: ContatoJson,
}

#[derive(Deserialize, Debug)]
pub struct DadosTomador {
    pub cnpj: Data,
    pub cpf: Data,
    pub razao_social: Data,
}

#[derive(Deserialize, Debug)]
pub struct EnderecoJson {
    pub bairro: Data,
    pub cep: Data,
    pub codigo_municipio: Data,
    pub codigo_pais: Data,
    pub complemento: Data,
    pub logradouro: Data,
    pub numero: Data,
    pub uf: Data,
}

#[derive(Deserialize, Debug)]
pub struct ContatoJson {
    pub email: Data,
    pub telefone: Data,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub layout: String,
    pub identifier: Identifier,
    pub prestador_json: PrestadorJson,
    pub tomador_json: TomadorJson,
    pub dados_nfse: Vec<Data>,
    pub valores: Vec<Data>,
}

#[derive(Deserialize, Debug)]
pub struct Identifier {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub field: String,
    pub fieldpath: String,
}
