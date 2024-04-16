CREATE TABLE IF NOT EXISTS nfse_dados (
  id INT NOT NULL GENERATED ALWAYS AS IDENTITY,
  numero_da_nota VARCHAR(45) NOT NULL,
  competencia DATE NULL,
  codigo_do_municipio VARCHAR(45) NULL,
  exigibilidade_iss VARCHAR(45) NULL,
  iss_retido VARCHAR(45) NULL,
  item_lista_servico VARCHAR(45) NULL,
  municipio_incidencia VARCHAR(45) NULL,
  responsavel_retencao VARCHAR(45) NULL,
  PRIMARY KEY (id))
;

CREATE TABLE IF NOT EXISTS nfse_endereco (
  id INT NOT NULL GENERATED ALWAYS AS IDENTITY,
  logradouro VARCHAR(45) NULL,
  bairro VARCHAR(45) NULL,
  numero VARCHAR(45) NULL,
  complemento VARCHAR(45) NULL,
  cep VARCHAR(45) NULL,
  uf VARCHAR(45) NULL,
  codigo_municipio VARCHAR(45) NULL,
  codigo_pais VARCHAR(45) NULL,
  PRIMARY KEY (id))
;

CREATE TABLE IF NOT EXISTS nfse_prestador (
  id INT NOT NULL GENERATED ALWAYS AS IDENTITY,
  cnpj VARCHAR(45) NOT NULL,
  inscricao_municipal VARCHAR(45) NOT NULL,
  nome_fantasia VARCHAR(45) NULL,
  razao_social VARCHAR(45) NULL,
  email VARCHAR(45) NULL,
  telefone VARCHAR(45) NULL,
  endereco_id INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT fk_prestador_endereco1
    FOREIGN KEY (endereco_id)
    REFERENCES nfse_endereco (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX fk_prestador_endereco1_idx ON nfse_prestador (endereco_id ASC);

CREATE TABLE IF NOT EXISTS nfse_valores (
  id INT NOT NULL,
  aliquota VARCHAR(45) NULL,
  base_calculo VARCHAR(45) NULL,
  desconto_incondicionado VARCHAR(45) NULL,
  desconto_condicionado VARCHAR(45) NULL,
  outras_retencoes VARCHAR(45) NULL,
  valor_cofins VARCHAR(45) NULL,
  valor_csll VARCHAR(45) NULL,
  valor_deducoes VARCHAR(45) NULL,
  valor_inss VARCHAR(45) NULL,
  valor_ir VARCHAR(45) NULL,
  valor_iss VARCHAR(45) NULL,
  valor_pis VARCHAR(45) NULL,
  valor_servicos VARCHAR(45) NULL,
  PRIMARY KEY (id))
;

CREATE TABLE IF NOT EXISTS nfse_tomador (
  id INT NOT NULL,
  cnpj VARCHAR(45) NULL,
  cpf VARCHAR(45) NULL,
  razao_social VARCHAR(45) NULL,
  inscricao_municipal VARCHAR(45) NULL,
  email VARCHAR(45) NULL,
  telefone VARCHAR(45) NULL,
  endereco_id INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT fk_tomador_endereco1
    FOREIGN KEY (endereco_id)
    REFERENCES nfse_endereco (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX fk_tomador_endereco1_idx ON nfse_tomador (endereco_id ASC);

CREATE TABLE IF NOT EXISTS nfse (
  id INT NOT NULL,
  dados_nfse_id INT NOT NULL,
  prestador_id INT NOT NULL,
  tomador_id INT NOT NULL,
  valores_id INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT fk_nfse_dados_nfse
    FOREIGN KEY (dados_nfse_id)
    REFERENCES nfse_dados (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT fk_nfse_prestador1
    FOREIGN KEY (prestador_id)
    REFERENCES nfse_prestador (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT fk_nfse_tomador1
    FOREIGN KEY (tomador_id)
    REFERENCES nfse_tomador (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT fk_nfse_valores1
    FOREIGN KEY (valores_id)
    REFERENCES nfse_valores (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX fk_nfse_dados_nfse_idx ON nfse (dados_nfse_id ASC);

CREATE INDEX fk_nfse_prestador1_idx ON nfse (prestador_id ASC);

CREATE INDEX fk_nfse_tomador1_idx ON nfse (tomador_id ASC);

CREATE INDEX fk_nfse_valores1_idx ON nfse (valores_id ASC);

CREATE TABLE IF NOT EXISTS users (
  id UUID NOT NULL,
  firstname VARCHAR(100) NULL DEFAULT NULL,
  secondname VARCHAR(100) NULL DEFAULT NULL,
  email VARCHAR(200) NULL DEFAULT NULL,
  password VARCHAR(100) NULL DEFAULT NULL,
  created_at DATE NULL DEFAULT NULL,
  PRIMARY KEY (id))
;

CREATE TABLE IF NOT EXISTS empresas (
  idempresa UUID NOT NULL,
  nome VARCHAR(100) NULL DEFAULT NULL,
  nome_fant VARCHAR(100) NULL DEFAULT NULL,
  cnpj VARCHAR(200) NULL DEFAULT NULL,
  rua VARCHAR(100) NULL DEFAULT NULL,
  numero VARCHAR(100) NULL DEFAULT NULL,
  bairro VARCHAR(100) NULL DEFAULT NULL,
  cidade VARCHAR(100) NULL DEFAULT NULL,
  estado VARCHAR(100) NULL DEFAULT NULL,
  cep VARCHAR(100) NULL DEFAULT NULL,
  telefone VARCHAR(100) NULL DEFAULT NULL,
  email VARCHAR(200) NULL DEFAULT NULL,
  regime_tributario VARCHAR(100) NULL DEFAULT NULL,
  created_at DATE NULL DEFAULT NULL,
  PRIMARY KEY (idempresa))
;

CREATE TABLE IF NOT EXISTS permissions (
  id UUID NOT NULL,
  permissions_user_id UUID NULL DEFAULT NULL,
  permissions_empresa_id UUID NULL DEFAULT NULL,
  permissions_allowed BOOLEAN NULL DEFAULT NULL,
  permissions_created_at DATE NULL DEFAULT NULL,
  PRIMARY KEY (id),
  CONSTRAINT permissions_user_id
    FOREIGN KEY (permissions_user_id)
    REFERENCES users (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT permissions_empresa_id
    FOREIGN KEY (permissions_empresa_id)
    REFERENCES empresas (idempresa)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX permissions_user_id ON permissions (permissions_user_id ASC);

CREATE INDEX permissions_empresa_id ON permissions (permissions_empresa_id ASC);

CREATE TABLE IF NOT EXISTS nfe_ender (
  id INT NOT NULL GENERATED ALWAYS AS IDENTITY,
  ender_cep VARCHAR(50) NULL DEFAULT NULL,
  ender_uf VARCHAR(4) NULL DEFAULT NULL,
  ender_cmun VARCHAR(50) NULL DEFAULT NULL,
  ender_cpais VARCHAR(50) NULL DEFAULT NULL,
  ender_nro VARCHAR(50) NULL DEFAULT NULL,
  ender_xbairro VARCHAR(400) NULL DEFAULT NULL,
  ender_xcpl VARCHAR(100) NULL DEFAULT NULL,
  ender_xlgr VARCHAR(400) NULL DEFAULT NULL,
  ender_xmun VARCHAR(200) NULL DEFAULT NULL,
  PRIMARY KEY (id))
;

CREATE TABLE IF NOT EXISTS nfe_dest (
  id INT NOT NULL GENERATED ALWAYS AS IDENTITY,
  dest_cnpjcpf VARCHAR(50) NULL DEFAULT NULL,
  dest_ie VARCHAR(50) NULL DEFAULT NULL,
  dest_email VARCHAR(200) NULL DEFAULT NULL,
  dest_indiedest VARCHAR(50) NULL DEFAULT NULL,
  dest_xnome VARCHAR(200) NULL DEFAULT NULL,
  dest_idender INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT destender
    FOREIGN KEY (dest_idender)
    REFERENCES nfe_ender (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX destender ON nfe_dest (dest_idender ASC);

CREATE TABLE IF NOT EXISTS nfe_emit (
  id INT NOT NULL GENERATED ALWAYS AS IDENTITY,
  emit_cnpjcpf VARCHAR(50) NULL DEFAULT NULL,
  emit_crt VARCHAR(50) NULL DEFAULT NULL,
  emit_ie VARCHAR(50) NULL DEFAULT NULL,
  emit_iest VARCHAR(50) NULL DEFAULT NULL,
  emit_xfant VARCHAR(200) NULL DEFAULT NULL,
  emit_xnome VARCHAR(200) NULL DEFAULT NULL,
  emit_idender INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT emitender
    FOREIGN KEY (emit_idender)
    REFERENCES nfe_ender (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX emitender ON nfe_emit (emit_idender ASC);

CREATE TABLE IF NOT EXISTS nfe (
  id INT NOT NULL GENERATED ALWAYS AS IDENTITY,
  nfe_cdv VARCHAR(50) NULL DEFAULT NULL,
  nfe_cmunfg VARCHAR(50) NULL DEFAULT NULL,
  nfe_cnf VARCHAR(50) NULL DEFAULT NULL,
  nfe_cuf VARCHAR(50) NULL DEFAULT NULL,
  nfe_dhemi VARCHAR(100) NULL DEFAULT NULL,
  nfe_dhsaient VARCHAR(100) NULL DEFAULT NULL,
  nfe_finnfe VARCHAR(50) NULL DEFAULT NULL,
  nfe_nfe_iddest VARCHAR(50) NULL DEFAULT NULL,
  nfe_indfinal VARCHAR(50) NULL DEFAULT NULL,
  nfe_indintermed VARCHAR(50) NULL DEFAULT NULL,
  nfe_indpres VARCHAR(50) NULL DEFAULT NULL,
  nfe_modnfe VARCHAR(50) NULL DEFAULT NULL,
  nfe_nnf VARCHAR(50) NOT NULL,
  nfe_natop VARCHAR(100) NULL DEFAULT NULL,
  nfe_procemi VARCHAR(50) NULL DEFAULT NULL,
  nfe_serie VARCHAR(50) NULL DEFAULT NULL,
  nfe_tpamb VARCHAR(50) NULL DEFAULT NULL,
  nfe_tpemis VARCHAR(50) NULL DEFAULT NULL,
  nfe_tpimp VARCHAR(50) NULL DEFAULT NULL,
  nfe_tpnf VARCHAR(50) NULL DEFAULT NULL,
  nfe_verproc VARCHAR(100) NULL DEFAULT NULL,
  nfe_nftotal VARCHAR(100) NULL DEFAULT NULL,
  nfe_idemit INT NOT NULL,
  nfe_iddest INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT nfedest
    FOREIGN KEY (nfe_iddest)
    REFERENCES nfe_dest (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT nfeemit
    FOREIGN KEY (nfe_idemit)
    REFERENCES nfe_emit (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX nfedest ON nfe (nfe_iddest ASC);

CREATE INDEX nfeemit ON nfe (nfe_idemit ASC);

CREATE TABLE IF NOT EXISTS nfe_produto (
  id INT NOT NULL GENERATED ALWAYS AS IDENTITY,
  produto_nitem VARCHAR(50) NULL DEFAULT NULL,
  produto_cprod VARCHAR(200) NULL DEFAULT NULL,
  produto_cean VARCHAR(200) NULL DEFAULT NULL,
  produto_xprod VARCHAR(400) NULL DEFAULT NULL,
  produto_ncm VARCHAR(50) NULL DEFAULT NULL,
  produto_cfop VARCHAR(50) NULL DEFAULT NULL,
  produto_ucom VARCHAR(200) NULL DEFAULT NULL,
  produto_qcom DECIMAL(12,2) NULL DEFAULT NULL,
  produto_vuncom DECIMAL(12,2) NULL DEFAULT NULL,
  produto_vprod DECIMAL(12,2) NULL DEFAULT NULL,
  produto_ceantrib VARCHAR(200) NULL DEFAULT NULL,
  produto_utrib VARCHAR(200) NULL DEFAULT NULL,
  produto_qtrib DECIMAL(12,2) NULL DEFAULT NULL,
  produto_vuntrib DECIMAL(12,2) NULL DEFAULT NULL,
  produto_indtot VARCHAR(50) NULL DEFAULT NULL,
  produto_xped VARCHAR(200) NULL DEFAULT NULL,
  produto_idnfe INT NULL DEFAULT NULL,
  PRIMARY KEY (id),
  CONSTRAINT produtonfe
    FOREIGN KEY (produto_idnfe)
    REFERENCES nfe (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX produtonfe ON nfe_produto (produto_idnfe ASC);

CREATE TABLE IF NOT EXISTS nfe_cofins (
  id INT NOT NULL,
  cofins_cst VARCHAR(50) NULL DEFAULT NULL,
  cofins_vbc DECIMAL(12,2) NULL DEFAULT NULL,
  cofins_pcofins DECIMAL(12,2) NULL DEFAULT NULL,
  cofins_vcofins DECIMAL(12,2) NULL DEFAULT NULL,
  cofins_idproduto INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT cofinsproduto
    FOREIGN KEY (cofins_idproduto)
    REFERENCES nfe_produto (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX cofinsproduto ON nfe_cofins (cofins_idproduto ASC);

CREATE TABLE IF NOT EXISTS nfe_icms (
  id INT NOT NULL,
  icms_orig VARCHAR(50) NULL DEFAULT NULL,
  icms_cst VARCHAR(50) NULL DEFAULT NULL,
  icms_modbc VARCHAR(50) NULL DEFAULT NULL,
  icms_vbc DECIMAL(12,2) NULL DEFAULT NULL,
  icms_picms DECIMAL(12,2) NULL DEFAULT NULL,
  icms_vicms DECIMAL(12,2) NULL DEFAULT NULL,
  icms_idproduto INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT icmsproduto
    FOREIGN KEY (icms_idproduto)
    REFERENCES nfe_produto (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX icmsproduto ON nfe_icms (icms_idproduto ASC);

CREATE TABLE IF NOT EXISTS nfe_icmsufdest (
  id INT NOT NULL,
  icms_uf_vbcufdest DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_vbcfcpufdest DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_pfcpufdest DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_picmsufdest DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_picmsinter DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_picmsinterpart DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_vfcpufdest DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_vicmsufdest DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_vicmsufremet DECIMAL(12,2) NULL DEFAULT NULL,
  icms_uf_idproduto INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT icmsufdestproduto
    FOREIGN KEY (icms_uf_idproduto)
    REFERENCES nfe_produto (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX icmsufdestproduto ON nfe_icmsufdest (icms_uf_idproduto ASC);

CREATE TABLE IF NOT EXISTS nfe_ipi (
  id INT NOT NULL,
  ipi_cenq VARCHAR(50) NULL DEFAULT NULL,
  ipi_cst VARCHAR(50) NULL DEFAULT NULL,
  ipi_vbc DECIMAL(12,2) NULL DEFAULT NULL,
  ipi_pipi VARCHAR(50) NULL DEFAULT NULL,
  ipi_vipi VARCHAR(50) NULL DEFAULT NULL,
  ipi_idproduto INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT ipiproduto
    FOREIGN KEY (ipi_idproduto)
    REFERENCES nfe_produto (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX ipiproduto ON nfe_ipi (ipi_idproduto ASC);

CREATE TABLE IF NOT EXISTS nfe_pis (
  id INT NOT NULL,
  pis_cst VARCHAR(50) NULL DEFAULT NULL,
  pis_vbc DECIMAL(12,2) NULL DEFAULT NULL,
  pis_ppis DECIMAL(12,2) NULL DEFAULT NULL,
  pis_vpis DECIMAL(12,2) NULL DEFAULT NULL,
  pis_idproduto INT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT pisproduto
    FOREIGN KEY (pis_idproduto)
    REFERENCES nfe_produto (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
;

CREATE INDEX pisproduto ON nfe_pis (pis_idproduto ASC);
