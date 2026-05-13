CREATE TABLE Usuario (
    id INT PRIMARY KEY AUTO_INCREMENT,
    nome VARCHAR(100) NOT NULL,
    login VARCHAR(50) UNIQUE NOT NULL,
    senha_hash VARCHAR(255) NOT NULL,
    perfil ENUM('Psicologo', 'Secretaria', 'Contador') NOT NULL,
    email VARCHAR(100),
    telefone VARCHAR(15),
    status BOOLEAN NOT NULL
);

CREATE TABLE Responsavel_Legal (
    id INT PRIMARY KEY AUTO_INCREMENT,
    nome VARCHAR(100) NOT NULL,
    cpf VARCHAR(14) UNIQUE NOT NULL,
    parentesco VARCHAR(50),
    contato_wpp VARCHAR(15)
);

CREATE TABLE Prontuario (
    id INT PRIMARY KEY AUTO_INCREMENT,
    data_abertura DATE NOT NULL
);

CREATE TABLE Transcricao_IA (
    id INT PRIMARY KEY AUTO_INCREMENT,
    audio_path VARCHAR(255),
    texto_ia TEXT
);

CREATE TABLE Doc_Fiscal (
    id INT PRIMARY KEY AUTO_INCREMENT,
    tipo VARCHAR(20),
    data_emissao DATE,
    arq_pdf VARCHAR(255)
);

CREATE TABLE Psicologo (
    crp VARCHAR(20) NOT NULL,
    validador_epsi VARCHAR(50),
    fk_usuario_id INT PRIMARY KEY,
    FOREIGN KEY (fk_usuario_id) REFERENCES Usuario(id)
);

CREATE TABLE Secretaria (
    fk_usuario_id INT PRIMARY KEY,
    FOREIGN KEY (fk_usuario_id) REFERENCES Usuario(id)
);

CREATE TABLE Contador (
    fk_usuario_id INT PRIMARY KEY,
    FOREIGN KEY (fk_usuario_id) REFERENCES Usuario(id)
);

CREATE TABLE Paciente (
    id INT PRIMARY KEY AUTO_INCREMENT,
    nome VARCHAR(100) NOT NULL,
    cpf VARCHAR(14) UNIQUE NOT NULL,
    data_nasc DATE,
    genero VARCHAR(20),
    endereco VARCHAR(255),
    telefone VARCHAR(15),
    contato_wpp VARCHAR(15),
    status_ativo BOOLEAN,
    fk_Responsavel_Legal_id INT,
    fk_Prontuario_id INT,
    FOREIGN KEY (fk_Responsavel_Legal_id) REFERENCES Responsavel_Legal(id),
    FOREIGN KEY (fk_Prontuario_id) REFERENCES Prontuario(id)
);

CREATE TABLE Conteudo (
    id INT PRIMARY KEY AUTO_INCREMENT,
    titulo VARCHAR(100),
    arq_midia VARCHAR(255),
    perfil_interface VARCHAR(50),
    fk_Psicologo_id INT,
    FOREIGN KEY (fk_Psicologo_id) REFERENCES Psicologo(fk_usuario_id)
);

CREATE TABLE Acessa (
    fk_Paciente_id INT,
    fk_Conteudo_id INT,
    status_liberacao BOOLEAN,
    PRIMARY KEY (fk_Paciente_id, fk_Conteudo_id),
    FOREIGN KEY (fk_Paciente_id) REFERENCES Paciente(id),
    FOREIGN KEY (fk_Conteudo_id) REFERENCES Conteudo(id)
);

CREATE TABLE Atv_Terap (
    id INT PRIMARY KEY AUTO_INCREMENT,
    categoria VARCHAR(50),
    enunciado TEXT,
    resposta_pac TEXT,
    feedback_psi TEXT,
    fk_Psicologo_id INT,
    fk_Paciente_id INT,
    FOREIGN KEY (fk_Psicologo_id) REFERENCES Psicologo(fk_usuario_id),
    FOREIGN KEY (fk_Paciente_id) REFERENCES Paciente(id)
);

CREATE TABLE Reg_Sessao (
    id INT PRIMARY KEY AUTO_INCREMENT,
    identificacao_sessao VARCHAR(50),
    data_registro DATE,
    queixa TEXT,
    evolucao TEXT,
    fk_Prontuario_id INT,
    fk_Transcricao_IA_id INT,
    FOREIGN KEY (fk_Prontuario_id) REFERENCES Prontuario(id),
    FOREIGN KEY (fk_Transcricao_IA_id) REFERENCES Transcricao_IA(id)
);

CREATE TABLE Pagamento (
    id INT PRIMARY KEY AUTO_INCREMENT,
    valor DECIMAL(10,2) NOT NULL,
    data_pagto DATE,
    forma_pagto VARCHAR(50),
    status_quitado BOOLEAN,
    fk_Doc_Fiscal_id INT,
    FOREIGN KEY (fk_Doc_Fiscal_id) REFERENCES Doc_Fiscal(id)
);

CREATE TABLE Pacote_Sessao (
    id INT PRIMARY KEY AUTO_INCREMENT,
    valor_total DECIMAL(10,2),
    total_sessoes INT,
    sessoes_utilizadas INT,
    data_validade DATE,
    fk_Paciente_id INT,
    FOREIGN KEY (fk_Paciente_id) REFERENCES Paciente(id)
);

CREATE TABLE Consulta (
    id INT PRIMARY KEY AUTO_INCREMENT,
    data_hora_sessao DATETIME NOT NULL,
    status_presenca VARCHAR(50),
    fk_Paciente_id INT,
    fk_Psicologo_id INT,
    fk_Pacote_Sessao_id INT,
    fk_Pagamento_id INT,
    FOREIGN KEY (fk_Paciente_id) REFERENCES Paciente(id),
    FOREIGN KEY (fk_Psicologo_id) REFERENCES Psicologo(fk_usuario_id),
    FOREIGN KEY (fk_Pacote_Sessao_id) REFERENCES Pacote_Sessao(id),
    FOREIGN KEY (fk_Pagamento_id) REFERENCES Pagamento(id)
);

CREATE TABLE Notificacao (
    id INT PRIMARY KEY AUTO_INCREMENT,
    conteudo_msg TEXT,
    data_hora_disparo DATETIME,
    acao_resposta VARCHAR(50),
    fk_Consulta_id INT,
    FOREIGN KEY (fk_Consulta_id) REFERENCES Consulta(id)
);