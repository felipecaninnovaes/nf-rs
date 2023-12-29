-- Created by Vertabelo (http://vertabelo.com)
-- Last modification date: 2023-12-19 17:00:00.113

-- CREATE TABLE user (
--     iduser serial  NOT NULL,
--     fistname varchar(100) NULL,
--     secondname varchar(100)  NULL,
--     email varchar(200)  NULL,
--     userpassword varchar(100)  NULL,
--     CONSTRAINT Cofinspk PRIMARY KEY (iduser)
-- );

-- tables
-- Table: Cofins
CREATE TABLE nfe_Cofins (
    nfe_idCofins serial  NOT NULL,
    cst varchar(50) NULL,
    vbc decimal(12,2)  NULL,
    pcofins decimal(12,2)  NULL,
    vcofins decimal(12,2)  NULL,
    nfe_idProduto int NOT NULL,
    CONSTRAINT Cofinspk PRIMARY KEY (nfe_idCofins)
);

-- Table: Dest
CREATE TABLE nfe_Dest (
    nfe_idDest serial  NOT NULL,
    cnpjcpf varchar(50) NULL,
    ie varchar(50) NULL,
    email varchar(200)  NULL,
    indiedest varchar(50) NULL,
    xnome varchar(200)  NULL,
    nfe_idEnder int NOT NULL,
    CONSTRAINT Destpk PRIMARY KEY (nfe_idDest)
);

-- Table: Emit
CREATE TABLE nfe_Emit (
    nfe_idEmit serial  NOT NULL,
    cnpjcpf varchar(50) NULL,
    crt varchar(50) NULL,
    ie varchar(50) NULL,
    iest varchar(50) NULL,
    xfant varchar(200)  NULL,
    xnome varchar(200)  NULL,
    nfe_idEnder int NOT NULL,
    CONSTRAINT Eestpk PRIMARY KEY (nfe_idEmit)
);

-- Table: Ender
CREATE TABLE nfe_Ender (
    nfe_idEnder serial NOT NULL,
    cep varchar(50) NULL,
    uf varchar(4)  NULL,
    cmun varchar(50) NULL,
    cpais varchar(50) NULL,
    nro varchar(50) NULL,
    xbairro varchar(400)  NULL,
    xcpl varchar(100)  NULL,
    xlgr varchar(400)  NULL,
    xmun varchar(200)  NULL,
    CONSTRAINT Enderpk PRIMARY KEY (nfe_idEnder)
);

-- Table: Icms
CREATE TABLE nfe_Icms (
    nfe_idIcms serial  NOT NULL,
    orig varchar(50) NULL,
    cst varchar(50) NULL,
    modbc varchar(50) NULL,
    vbc decimal(12,2)  NULL,
    picms decimal(12,2)  NULL,
    vicms decimal(12,2)  NULL,
    nfe_idProduto int NOT NULL,
    CONSTRAINT Icmspk PRIMARY KEY (nfe_idIcms)
);

-- Table: IcmsUfDest
CREATE TABLE nfe_IcmsUfDest (
    nfe_idIcmsufdest serial  NOT NULL,
    vbcufdest decimal(12,2)  NULL,
    vbcfcpufdest decimal(12,2)  NULL,
    pfcpufdest decimal(12,2)  NULL,
    picmsufdest decimal(12,2)  NULL,
    picmsinter decimal(12,2)  NULL,
    picmsinterpart decimal(12,2)  NULL,
    vfcpufdest decimal(12,2)  NULL,
    vicmsufdest decimal(12,2)  NULL,
    vicmsufremet decimal(12,2)  NULL,
    nfe_idProduto int NOT NULL,
    CONSTRAINT IcmsUfDestpk PRIMARY KEY (nfe_idIcmsufdest)
);

-- Table: Ipi
CREATE TABLE nfe_Ipi (
    nfe_idIpi serial  NOT NULL,
    cenq varchar(50) NULL,
    cst varchar(50) NULL,
    vbc decimal(12,2)  NULL,
    pipi varchar(50) NULL,
    vipi varchar(50) NULL,
    nfe_idProduto int NOT NULL,
    CONSTRAINT Ipipk PRIMARY KEY (nfe_idIpi)
);

-- Table: Nfe
CREATE TABLE Nfe (
    idNfe serial  NOT NULL,
    cdv varchar(50) NULL,
    cmunfg varchar(50) NULL,
    cnf varchar(50) NULL,
    cuf varchar(50) NULL,
    dhemi varchar(100)  NULL,
    dhsaient varchar(100)  NULL,
    finnfe varchar(50) NULL,
    iddest varchar(50) NULL,
    indfinal varchar(50) NULL,
    indintermed varchar(50) NULL,
    indpres varchar(50) NULL,
    modnfe varchar(50) NULL,
    nnf varchar(50) NOT NULL,
    natop varchar(100)  NULL,
    procemi varchar(50) NULL,
    serie varchar(50) NULL,
    tpamb varchar(50) NULL,
    tpemis varchar(50) NULL,
    tpimp varchar(50) NULL,
    tpnf varchar(50) NULL,
    verproc varchar(100)  NULL,
    nftotal varchar(100)  NULL,
    nfe_idEmit int NOT NULL,
    nfe_idDest int NOT NULL,
    CONSTRAINT Nfepk PRIMARY KEY (idNfe)
);

-- Table: Pis
CREATE TABLE nfe_Pis (
    nfe_idPis serial  NOT NULL,
    cst varchar(50) NULL,
    vbc decimal(12,2)  NULL,
    ppis decimal(12,2)  NULL,
    vpis decimal(12,2)  NULL,
    nfe_idProduto int NOT NULL,
    CONSTRAINT Pispk PRIMARY KEY (nfe_idPis)
);

-- Table: Produto
CREATE TABLE nfe_Produto (
    nfe_idProduto serial  NOT NULL,
    nitem varchar(50) NULL,
    cprod varchar(200)  NULL,
    cean varchar(200)  NULL,
    xprod varchar(400)  NULL,
    ncm varchar(50) NULL,
    cfop varchar(50) NULL,
    ucom varchar(200)  NULL,
    qcom decimal(12,2)  NULL,
    vuncom decimal(12,2)  NULL,
    vprod decimal(12,2)  NULL,
    ceantrib varchar(200)  NULL,
    utrib varchar(200)  NULL,
    qtrib decimal(12,2)  NULL,
    vuntrib decimal(12,2)  NULL,
    indtot varchar(50) NULL,
    xped varchar(200)  NULL,
    idNfe int  NULL,
    CONSTRAINT Produtopk PRIMARY KEY (nfe_idProduto)
);

-- foreign keys
-- Reference: CofinsProduto (table: Cofins)
ALTER TABLE nfe_Cofins ADD CONSTRAINT CofinsProduto
    FOREIGN KEY (nfe_idProduto)
    REFERENCES nfe_Produto (nfe_idProduto)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: DestEnder (table: Dest)
ALTER TABLE nfe_Dest ADD CONSTRAINT DestEnder
    FOREIGN KEY (nfe_idEnder)
    REFERENCES nfe_Ender (nfe_idEnder)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: EmitEnder (table: Emit)
ALTER TABLE nfe_Emit ADD CONSTRAINT EmitEnder
    FOREIGN KEY (nfe_idEnder)
    REFERENCES nfe_Ender (nfe_idEnder)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: IcmsUfDestProduto (table: IcmsUfDest)
ALTER TABLE nfe_IcmsUfDest ADD CONSTRAINT IcmsUfDestProduto
    FOREIGN KEY (nfe_idProduto)
    REFERENCES nfe_Produto (nfe_idProduto)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: IcmsProduto (table: Icms)
ALTER TABLE nfe_Icms ADD CONSTRAINT IcmsProduto
    FOREIGN KEY (nfe_idProduto)
    REFERENCES nfe_Produto (nfe_idProduto)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: IpiProduto (table: Ipi)
ALTER TABLE nfe_Ipi ADD CONSTRAINT IpiProduto
    FOREIGN KEY (nfe_idProduto)
    REFERENCES nfe_Produto (nfe_idProduto)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: NfeDest (table: Nfe)
ALTER TABLE Nfe ADD CONSTRAINT NfeDest
    FOREIGN KEY (nfe_idDest)
    REFERENCES nfe_Dest (nfe_idDest)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: NfeEmit (table: Nfe)
ALTER TABLE Nfe ADD CONSTRAINT NfeEmit
    FOREIGN KEY (nfe_idEmit)
    REFERENCES nfe_Emit (nfe_idEmit)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: PisProduto (table: Pis)
ALTER TABLE nfe_Pis ADD CONSTRAINT PisProduto
    FOREIGN KEY (nfe_idProduto)
    REFERENCES nfe_Produto (nfe_idProduto)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: ProdutoNfe (table: Produto)
ALTER TABLE nfe_Produto ADD CONSTRAINT ProdutoNfe
    FOREIGN KEY (idNfe)
    REFERENCES Nfe (idNfe)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- End of file.