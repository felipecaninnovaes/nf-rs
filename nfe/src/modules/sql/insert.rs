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
    let q = "SELECT nfe_idender FROM nfe_ender WHERE nro = $1 AND cep = $2";
    let idender = sqlx::query_scalar(q)
        .bind(nro)
        .bind(cep)
        .fetch_one(pool)
        .await?;
    Ok(idender)
}

// insert ender into nfe_database
pub async fn insert_ender(pool: &sqlx::PgPool, ender: &Ender) -> Result<i32, i32> {
    match get_idender(pool, &ender.nro, &ender.cep).await {
        Ok(idender) => {
            Ok(idender)
        }
        Err(_) => {
            let q = "INSERT INTO nfe_ender (cep, uf, cmun, cpais, nro, xbairro, xcpl, xlgr, xmun) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING nfe_idender";
            let idender = sqlx::query(q)
                .bind(&ender.cep)
                .bind(&ender.uf)
                .bind(&ender.c_mun)
                .bind(&ender.c_pais)
                .bind(&ender.nro)
                .bind(&ender.x_bairro)
                .bind(&ender.x_cpl)
                .bind(&ender.x_lgr)
                .bind(&ender.x_mun)
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
    let q = "SELECT nfe_idemit FROM nfe_emit WHERE cnpjcpf = $1";
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
    match get_idemit(pool, &emit.cnpj_cpf).await {
        Ok(idemit) => {
            Ok(idemit)
        }
        Err(_) => {
            let idender = insert_ender(pool, &emit.ender_emit).await.unwrap();
            let q = "INSERT INTO nfe_emit (cnpjcpf, crt, ie, iest, xfant, xnome, nfe_idender) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING nfe_idemit";
            let idemit = sqlx::query(q)
                .bind(&emit.cnpj_cpf)
                .bind(&emit.crt)
                .bind(&emit.ie)
                .bind(&emit.iest)
                .bind(&emit.x_fant)
                .bind(&emit.x_nome)
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
    let q = "SELECT nfe_iddest FROM nfe_dest WHERE cnpjcpf = $1";
    let iddest = sqlx::query(q)
        .bind(cpf_cnpj)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(iddest)
}

// insert dest into nfe_database
pub async fn insert_dest(pool: &sqlx::PgPool, dest: &Dest) -> Result<i32, i32> {
    match get_iddest(pool, &dest.cnpj_cpf).await {
        Ok(iddest) => {
            println!(
                "Destinatario já existe no banco de dados DestID: {}",
                iddest
            );
            Ok(iddest)
        }
        Err(_) => {
            let idender = insert_ender(pool, &dest.ender_dest).await.unwrap();
            let q = "INSERT INTO nfe_dest (cnpjcpf , ie, email, indiedest,xnome, nfe_idender) VALUES ($1, $2, $3, $4, $5, $6) RETURNING nfe_iddest";
            let iddest = sqlx::query(q)
                .bind(&dest.cnpj_cpf)
                .bind(&dest.ie)
                .bind(&dest.email)
                .bind(&dest.ind_iedest)
                .bind(&dest.x_nome)
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
    let q = "SELECT nfe_idnfe FROM nfe WHERE nnf = $1 AND nfe_idemit = $2";
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
    let idemit = insert_emit(pool, &nfe.emit).await.unwrap();
    let iddest = insert_dest(pool, &nfe.dest).await.unwrap();
    match get_nfeid(pool, &nfe.n_nf, &idemit).await {
        Ok(idnfe) => {
            Ok(idnfe)
        }
        Err(_) => {
            let q = "INSERT INTO nfe (cdv, cmunfg, cnf, cuf, dhemi, dhsaient, finnfe, iddest, indfinal, indintermed, indpres, modnfe, nnf, natop, procemi, serie, tpamb, tpemis, tpimp, tpnf, verproc, nftotal, nfe_idemit, nfe_iddest) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) RETURNING idnfe";
            let idnfe = sqlx::query(q)
                .bind(&nfe.c_dv)
                .bind(&nfe.c_mun_fg)
                .bind(&nfe.c_nf)
                .bind(&nfe.c_uf)
                .bind(&nfe.dh_emi)
                .bind(&nfe.dh_sai_ent)
                .bind(&nfe.fin_nfe)
                .bind(&nfe.id_dest)
                .bind(&nfe.ind_final)
                .bind(&nfe.ind_intermed)
                .bind(&nfe.ind_pres)
                .bind(&nfe.mod_nfe)
                .bind(&nfe.n_nf)
                .bind(&nfe.nat_op)
                .bind(&nfe.proc_emi)
                .bind(&nfe.serie)
                .bind(&nfe.tp_amb)
                .bind(&nfe.tp_emis)
                .bind(&nfe.tp_imp)
                .bind(&nfe.tp_nf)
                .bind(&nfe.ver_proc)
                .bind(nfe.nf_total)
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
    let q = "SELECT nfe_idproduto FROM nfe_produto WHERE nitem = $1 AND idnfe = $2";
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
        match get_idproduto(pool, &p.n_item, idnfe).await {
            Ok(idproduto) => {
                insert_cofins(pool, &p.impostos.cofins, &idproduto)
                    .await
                    .unwrap();
                insert_icmsufdest(pool, &p.impostos.icms_uf_dest, &idproduto)
                    .await
                    .unwrap();
                insert_icms(pool, &p.impostos.icms, &idproduto)
                    .await
                    .unwrap();
                insert_ipi(pool, &p.impostos.ipi, &idproduto).await.unwrap();
                insert_pis(pool, &p.impostos.pis, &idproduto).await.unwrap();
                println!("Produto {} já existe no banco de dados", idproduto);
            }
            Err(_) => {
                let q = "INSERT INTO nfe_produto (nitem, cprod, cean, xprod, ncm, cfop, ucom, qcom, vuncom, vprod, ceantrib, utrib, qtrib, vuntrib, indtot, xped, idnfe) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,$16, $17) RETURNING nfe_idproduto";
                let idproduto = sqlx::query(q)
                    .bind(&p.n_item)
                    .bind(&p.c_prod)
                    .bind(&p.c_ean)
                    .bind(&p.x_prod)
                    .bind(&p.ncm)
                    .bind(&p.cfop)
                    .bind(&p.u_com)
                    .bind(p.q_com)
                    .bind(p.v_un_com)
                    .bind(p.v_prod)
                    .bind(&p.c_eantrib)
                    .bind(&p.u_trib)
                    .bind(p.q_trib)
                    .bind(p.v_un_trib)
                    .bind(&p.ind_tot)
                    .bind(&p.x_ped)
                    .bind(idnfe)
                    .fetch_one(pool)
                    .await
                    .unwrap()
                    .get::<i32, _>(0);

                // insert impostos
                insert_cofins(pool, &p.impostos.cofins, &idproduto)
                    .await
                    .unwrap();
                insert_icmsufdest(pool, &p.impostos.icms_uf_dest, &idproduto)
                    .await
                    .unwrap();
                insert_icms(pool, &p.impostos.icms, &idproduto)
                    .await
                    .unwrap();
                insert_ipi(pool, &p.impostos.ipi, &idproduto).await.unwrap();
                insert_pis(pool, &p.impostos.pis, &idproduto).await.unwrap();
            }
        }
    }
    Ok(())
}

// get id from nfe_cofins
pub async fn get_idcofins(pool: &sqlx::PgPool, idproduto: &i32) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT nfe_idcofins FROM nfe_cofins WHERE nfe_idproduto = $1";
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
    let q = "INSERT INTO nfe_cofins (cst, vbc, pcofins, vcofins, nfe_idproduto) VALUES ($1, $2, $3, $4, $5)";
    match get_idcofins(pool, idproduto).await {
        Ok(idcofins) => {
            println!(
                "Cofins do produto com o ID: {} já existe no banco de dados",
                idcofins
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(cofins.cst)
                .bind(cofins.v_bc)
                .bind(cofins.p_cofins)
                .bind(cofins.v_cofins)
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
    let q = "SELECT nfe_idicmsufdest FROM nfe_icmsufdest WHERE nfe_idproduto = $1";
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
    let q = "INSERT INTO nfe_icmsufdest (vbcufdest, vbcfcpufdest, pfcpufdest, picmsufdest, picmsinter, picmsinterpart, vfcpufdest, vicmsufdest, vicmsufremet, nfe_idproduto) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";
    match get_idicmsufdest(pool, idproduto).await {
        Ok(idicmsufdest) => {
            println!(
                "IcmsUfDest do produto com o ID: {} já existe no banco de dados",
                idicmsufdest
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(icmsufdest.v_bcufdest)
                .bind(icmsufdest.v_bcfcpufdest)
                .bind(icmsufdest.p_fcpufdest)
                .bind(icmsufdest.p_icmsufdest)
                .bind(icmsufdest.p_icmsinter)
                .bind(icmsufdest.p_icmsinter_part)
                .bind(icmsufdest.v_fcpufdest)
                .bind(icmsufdest.v_icmsufdest)
                .bind(icmsufdest.v_icmsufremet)
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
    let q = "SELECT nfe_idicms FROM nfe_icms WHERE nfe_idproduto = $1";
    let idicms = sqlx::query(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idicms)
}

// insert icms into nfe_database
pub async fn insert_icms(pool: &sqlx::PgPool, icms: &Icms, idproduto: &i32) -> Result<(), i32> {
    let q = "INSERT INTO nfe_icms (orig, cst, modbc, vbc, picms, vicms, nfe_idproduto) VALUES ($1, $2, $3, $4, $5, $6, $7)";
    match get_idicms(pool, idproduto).await {
        Ok(idicms) => {
            println!(
                "Icms do produto com o ID: {} já existe no banco de dados",
                idicms
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(icms.orig)
                .bind(icms.cst)
                .bind(icms.mod_bc)
                .bind(icms.v_bc)
                .bind(icms.p_icms)
                .bind(icms.v_icms)
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
    let q = "SELECT nfe_idipi FROM nfe_ipi WHERE nfe_idproduto = $1";
    let idipi = sqlx::query(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idipi)
}

// insert ipi into nfe_database
pub async fn insert_ipi(pool: &sqlx::PgPool, ipi: &Ipi, idproduto: &i32) -> Result<(), i32> {
    let q = "INSERT INTO nfe_ipi (cenq, cst, vbc, pipi, vipi, nfe_idproduto) VALUES ($1, $2, $3, $4, $5, $6)";
    match get_idipi(pool, idproduto).await {
        Ok(idipi) => {
            println!(
                "Ipi do produto com o ID: {} já existe no banco de dados",
                idipi
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(ipi.c_enq)
                .bind(ipi.cst)
                .bind(ipi.v_bc)
                .bind(ipi.p_ipi)
                .bind(ipi.v_ipi)
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
    let q = "SELECT nfe_idpis FROM nfe_pis WHERE nfe_idproduto = $1";
    let idpis = sqlx::query(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idpis)
}

// insert pis into nfe_database
pub async fn insert_pis(pool: &sqlx::PgPool, pis: &Pis, idproduto: &i32) -> Result<(), i32> {
    let q = "INSERT INTO nfe_pis (cst, vbc, ppis, vpis, nfe_idproduto) VALUES ($1, $2, $3, $4, $5)";
    match get_idpis(pool, idproduto).await {
        Ok(idpis) => {
            println!(
                "Pis do produto com o ID: {} já existe no banco de dados",
                idpis
            );
        }
        Err(_) => {
            sqlx::query(q)
                .bind(pis.cst)
                .bind(pis.v_bc)
                .bind(pis.p_pis)
                .bind(pis.v_pis)
                .bind(idproduto)
                .execute(pool)
                .await
                .unwrap();
        }
    }
    Ok(())
}
