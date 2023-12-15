pub mod objects {
    use crate::modules::json::structs::impostos::impostos::Impostos;

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
        pub impostos: Impostos
    }    
}
