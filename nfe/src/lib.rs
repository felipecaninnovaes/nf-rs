
#![feature(test)]
pub mod modules; // Add crate attribute to enable unstable library feature 'test'
pub mod run;

#[cfg(test)]

mod tests {
    extern crate test;
    use test::Bencher;
    use crate::modules::json::structs::dest::Dest;
    use crate::modules::json::structs::emit::Emit;
    use crate::modules::json::structs::ender::Ender;
    use crate::modules::json::structs::impostos::*;
    use crate::modules::json::structs::nfe::Nfe;
    use crate::modules::json::structs::produtos::Produto;

    #[test]
    fn new_nfe_vec_products() {
        let input = Nfe::new("tests/data/1.xml");
        let expected = Nfe {
            c_dv: "3".to_string(),
            c_mun_fg: "3304557".to_string(),
            c_nf: "00151204".to_string(),
            c_uf: "33".to_string(),
            dh_emi: "2023-12-11T19:55:57-03:00".to_string(),
            dh_sai_ent: "2023-12-11T19:55:57-03:00".to_string(),
            fin_nfe: "1".to_string(),
            id_dest: "2".to_string(),
            ind_final: "1".to_string(),
            ind_intermed: "0".to_string(),
            ind_pres: "2".to_string(),
            mod_nfe: "55".to_string(),
            n_nf: "00000000001".to_string(),
            nat_op: "NOTA DE TESTE".to_string(),
            proc_emi: "0".to_string(),
            serie: "4".to_string(),
            tp_amb: "1".to_string(),
            tp_emis: "1".to_string(),
            tp_imp: "1".to_string(),
            tp_nf: "1".to_string(),
            ver_proc: "Notamax 1.0".to_string(),
            emit: Emit {
                cnpj_cpf: "99999999999999".to_string(),
                crt: "3".to_string(),
                ie: "86607492".to_string(),
                iest: "819012855115".to_string(),
                ender_emit: Ender {
                    cep: "9999999".to_string(),
                    uf: "RJ".to_string(),
                    c_mun: "3304557".to_string(),
                    c_pais: "1058".to_string(),
                    nro: "9999".to_string(),
                    x_bairro: "BAIRRO DA LOJA TEST".to_string(),
                    x_cpl: "Compemento".to_string(),
                    x_lgr: "RUA DA LOJA TEST".to_string(),
                    x_mun: "Rio de Janeiro".to_string(),
                },
                x_fant: "LOJA DE TESTE".to_string(),
                x_nome: "LOJA DE TESTE".to_string(),
            },
            dest: Dest {
                cnpj_cpf: "99999999999".to_string(),
                ie: "null".to_string(),
                email: "email@test.com".to_string(),
                ender_dest: Ender {
                    cep: "9999999".to_string(),
                    uf: "SP".to_string(),
                    c_mun: "99999999".to_string(),
                    c_pais: "1058".to_string(),
                    nro: "31".to_string(),
                    x_bairro: "BAIRRO DO CLIENTE PF DE TESTE".to_string(),
                    x_cpl: "null".to_string(),
                    x_lgr: "RUA DO CLIENTE PF DE TESTE".to_string(),
                    x_mun: "Catanduva".to_string(),
                },
                ind_iedest: "9".to_string(),
                x_nome: "CLIENTE PF DE TESTE".to_string(),
            },
            produtos: vec![
                Produto {
                    n_item: "1".to_string(),
                    c_prod: "678651245".to_string(),
                    c_ean: "SEM GTIN".to_string(),
                    x_prod: "CAMISETA MANGA CURTA ALGODAO".to_string(),
                    ncm: "61091000".to_string(),
                    cfop: "6108".to_string(),
                    u_com: "UN".to_string(),
                    q_com: 1.0,
                    v_un_com: 69.9,
                    v_prod: 69.9,
                    c_eantrib: "SEM GTIN".to_string(),
                    u_trib: "UN".to_string(),
                    q_trib: 1.0,
                    v_un_trib: 69.9,
                    ind_tot: "1".to_string(),
                    x_ped: "999999999".to_string(),
                    impostos: Impostos {
                        icms: Icms {
                            orig: 0,
                            cst: 0,
                            mod_bc: 3,
                            v_bc: 69.9,
                            p_icms: 12.0,
                            v_icms: 8.39,
                        },
                        ipi: Ipi {
                            c_enq: 0,
                            cst: 0,
                            v_bc: 0.0,
                            p_ipi: 0,
                            v_ipi: 0,
                        },
                        pis: Pis {
                            cst: 1,
                            v_bc: 57.32,
                            p_pis: 1.65,
                            v_pis: 0.95,
                        },
                        cofins: Cofins {
                            cst: 1,
                            v_bc: 57.32,
                            p_cofins: 7.6,
                            v_cofins: 4.36,
                        },
                        icms_uf_dest: IcmsUfDest {
                            v_bcufdest: 69.9,
                            v_bcfcpufdest: 69.9,
                            p_fcpufdest: 0.0,
                            p_icmsufdest: 18.0,
                            p_icmsinter: 12.0,
                            p_icmsinter_part: 100.0,
                            v_fcpufdest: 0.0,
                            v_icmsufdest: 4.19,
                            v_icmsufremet: 0.0,
                        },
                    },
                },
                Produto {
                    n_item: "2".to_string(),
                    c_prod: "742245576".to_string(),
                    c_ean: "SEM GTIN".to_string(),
                    x_prod: "CAMISETA MANGA CURTA ALGODAO GEOMETRICO".to_string(),
                    ncm: "61091000".to_string(),
                    cfop: "6108".to_string(),
                    u_com: "UN".to_string(),
                    q_com: 1.0,
                    v_un_com: 29.9,
                    v_prod: 29.9,
                    c_eantrib: "SEM GTIN".to_string(),
                    u_trib: "UN".to_string(),
                    q_trib: 1.0,
                    v_un_trib: 29.9,
                    ind_tot: "1".to_string(),
                    x_ped: "999999999".to_string(),
                    impostos: Impostos {
                        icms: Icms {
                            orig: 0,
                            cst: 0,
                            mod_bc: 3,
                            v_bc: 29.9,
                            p_icms: 12.0,
                            v_icms: 3.59,
                        },
                        ipi: Ipi {
                            c_enq: 0,
                            cst: 0,
                            v_bc: 0.0,
                            p_ipi: 0,
                            v_ipi: 0,
                        },
                        pis: Pis {
                            cst: 1,
                            v_bc: 24.52,
                            p_pis: 1.65,
                            v_pis: 0.4,
                        },
                        cofins: Cofins {
                            cst: 1,
                            v_bc: 24.52,
                            p_cofins: 7.6,
                            v_cofins: 1.86,
                        },
                        icms_uf_dest: IcmsUfDest {
                            v_bcufdest: 29.9,
                            v_bcfcpufdest: 29.9,
                            p_fcpufdest: 0.0,
                            p_icmsufdest: 18.0,
                            p_icmsinter: 12.0,
                            p_icmsinter_part: 100.0,
                            v_fcpufdest: 0.0,
                            v_icmsufdest: 1.79,
                            v_icmsufremet: 0.0,
                        },
                    },
                },
                Produto {
                    n_item: "3".to_string(),
                    c_prod: "882668327".to_string(),
                    c_ean: "SEM GTIN".to_string(),
                    x_prod: "CAMISETA MANGA CURTA MEIA MALHA".to_string(),
                    ncm: "61091000".to_string(),
                    cfop: "6108".to_string(),
                    u_com: "UN".to_string(),
                    q_com: 1.0,
                    v_un_com: 49.9,
                    v_prod: 49.9,
                    c_eantrib: "SEM GTIN".to_string(),
                    u_trib: "UN".to_string(),
                    q_trib: 1.0,
                    v_un_trib: 49.9,
                    ind_tot: "1".to_string(),
                    x_ped: "999999999".to_string(),
                    impostos: Impostos {
                        icms: Icms {
                            orig: 0,
                            cst: 0,
                            mod_bc: 3,
                            v_bc: 49.9,
                            p_icms: 12.0,
                            v_icms: 5.99,
                        },
                        ipi: Ipi {
                            c_enq: 0,
                            cst: 0,
                            v_bc: 0.0,
                            p_ipi: 0,
                            v_ipi: 0,
                        },
                        pis: Pis {
                            cst: 1,
                            v_bc: 40.92,
                            p_pis: 1.65,
                            v_pis: 0.68,
                        },
                        cofins: Cofins {
                            cst: 1,
                            v_bc: 40.92,
                            p_cofins: 7.6,
                            v_cofins: 3.11,
                        },
                        icms_uf_dest: IcmsUfDest {
                            v_bcufdest: 49.9,
                            v_bcfcpufdest: 49.9,
                            p_fcpufdest: 0.0,
                            p_icmsufdest: 18.0,
                            p_icmsinter: 12.0,
                            p_icmsinter_part: 100.0,
                            v_fcpufdest: 0.0,
                            v_icmsufdest: 2.99,
                            v_icmsufremet: 0.0,
                        },
                    },
                },
            ],
        };
        assert_eq!(input, expected);
    }

    #[test]
    fn new_nfe_single_products() {
        let input = Nfe::new("tests/data/2.xml");
        let expected = Nfe {
            c_dv: "3".to_string(),
            c_mun_fg: "3304557".to_string(),
            c_nf: "00151204".to_string(),
            c_uf: "33".to_string(),
            dh_emi: "2023-12-11T19:55:57-03:00".to_string(),
            dh_sai_ent: "2023-12-11T19:55:57-03:00".to_string(),
            fin_nfe: "1".to_string(),
            id_dest: "2".to_string(),
            ind_final: "1".to_string(),
            ind_intermed: "0".to_string(),
            ind_pres: "2".to_string(),
            mod_nfe: "55".to_string(),
            n_nf: "00000000001".to_string(),
            nat_op: "NOTA DE TESTE".to_string(),
            proc_emi: "0".to_string(),
            serie: "4".to_string(),
            tp_amb: "1".to_string(),
            tp_emis: "1".to_string(),
            tp_imp: "1".to_string(),
            tp_nf: "1".to_string(),
            ver_proc: "Notamax 1.0".to_string(),
            emit: Emit {
                cnpj_cpf: "99999999999999".to_string(),
                crt: "3".to_string(),
                ie: "86607492".to_string(),
                iest: "819012855115".to_string(),
                ender_emit: Ender {
                    cep: "9999999".to_string(),
                    uf: "RJ".to_string(),
                    c_mun: "3304557".to_string(),
                    c_pais: "1058".to_string(),
                    nro: "9999".to_string(),
                    x_bairro: "BAIRRO DA LOJA TEST".to_string(),
                    x_cpl: "Compemento".to_string(),
                    x_lgr: "RUA DA LOJA TEST".to_string(),
                    x_mun: "Rio de Janeiro".to_string(),
                },
                x_fant: "LOJA DE TESTE".to_string(),
                x_nome: "LOJA DE TESTE".to_string(),
            },
            dest: Dest {
                cnpj_cpf: "99999999999".to_string(),
                ie: "null".to_string(),
                email: "email@test.com".to_string(),
                ender_dest: Ender {
                    cep: "9999999".to_string(),
                    uf: "SP".to_string(),
                    c_mun: "99999999".to_string(),
                    c_pais: "1058".to_string(),
                    nro: "31".to_string(),
                    x_bairro: "BAIRRO DO CLIENTE PF DE TESTE".to_string(),
                    x_cpl: "null".to_string(),
                    x_lgr: "RUA DO CLIENTE PF DE TESTE".to_string(),
                    x_mun: "Catanduva".to_string(),
                },
                ind_iedest: "9".to_string(),
                x_nome: "CLIENTE PF DE TESTE".to_string(),
            },
            produtos: vec![Produto {
                n_item: "1".to_string(),
                c_prod: "000I0020NUD000M".to_string(),
                c_ean: "7899625704614".to_string(),
                x_prod: "ROUPA DE ALGUDAO".to_string(),
                ncm: "62121000".to_string(),
                cfop: "6102".to_string(),
                u_com: "UN".to_string(),
                q_com: 10.0,
                v_un_com: 61.43,
                v_prod: 614.3,
                c_eantrib: "7899625704614".to_string(),
                u_trib: "UN".to_string(),
                q_trib: 10.0,
                v_un_trib: 61.43,
                ind_tot: "1".to_string(),
                x_ped: "null".to_string(),
                impostos: Impostos {
                    icms: Icms {
                        orig: 1,
                        cst: 0,
                        mod_bc: 3,
                        v_bc: 583.59,
                        p_icms: 4.0,
                        v_icms: 23.34,
                    },
                    ipi: Ipi {
                        c_enq: 999,
                        cst: 50,
                        v_bc: 614.30,
                        p_ipi: 0,
                        v_ipi: 0,
                    },
                    pis: Pis {
                        cst: 1,
                        v_bc: 560.25,
                        p_pis: 1.65,
                        v_pis: 9.24,
                    },
                    cofins: Cofins {
                        cst: 1,
                        v_bc: 560.25,
                        p_cofins: 7.6,
                        v_cofins: 42.58,
                    },
                    icms_uf_dest: IcmsUfDest {
                        v_bcufdest: 0.0,
                        v_bcfcpufdest: 0.0,
                        p_fcpufdest: 0.0,
                        p_icmsufdest: 0.0,
                        p_icmsinter: 0.0,
                        p_icmsinter_part: 0.0,
                        v_fcpufdest: 0.0,
                        v_icmsufdest: 0.0,
                        v_icmsufremet: 0.0,
                    },
                },
            }],
        };
        assert_eq!(input, expected);
    }
    #[bench]
    fn nfe_vec_products_bench(b: &mut Bencher) {
        b.iter(|| Nfe::new("tests/data/1.xml"));
    }

    #[bench]
    fn nfe_single_products_bench(b: &mut Bencher) {
        b.iter(|| Nfe::new("tests/data/2.xml"));
    }
}
