use sqlx::prelude::FromRow;

#[derive(Debug, PartialEq, FromRow)]
pub struct IcmsId {
    pub icms_idicms: i32,
}

#[derive(Debug, PartialEq)]
pub struct Icms {
    pub icms_orig: i64,
    pub icms_cst: i64,
    pub icms_modbc: i64,
    pub icms_vbc: f64,
    pub icms_picms: f64,
    pub icms_vicms: f64,
}

#[derive(Debug, PartialEq, FromRow)]
pub struct IpiId {
    pub ipi_idipi: i32,
}

#[derive(Debug, PartialEq)]
pub struct Ipi {
    pub ipi_cenq: i64,
    pub ipi_cst: i64,
    pub ipi_vbc: f64,
    pub ipi_pipi: i64,
    pub ipi_vipi: i64,
}

#[derive(Debug, PartialEq, FromRow)]
pub struct PisId {
    pub pis_idpis: i32,
}

#[derive(Debug, PartialEq)]
pub struct Pis {
    pub pis_cst: i64,
    pub pis_vbc: f64,
    pub pis_ppis: f64,
    pub pis_vpis: f64,
}

#[derive(Debug, PartialEq, FromRow)]
pub struct CofinsId {
    pub cofins_idcofins: i32,
}

#[derive(Debug, PartialEq)]
pub struct Cofins {
    pub cofins_cst: i64,
    pub cofins_vbc: f64,
    pub cofins_pcofins: f64,
    pub cofins_vcofins: f64,
}

#[derive(Debug, PartialEq, FromRow)]
pub struct IcmsUfDestId {
    pub icms_uf_idicmsufdest: i32,
}

#[derive(Debug, PartialEq)]
pub struct IcmsUfDest {
    pub icms_uf_vbcufdest: f64,
    pub icms_uf_vbcfcpufdest: f64,
    pub icms_uf_pfcpufdest: f64,
    pub icms_uf_picmsufdest: f64,
    pub icms_uf_picmsinter: f64,
    pub icms_uf_picmsinterpart: f64,
    pub icms_uf_vfcpufdest: f64,
    pub icms_uf_vicmsufdest: f64,
    pub icms_uf_vicmsufremet: f64,
}

#[derive(Debug, PartialEq, FromRow)]
pub struct ImpostoId {
    pub imposto_idimposto: i32,
}

#[derive(Debug, PartialEq)]
pub struct Imposto {
    pub imposto_icms: Icms,
    pub imposto_ipi: Ipi,
    pub imposto_pis: Pis,
    pub imposto_cofins: Cofins,
    pub imposto_icms_uf_dest: IcmsUfDest,
}
