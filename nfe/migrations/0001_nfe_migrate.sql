
-- tables
-- Table: users
CREATE TABLE users (
    iduser uuid  NOT NULL,
    firstname varchar(100) NULL,
    secondname varchar(100)  NULL,
    email varchar(200)  NULL,
    password varchar(100)  NULL,
    created_at date  NULL,
    CONSTRAINT Userspk PRIMARY KEY (iduser)
);

CREATE TABLE empresas (
    idempresa uuid  NOT NULL,
    nome varchar(100) NULL,
    nome_fant varchar(100)  NULL,
    cnpj varchar(200)  NULL,
    rua varchar(100)  NULL,
    numero varchar(100)  NULL,
    bairro varchar(100)  NULL,
    cidade varchar(100)  NULL,
    estado varchar(100)  NULL,
    cep varchar(100)  NULL,
    telefone varchar(100)  NULL,
    email varchar(200)  NULL,
    regime_tributario varchar(100)  NULL,
    created_at date  NULL,
    CONSTRAINT emoresaspk PRIMARY KEY (idempresa)
);

-- Table: Permissions
CREATE TABLE permissions (
    permissions_idpermission uuid  NOT NULL,
    permissions_user_id uuid  NULL,
    permissions_empresa_id uuid  NULL,
    permissions_allowed boolean  NULL,
    permissions_created_at date  NULL,
    CONSTRAINT Permissionspk PRIMARY KEY (permissions_idpermission)
);

-- Table: cofins
CREATE TABLE nfe_cofins (
    cofins_idcofins serial  NOT NULL,
    cofins_cst varchar(50) NULL,
    cofins_vbc decimal(12,2)  NULL,
    cofins_pcofins decimal(12,2)  NULL,
    cofins_vcofins decimal(12,2)  NULL,
    cofins_idproduto int NOT NULL,
    CONSTRAINT cofinspk PRIMARY KEY (cofins_idcofins)
);

-- Table: dest
CREATE TABLE nfe_dest (
    dest_iddest serial  NOT NULL,
    dest_cnpjcpf varchar(50) NULL,
    dest_ie varchar(50) NULL,
    dest_email varchar(200)  NULL,
    dest_indiedest varchar(50) NULL,
    dest_xnome varchar(200)  NULL,
    dest_idender int NOT NULL,
    CONSTRAINT destpk PRIMARY KEY (dest_iddest)
);

-- Table: emit
CREATE TABLE nfe_emit (
    emit_idemit serial  NOT NULL,
    emit_cnpjcpf varchar(50) NULL,
    emit_crt varchar(50) NULL,
    emit_ie varchar(50) NULL,
    emit_iest varchar(50) NULL,
    emit_xfant varchar(200)  NULL,
    emit_xnome varchar(200)  NULL,
    emit_idender int NOT NULL,
    CONSTRAINT Eestpk PRIMARY KEY (emit_idemit)
);

-- Table: ender
CREATE TABLE nfe_ender (
    ender_idender serial NOT NULL,
    ender_cep varchar(50) NULL,
    ender_uf varchar(4)  NULL,
    ender_cmun varchar(50) NULL,
    ender_cpais varchar(50) NULL,
    ender_nro varchar(50) NULL,
    ender_xbairro varchar(400)  NULL,
    ender_xcpl varchar(100)  NULL,
    ender_xlgr varchar(400)  NULL,
    ender_xmun varchar(200)  NULL,
    CONSTRAINT enderpk PRIMARY KEY (ender_idender)
);

-- Table: icms
CREATE TABLE nfe_icms (
    icms_idicms serial  NOT NULL,
    icms_orig varchar(50) NULL,
    icms_cst varchar(50) NULL,
    icms_modbc varchar(50) NULL,
    icms_vbc decimal(12,2)  NULL,
    icms_picms decimal(12,2)  NULL,
    icms_vicms decimal(12,2)  NULL,
    icms_idproduto int NOT NULL,
    CONSTRAINT icmspk PRIMARY KEY (icms_idicms)
);

-- Table: icmsufdest
CREATE TABLE nfe_icmsufdest (
    icms_uf_idicmsufdest serial  NOT NULL,
    icms_uf_vbcufdest decimal(12,2)  NULL,
    icms_uf_vbcfcpufdest decimal(12,2)  NULL,
    icms_uf_pfcpufdest decimal(12,2)  NULL,
    icms_uf_picmsufdest decimal(12,2)  NULL,
    icms_uf_picmsinter decimal(12,2)  NULL,
    icms_uf_picmsinterpart decimal(12,2)  NULL,
    icms_uf_vfcpufdest decimal(12,2)  NULL,
    icms_uf_vicmsufdest decimal(12,2)  NULL,
    icms_uf_vicmsufremet decimal(12,2)  NULL,
    icms_uf_idproduto int NOT NULL,
    CONSTRAINT icmsufdestpk PRIMARY KEY (icms_uf_idicmsufdest)
);

-- Table: ipi
CREATE TABLE nfe_ipi (
    ipi_idipi serial  NOT NULL,
    ipi_cenq varchar(50) NULL,
    ipi_cst varchar(50) NULL,
    ipi_vbc decimal(12,2)  NULL,
    ipi_pipi varchar(50) NULL,
    ipi_vipi varchar(50) NULL,
    ipi_idproduto int NOT NULL,
    CONSTRAINT ipipk PRIMARY KEY (ipi_idipi)
);

-- Table: nfe
CREATE TABLE nfe (
    nfe_idnfe serial  NOT NULL,
    nfe_cdv varchar(50) NULL,
    nfe_cmunfg varchar(50) NULL,
    nfe_cnf varchar(50) NULL,
    nfe_cuf varchar(50) NULL,
    nfe_dhemi varchar(100)  NULL,
    nfe_dhsaient varchar(100)  NULL,
    nfe_finnfe varchar(50) NULL,
    nfe_nfe_iddest varchar(50) NULL,
    nfe_indfinal varchar(50) NULL,
    nfe_indintermed varchar(50) NULL,
    nfe_indpres varchar(50) NULL,
    nfe_modnfe varchar(50) NULL,
    nfe_nnf varchar(50) NOT NULL,
    nfe_natop varchar(100)  NULL,
    nfe_procemi varchar(50) NULL,
    nfe_serie varchar(50) NULL,
    nfe_tpamb varchar(50) NULL,
    nfe_tpemis varchar(50) NULL,
    nfe_tpimp varchar(50) NULL,
    nfe_tpnf varchar(50) NULL,
    nfe_verproc varchar(100)  NULL,
    nfe_nftotal varchar(100)  NULL,
    nfe_idemit int NOT NULL,
    nfe_iddest int NOT NULL,
    CONSTRAINT nfepk PRIMARY KEY (nfe_idnfe)
);

-- Table: pis
CREATE TABLE nfe_pis (
    pis_idpis serial  NOT NULL,
    pis_cst varchar(50) NULL,
    pis_vbc decimal(12,2)  NULL,
    pis_ppis decimal(12,2)  NULL,
    pis_vpis decimal(12,2)  NULL,
    pis_idproduto int NOT NULL,
    CONSTRAINT pispk PRIMARY KEY (pis_idpis)
);

-- Table: produto
CREATE TABLE nfe_produto (
    produto_idproduto serial  NOT NULL,
    produto_nitem varchar(50) NULL,
    produto_cprod varchar(200)  NULL,
    produto_cean varchar(200)  NULL,
    produto_xprod varchar(400)  NULL,
    produto_ncm varchar(50) NULL,
    produto_cfop varchar(50) NULL,
    produto_ucom varchar(200)  NULL,
    produto_qcom decimal(12,2)  NULL,
    produto_vuncom decimal(12,2)  NULL,
    produto_vprod decimal(12,2)  NULL,
    produto_ceantrib varchar(200)  NULL,
    produto_utrib varchar(200)  NULL,
    produto_qtrib decimal(12,2)  NULL,
    produto_vuntrib decimal(12,2)  NULL,
    produto_indtot varchar(50) NULL,
    produto_xped varchar(200)  NULL,
    produto_idnfe int  NULL,
    CONSTRAINT produtopk PRIMARY KEY (produto_idproduto)
);
-- foreign keys
-- Reference: cofinsproduto (table: cofins)
ALTER TABLE nfe_cofins ADD CONSTRAINT cofinsproduto
    FOREIGN KEY (cofins_idproduto)
    REFERENCES nfe_produto (produto_idproduto)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: destender (table: dest)
ALTER TABLE nfe_dest ADD CONSTRAINT destender
    FOREIGN KEY (dest_idender)
    REFERENCES nfe_ender (ender_idender)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: emitender (table: emit)
ALTER TABLE nfe_emit ADD CONSTRAINT emitender
    FOREIGN KEY (emit_idender)
    REFERENCES nfe_ender (ender_idender)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: permissions_user_id (table: permissions)
ALTER TABLE permissions ADD CONSTRAINT permissions_user_id
    FOREIGN KEY (permissions_user_id)
    REFERENCES users (iduser)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: permissions_empresa_id (table: permissions)
ALTER TABLE permissions ADD CONSTRAINT permissions_empresa_id
    FOREIGN KEY (permissions_empresa_id)
    REFERENCES empresas (idempresa) 
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: icmsufdestproduto (table: icmsufdest)
ALTER TABLE nfe_icmsufdest ADD CONSTRAINT icmsufdestproduto
    FOREIGN KEY (icms_uf_idproduto)
    REFERENCES nfe_produto (produto_idproduto)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: icmsproduto (table: icms)
ALTER TABLE nfe_icms ADD CONSTRAINT icmsproduto
    FOREIGN KEY (icms_idproduto)
    REFERENCES nfe_produto (produto_idproduto)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: ipiproduto (table: ipi)
ALTER TABLE nfe_ipi ADD CONSTRAINT ipiproduto
    FOREIGN KEY (ipi_idproduto)
    REFERENCES nfe_produto (produto_idproduto)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: nfedest (table: nfe)
ALTER TABLE nfe ADD CONSTRAINT nfedest
    FOREIGN KEY (nfe_iddest)
    REFERENCES nfe_dest (dest_iddest)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: nfeemit (table: nfe)
ALTER TABLE nfe ADD CONSTRAINT nfeemit
    FOREIGN KEY (nfe_idemit)
    REFERENCES nfe_emit (emit_idemit)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: pisproduto (table: pis)
ALTER TABLE nfe_pis ADD CONSTRAINT pisproduto
    FOREIGN KEY (pis_idproduto)
    REFERENCES nfe_produto (produto_idproduto)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;

-- Reference: produtonfe (table: produto)
ALTER TABLE nfe_produto ADD CONSTRAINT produtonfe
    FOREIGN KEY (produto_idnfe)
    REFERENCES nfe (nfe_idnfe)  
    ON DELETE CASCADE
    ON UPDATE CASCADE
;
-- End of file.