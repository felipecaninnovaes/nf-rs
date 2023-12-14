pub mod objects {

    #[derive(Debug)]
    pub struct Produto {
        pub n_item: i64,
        pub c_prod: String,
        pub c_ean: String,
        pub x_prod: String,
        pub ncm: i64,
        pub cfop: i64,
        pub u_com: String,
        pub q_com: f64,
        pub v_un_com: f64,
        pub v_prod: f64,
        pub c_eantrib: String,
        pub u_trib: String,
        pub q_trib: f64,
        pub v_un_trib: f64,
        pub ind_tot: i64,
        pub x_ped: String,
        pub icms: Icms,
        pub ipi: Ipi,
    }
    #[derive(Debug)]
    pub struct Icms {
        pub orig: i64,
        pub cst: i64,
        pub mod_bc: i64,
        pub v_bc: f64,
        pub p_icms: f64,
        pub v_icms: f64,
    }
    #[derive(Debug)]
    pub struct Ipi {
        pub c_enq: i64,
        pub cst: i64,
    }

    #[derive(Debug)]
    pub struct Pis {
        pub cst: i64,
        pub v_bc: f64,
        pub p_pis: f64,
        pub v_pis: f64,
    }
    #[derive(Debug)]
    pub struct Cofins {
        pub cst: i64,
        pub v_bc: f64,
        pub p_cofins: f64,
        pub v_cofins: f64,
    }
    #[derive(Debug)]
    pub struct IcmsUfDest {
        pub v_bcufdest: f64,
        pub v_bcfcpufdest: f64,
        pub p_fcpufdest: f64,
        pub p_icmsufdest: f64,
        pub p_icmsinter: f64,
        pub p_icmsinter_part: f64,
        pub v_fcpufdest: f64,
        pub v_icmsufdest: f64,
        pub v_icmsufremet: f64,
    }
    #[derive(Debug)]
    pub struct Imposto {
        pub icms: Icms,
        pub pis: Pis,
        pub confins: Cofins,
        pub icms_uf_dest: IcmsUfDest,
    }

    
}
