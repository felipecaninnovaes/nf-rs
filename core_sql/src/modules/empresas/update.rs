use sqlx::PgPool;
use std::error::Error;

use crate::structs::empresas::empresa_struct::UpdateEmpresasModel;

#[allow(dead_code)]
pub async fn update_empresa(
    pool: &PgPool,
    empresa: UpdateEmpresasModel,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"UPDATE empresas SET nome = $1 ,nome_fant = $2 ,rua = $3 ,numero = $4 ,bairro = $5 ,cidade = $6 ,estado = $7 ,cep = $8 ,telefone = $9 ,email = $10 ,regime_tributario = $11 WHERE cnpj= $12"#,
        empresa.nome.to_uppercase(),
        empresa.nome_fant.to_uppercase(),
        empresa.rua.to_uppercase(),
        empresa.numero.to_uppercase(),
        empresa.bairro.to_uppercase(),
        empresa.cidade.to_uppercase(),
        empresa.estado.to_uppercase(),
        empresa.cep.to_uppercase(),
        empresa.telefone.to_uppercase(),
        empresa.email.to_uppercase(),
        empresa.regime_tributario.to_uppercase(),
        empresa.cnpj.to_uppercase(),
    ).execute(pool)
    .await?;

    Ok(())
}
