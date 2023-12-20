use crate::modules::json::structs::dest::Dest;
use crate::modules::json::structs::emit::Emit;
use crate::modules::json::structs::ender::Ender;
use crate::modules::json::structs::nfe::Nfe;
use crate::modules::json::structs::produtos::Produto;

use sqlx::Row;
use std::error::Error;

// read enderid from database
pub async fn get_idender(
    pool: &sqlx::PgPool,
    nro: &String,
    cep: &String,
) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT idender FROM ender WHERE nro = $1 AND cep = $2";
    let idender = sqlx::query(&q)
        .bind(nro)
        .bind(cep)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idender)
}

// insert ender into database
pub async fn insert_ender(pool: &sqlx::PgPool, ender: &Ender) -> Result<i32, i32> {
    match get_idender(pool, &ender.nro, &ender.cep).await {
        Ok(idender) => {
            return Ok(idender);
        }
        Err(_) => {
            let q = "INSERT INTO ender (cep, uf, cmun, cpais, nro, xbairro, xcpl, xlgr, xmun) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING idender";
            let idender = sqlx::query(&q)
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
            return Ok(idender);
        }
    }

    // Ok(Result)
}

// get id from emit
pub async fn get_idemit(pool: &sqlx::PgPool, cnpj_cpf: &String) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT idemit FROM emit WHERE cnpjcpf = $1";
    let iddest = sqlx::query(&q)
        .bind(cnpj_cpf)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(iddest)
}

// insert emit into database
pub async fn insert_emit(pool: &sqlx::PgPool, emit: &Emit) -> Result<i32, i32> {
    match get_idemit(pool, &emit.cnpj_cpf).await {
        Ok(idemit) => {
            return Ok(idemit);
        }
        Err(_) => {
            let idender = insert_ender(pool, &emit.ender_emit).await.unwrap();
            let q = "INSERT INTO emit (cnpjcpf, crt, ie, iest, xfant, xnome, enderidender) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING idemit";
            let idemit = sqlx::query(&q)
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
            return Ok(idemit);
        }
    }

    // Ok(Result)
}

// get id from dest
pub async fn get_iddest(pool: &sqlx::PgPool, cpf_cnpj: &String) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT iddest FROM dest WHERE cpfcnpj = $1";
    let iddest = sqlx::query(&q)
        .bind(cpf_cnpj)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(iddest)
}

// insert dest into database
pub async fn insert_dest(pool: &sqlx::PgPool, dest: &Dest) -> Result<i32, i32> {
    match get_iddest(pool, &dest.cnpj_cpf).await {
        Ok(iddest) => {
            return Ok(iddest);
        }
        Err(_) => {
            let idender = insert_ender(pool, &dest.ender_dest).await.unwrap();
            let q = "INSERT INTO dest (cnpjcpf , ie, email, indiedest,xnome, enderidender) VALUES ($1, $2, $3, $4, $5, $6) RETURNING iddest";
            let iddest = sqlx::query(&q)
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
            return Ok(iddest);
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
    let q = "SELECT idnfe FROM nfe WHERE nnf = $1 AND emitidemit = $2";
    let idnfe = sqlx::query(&q)
        .bind(nnf)
        .bind(idemit)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idnfe)
}

// insert nfe into database
pub async fn insert_nfe(pool: &sqlx::PgPool, nfe: &Nfe) -> Result<i32, i32> {
    let idemit = insert_emit(pool, &nfe.emit).await.unwrap();
    let iddest = insert_dest(pool, &nfe.dest).await.unwrap();
    match get_nfeid(pool, &nfe.n_nf, &idemit).await {
        Ok(idnfe) => {
            return Ok(idnfe);
        }
        Err(_) => {
            let q = "INSERT INTO nfe (cdv, cmunfg, cnf, cuf, dhemi, dhsaient, finnfe, iddest, indfinal, indintermed, indpres, modnfe, nnf, natop, procemi, serie, tpamb, tpemis, tpimp, tpnf, verproc, emitidemit, destiddest) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23) RETURNING idnfe";
            let idnfe = sqlx::query(&q)
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
                .bind(idemit)
                .bind(iddest)
                .fetch_one(pool)
                .await
                .unwrap()
                .get::<i32, _>(0);
            return Ok(idnfe);
        }
    }
}

// get id from produto
pub async fn get_idproduto(pool: &sqlx::PgPool, nitem: &String, idnfe: &i32) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT idproduto FROM produto WHERE nitem = $1 AND nfeidnfe = $2";
    // let idnfe_i32 = idnfe.parse::<i32>().unwrap();
    let idproduto = sqlx::query(&q)
        .bind(nitem)
        .bind(idnfe)
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(idproduto)
}
// insert produto into database
pub async fn insert_produto(pool: &sqlx::PgPool, produto: &Vec<Produto>, idnfe: &i32) -> Result<(), i32> {
    for p in produto {
        // println!("{:?}", p);
        match get_idproduto(pool, &p.n_item, &idnfe).await {
            Ok(idproduto) => {
                println!("Produto {} já existe no banco de dados", idproduto);
            }
            Err(_) => {
                let q = "INSERT INTO produto (nitem, cprod, cean, xprod, ncm, cfop, ucom, qcom, vuncom, vprod, ceantrib, utrib, qtrib, vuntrib, indtot, xped, nfeidnfe) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,$16, $17) RETURNING idproduto";
                sqlx::query(&q)
                    .bind(&p.n_item)
                    .bind(&p.c_prod)
                    .bind(&p.c_ean)
                    .bind(&p.x_prod)
                    .bind(&p.ncm)
                    .bind(&p.cfop)
                    .bind(&p.u_com)
                    .bind(&p.q_com)
                    .bind(&p.v_un_com)
                    .bind(&p.v_prod)
                    .bind(&p.c_eantrib)
                    .bind(&p.u_trib)
                    .bind(&p.q_trib)
                    .bind(&p.v_un_trib)
                    .bind(&p.ind_tot)
                    .bind(&p.x_ped)
                    .bind(idnfe)
                    .fetch_one(pool)
                    .await
                    .unwrap()
                    .get::<i32, _>(0);
            }
        }
    }
    Ok(())
}