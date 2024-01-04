use crate::modules::json::structs::dest::Dest;
use crate::modules::json::structs::emit::Emit;
use crate::modules::json::structs::ender::Ender;
use crate::modules::json::structs::nfe::Nfe;
use crate::modules::json::structs::produtos::Produto;

use crate::modules::json::structs::impostos::{Cofins, Icms, IcmsUfDest, Ipi, Pis};

use sqlx::Row;
use std::error::Error;

// read enderid from nfe_database
pub async fn get_idender(
    pool: &sqlx::PgPool,
    nro: &String,
    cep: &String,
) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT ender_idender FROM nfe_ender WHERE ender_nro = $1 AND ender_cep = $2";
    let idender = sqlx::query_scalar(q)
        .bind(nro)
        .bind(cep)
        .fetch_one(pool)
        .await?;
    Ok(idender)
}

// insert ender into nfe_database
pub async fn insert_ender(pool: &sqlx::PgPool, ender: &Ender) -> Result<i32, i32> {
    match get_idender(pool, &ender.ender_nro, &ender.ender_cep).await {
        Ok(idender) => Ok(idender),
        Err(_) => {
            let q = "INSERT INTO nfe_ender (ender_cep, ender_uf, ender_cmun, ender_cpais, ender_nro, ender_xbairro, ender_xcpl, ender_xlgr, ender_xmun) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING ender_idender";
            let idender = sqlx::query(q)
                .bind(&ender.ender_cep)
                .bind(&ender.ender_uf)
                .bind(&ender.ender_cmun)
                .bind(&ender.ender_cpais)
                .bind(&ender.ender_nro)
                .bind(&ender.ender_xbairro)
                .bind(&ender.ender_xcpl)
                .bind(&ender.ender_xlgr)
                .bind(&ender.ender_xmun)
                .fetch_one(pool)
                .await
                .unwrap()
                .get::<i32, _>(0);
            Ok(idender)
        }
    }

    // Ok(Result)
}

// get id from nfe_emit
pub async fn get_idemit(pool: &sqlx::PgPool, cnpj_cpf: &String) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT emit_idemit FROM nfe_emit WHERE emit_cnpjcpf = $1";
    let iddest = sqlx::query(q)
        .bind(cnpj_cpf)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    println!("iddest: {}", iddest);
    Ok(iddest)
}

// insert emit into nfe_database
pub async fn insert_emit(pool: &sqlx::PgPool, emit: &Emit) -> Result<i32, i32> {
    match get_idemit(pool, &emit.emit_cnpjcpf).await {
        Ok(idemit) => Ok(idemit),
        Err(_) => {
            let idender = insert_ender(pool, &emit.ender_emit).await.unwrap();
            let q = "INSERT INTO nfe_emit (emit_cnpjcpf, emit_crt, emit_ie, emit_iest, emit_xfant, emit_xnome, emit_idender ) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING emit_idemit";
            let idemit = sqlx::query(q)
                .bind(&emit.emit_cnpjcpf)
                .bind(&emit.emit_crt)
                .bind(&emit.emit_ie)
                .bind(&emit.emit_iest)
                .bind(&emit.emit_xfant)
                .bind(&emit.emit_xnome)
                .bind(idender)
                .fetch_one(pool)
                .await
                .unwrap()
                .get::<i32, _>(0);
            Ok(idemit)
        }
    }

    // Ok(Result)
}

// get id from nfe_dest
pub async fn get_iddest(pool: &sqlx::PgPool, cpf_cnpj: &String) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT dest_iddest FROM nfe_dest WHERE dest_cnpjcpf = $1";
    let iddest = sqlx::query(q)
        .bind(cpf_cnpj)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(iddest)
}

// insert dest into nfe_database
pub async fn insert_dest(pool: &sqlx::PgPool, dest: &Dest) -> Result<i32, i32> {
    match get_iddest(pool, &dest.dest_cnpjcpf).await {
        Ok(iddest) => {
            println!(
                "Destinatario já existe no banco de dados DestID: {}",
                iddest
            );
            Ok(iddest)
        }
        Err(_) => {
            let idender = insert_ender(pool, &dest.dest_ender).await.unwrap();
            let q = "INSERT INTO nfe_dest (dest_cnpjcpf, dest_ie, dest_email, dest_indiedest, dest_xnome, dest_idender) VALUES ($1, $2, $3, $4, $5, $6) RETURNING dest_iddest";
            let iddest = sqlx::query(q)
                .bind(&dest.dest_cnpjcpf)
                .bind(&dest.dest_ie)
                .bind(&dest.dest_email)
                .bind(&dest.dest_indiedest)
                .bind(&dest.dest_xnome)
                .bind(idender)
                .fetch_one(pool)
                .await
                .unwrap()
                .get::<i32, _>(0);
            Ok(iddest)
        }
    }

    // Ok(Result)
}

// get id from nfe
pub async fn get_nfeid(
    pool: &sqlx::PgPool,
    nnf: &String,
    idemit: &i32,
) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT nfe_idnfe FROM nfe WHERE nfe_nnf = $1 AND nfe_idemit = $2";
    let idnfe = sqlx::query(q)
        .bind(nnf)
        .bind(idemit)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idnfe)
}

// insert nfe into nfe_database
pub async fn insert_nfe(pool: &sqlx::PgPool, nfe: &Nfe) -> Result<i32, i32> {
    let idemit = insert_emit(pool, &nfe.nfe_emit).await.unwrap();
    let iddest = insert_dest(pool, &nfe.nfe_dest).await.unwrap();
    match get_nfeid(pool, &nfe.nfe_nnf, &idemit).await {
        Ok(idnfe) => Ok(idnfe),
        Err(_) => {
            let q = "INSERT INTO nfe (nfe_cdv, nfe_cmunfg, nfe_cnf, nfe_cuf, nfe_dhemi, nfe_dhsaient, nfe_finnfe, nfe_nfe_iddest, nfe_indfinal, nfe_indintermed, nfe_indpres, nfe_modnfe, nfe_nnf, nfe_natop, nfe_procemi, nfe_serie, nfe_tpamb, nfe_tpemis, nfe_tpimp, nfe_tpnf, nfe_verproc, nfe_nftotal, nfe_idemit, nfe_iddest ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) RETURNING nfe_idnfe";
            let idnfe = sqlx::query(q)
                .bind(&nfe.nfe_cdv)
                .bind(&nfe.nfe_cmunfg)
                .bind(&nfe.nfe_cnf)
                .bind(&nfe.nfe_cuf)
                .bind(&nfe.nfe_dhemi)
                .bind(&nfe.nfe_dhsaient)
                .bind(&nfe.nfe_finnfe)
                .bind(&nfe.nfe_nfe_iddest)
                .bind(&nfe.nfe_indfinal)
                .bind(&nfe.nfe_indintermed)
                .bind(&nfe.nfe_indpres)
                .bind(&nfe.nfe_modnfe)
                .bind(&nfe.nfe_nnf)
                .bind(&nfe.nfe_natop)
                .bind(&nfe.nfe_procemi)
                .bind(&nfe.nfe_serie)
                .bind(&nfe.nfe_tpamb)
                .bind(&nfe.nfe_tpemis)
                .bind(&nfe.nfe_tpimp)
                .bind(&nfe.nfe_tpnf)
                .bind(&nfe.nfe_verproc)
                .bind(nfe.nfe_nftotal)
                .bind(idemit)
                .bind(iddest)
                .fetch_one(pool)
                .await
                .unwrap()
                .get::<i32, _>(0);
            Ok(idnfe)
        }
    }
}

// get id from nfe_produto
pub async fn get_idproduto(
    pool: &sqlx::PgPool,
    nitem: &String,
    idnfe: &i32,
) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT produto_idproduto FROM nfe_produto WHERE produto_nitem = $1 AND produto_idnfe = $2";
    // let idnfe_i32 = idnfe.parse::<i32>().unwrap();
    let idproduto = sqlx::query(q)
        .bind(nitem)
        .bind(idnfe)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idproduto)
}

// insert produto into nfe_database
pub async fn insert_produto(
    pool: &sqlx::PgPool,
    produto: &Vec<Produto>,
    idnfe: &i32,
) -> Result<(), i32> {
    for p in produto {
        // println!("{:?}", p);
        match get_idproduto(pool, &p.produto_nitem, idnfe).await {
            Ok(idproduto) => {
                insert_cofins(pool, &p.imposto.imposto_cofins, &idproduto)
                    .await
                    .unwrap();
                insert_icmsufdest(pool, &p.imposto.imposto_icms_uf_dest, &idproduto)
                    .await
                    .unwrap();
                insert_icms(pool, &p.imposto.imposto_icms, &idproduto)
                    .await
                    .unwrap();
                insert_ipi(pool, &p.imposto.imposto_ipi, &idproduto).await.unwrap();
                insert_pis(pool, &p.imposto.imposto_pis, &idproduto).await.unwrap();
                println!("Produto {} já existe no banco de dados", idproduto);
            }
            Err(_) => {
                let q = "INSERT INTO nfe_produto (produto_nitem, produto_cprod, produto_cean, produto_xprod, produto_ncm, produto_cfop, produto_ucom, produto_qcom, produto_vuncom, produto_vprod, produto_ceantrib, produto_utrib, produto_qtrib, produto_vuntrib, produto_indtot, produto_xped, produto_idnfe) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,$16, $17) RETURNING produto_idproduto";
                let idproduto = sqlx::query(q)
                    .bind(&p.produto_nitem)
                    .bind(&p.produto_cprod)
                    .bind(&p.produto_cean)
                    .bind(&p.produto_xprod)
                    .bind(&p.produto_ncm)
                    .bind(&p.produto_cfop)
                    .bind(&p.produto_ucom)
                    .bind(p.produto_qcom)
                    .bind(p.produto_vuncom)
                    .bind(p.produto_vprod)
                    .bind(&p.produto_ceantrib)
                    .bind(&p.produto_utrib)
                    .bind(p.produto_qtrib)
                    .bind(p.produto_vuntrib)
                    .bind(&p.produto_indtot)
                    .bind(&p.produto_xped)
                    .bind(idnfe)
                    .fetch_one(pool)
                    .await
                    .unwrap()
                    .get::<i32, _>(0);

                // insert impostos
                insert_cofins(pool, &p.imposto.imposto_cofins, &idproduto)
                    .await
                    .unwrap();
                insert_icmsufdest(pool, &p.imposto.imposto_icms_uf_dest, &idproduto)
                    .await
                    .unwrap();
                insert_icms(pool, &p.imposto.imposto_icms, &idproduto)
                    .await
                    .unwrap();
                insert_ipi(pool, &p.imposto.imposto_ipi, &idproduto).await.unwrap();
                insert_pis(pool, &p.imposto.imposto_pis, &idproduto).await.unwrap();
            }
        }
    }
    Ok(())
}

// get id from nfe_cofins
pub async fn get_idcofins(pool: &sqlx::PgPool, idproduto: &i32) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT cofins_idcofins FROM nfe_cofins WHERE cofins_idproduto = $1";
    let idcofins = sqlx::query(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idcofins)
}

// insert cofins into nfe_database
pub async fn insert_cofins(
    pool: &sqlx::PgPool,
    cofins: &Cofins,
    idproduto: &i32,
) -> Result<(), i32> {
    let q = "INSERT INTO nfe_cofins (cofins_cst, cofins_vbc, cofins_pcofins, cofins_vcofins, cofins_idproduto) VALUES ($1, $2, $3, $4, $5)";
    match get_idcofins(pool, idproduto).await {
        Ok(idcofins) => {
            println!(
                "Cofins do produto com o ID: {} já existe no banco de dados",
                idcofins
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(cofins.cofins_cst)
                .bind(cofins.cofins_vbc)
                .bind(cofins.cofins_pcofins)
                .bind(cofins.cofins_vcofins)
                .bind(idproduto)
                .execute(pool)
                .await
                .unwrap();
        }
    }
    Ok(())
}

// get id from nfe_icmsufdest
pub async fn get_idicmsufdest(pool: &sqlx::PgPool, idproduto: &i32) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT icms_uf_idicmsufdest FROM nfe_icmsufdest WHERE icms_uf_idproduto = $1";
    let idicmsufdest = sqlx::query(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idicmsufdest)
}

// insert icmsufdest into nfe_database
pub async fn insert_icmsufdest(
    pool: &sqlx::PgPool,
    icmsufdest: &IcmsUfDest,
    idproduto: &i32,
) -> Result<(), i32> {
    let q = "INSERT INTO nfe_icmsufdest (icms_uf_vbcufdest, icms_uf_vbcfcpufdest, icms_uf_pfcpufdest, icms_uf_picmsufdest, icms_uf_picmsinter, icms_uf_picmsinterpart, icms_uf_vfcpufdest, icms_uf_vicmsufdest, icms_uf_vicmsufremet, icms_uf_idproduto) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";
    match get_idicmsufdest(pool, idproduto).await {
        Ok(idicmsufdest) => {
            println!(
                "IcmsUfDest do produto com o ID: {} já existe no banco de dados",
                idicmsufdest
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(icmsufdest.icms_uf_vbcufdest)
                .bind(icmsufdest.icms_uf_vbcfcpufdest)
                .bind(icmsufdest.icms_uf_pfcpufdest)
                .bind(icmsufdest.icms_uf_picmsufdest)
                .bind(icmsufdest.icms_uf_picmsinter)
                .bind(icmsufdest.icms_uf_picmsinterpart)
                .bind(icmsufdest.icms_uf_vfcpufdest)
                .bind(icmsufdest.icms_uf_vicmsufdest)
                .bind(icmsufdest.icms_uf_vicmsufremet)
                .bind(idproduto)
                .execute(pool)
                .await
                .unwrap();
        }
    }
    Ok(())
}

// get id from nfe_icms
pub async fn get_idicms(pool: &sqlx::PgPool, idproduto: &i32) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT icms_idicms FROM nfe_icms WHERE icms_idproduto = $1";
    let idicms = sqlx::query(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idicms)
}

// insert icms into nfe_database
pub async fn insert_icms(pool: &sqlx::PgPool, icms: &Icms, idproduto: &i32) -> Result<(), i32> {
    let q = "INSERT INTO nfe_icms (icms_orig, icms_cst, icms_modbc, icms_vbc, icms_picms, icms_vicms, icms_idproduto) VALUES ($1, $2, $3, $4, $5, $6, $7)";
    match get_idicms(pool, idproduto).await {
        Ok(idicms) => {
            println!(
                "Icms do produto com o ID: {} já existe no banco de dados",
                idicms
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(icms.icms_orig)
                .bind(icms.icms_cst)
                .bind(icms.icms_modbc)
                .bind(icms.icms_vbc)
                .bind(icms.icms_picms)
                .bind(icms.icms_vicms)
                .bind(idproduto)
                .execute(pool)
                .await
                .unwrap();
        }
    }
    Ok(())
}

// get id from nfe_ipi
pub async fn get_idipi(pool: &sqlx::PgPool, idproduto: &i32) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT ipi_idipi FROM nfe_ipi WHERE ipi_idproduto = $1";
    let idipi = sqlx::query(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idipi)
}

// insert ipi into nfe_database
pub async fn insert_ipi(pool: &sqlx::PgPool, ipi: &Ipi, idproduto: &i32) -> Result<(), i32> {
    let q = "INSERT INTO nfe_ipi (ipi_cenq, ipi_cst, ipi_vbc, ipi_pipi, ipi_vipi, ipi_idproduto) VALUES ($1, $2, $3, $4, $5, $6)";
    match get_idipi(pool, idproduto).await {
        Ok(idipi) => {
            println!(
                "Ipi do produto com o ID: {} já existe no banco de dados",
                idipi
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(ipi.ipi_cenq)
                .bind(ipi.ipi_cst)
                .bind(ipi.ipi_vbc)
                .bind(ipi.ipi_pipi)
                .bind(ipi.ipi_vipi)
                .bind(idproduto)
                .execute(pool)
                .await
                .unwrap();
        }
    }
    Ok(())
}

// get id from nfe_pis
pub async fn get_idpis(pool: &sqlx::PgPool, idproduto: &i32) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT pis_idpis FROM nfe_pis WHERE pis_idproduto = $1";
    let idpis = sqlx::query(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idpis)
}

// insert pis into nfe_database
pub async fn insert_pis(pool: &sqlx::PgPool, pis: &Pis, idproduto: &i32) -> Result<(), i32> {
    let q = "INSERT INTO nfe_pis (pis_cst, pis_vbc, pis_ppis, pis_vpis, pis_idproduto) VALUES ($1, $2, $3, $4, $5)";
    match get_idpis(pool, idproduto).await {
        Ok(idpis) => {
            println!(
                "Pis do produto com o ID: {} já existe no banco de dados",
                idpis
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(pis.pis_cst)
                .bind(pis.pis_vbc)
                .bind(pis.pis_ppis)
                .bind(pis.pis_vpis)
                .bind(idproduto)
                .execute(pool)
                .await
                .unwrap();
        }
    }
    Ok(())
}
