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
        pub v_bc: f64,
        pub p_ipi: i64,
        pub v_ipi: i64,
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
    pub struct Impostos {
        pub icms: Icms,
        pub ipi: Ipi,
        pub pis: Pis,
        pub cofins: Cofins,
        pub icms_uf_dest: IcmsUfDest,
    }
