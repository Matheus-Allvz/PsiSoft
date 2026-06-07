-- Create Enums for Postgres
CREATE TYPE perfil_usuario AS ENUM ('Psicologo', 'Secretaria', 'Contador');
CREATE TYPE genero_paciente AS ENUM ('Masculino', 'Feminino', 'Não-binário', 'Prefiro não informar');
CREATE TYPE modalidade_consulta AS ENUM ('Presencial', 'Online');
CREATE TYPE presenca_consulta AS ENUM ('Agendada', 'Realizada', 'Falta', 'Cancelada');

CREATE TABLE Usuario (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    login VARCHAR(50) UNIQUE NOT NULL,
    senha_hash VARCHAR(255) NOT NULL,
    perfil perfil_usuario NOT NULL,
    email VARCHAR(100),
    telefone VARCHAR(15),
    status BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE Responsavel (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    cpf VARCHAR(11) UNIQUE NOT NULL,
    parentesco VARCHAR(20),
    contato_wpp VARCHAR(15)
);

CREATE TABLE Prontuario (
    id SERIAL PRIMARY KEY,
    data_abertura DATE NOT NULL DEFAULT CURRENT_DATE,
    status BOOLEAN DEFAULT TRUE
);

CREATE TABLE Transcricao(
    id SERIAL PRIMARY KEY,
    audio_path VARCHAR(255),
    texto_ia TEXT
);

CREATE TABLE Doc_Fiscal (
    id SERIAL PRIMARY KEY,
    tipo VARCHAR(20),
    data_emissao DATE,
    arq_pdf VARCHAR(255)
);

CREATE TABLE Psicologo (
    crp VARCHAR(20) NOT NULL,
    validador_epsi VARCHAR(50),
    fk_usuario_id INT PRIMARY KEY,
    FOREIGN KEY (fk_usuario_id) REFERENCES Usuario(id) ON DELETE CASCADE
);

CREATE TABLE Secretaria (
    fk_usuario_id INT PRIMARY KEY,
    FOREIGN KEY (fk_usuario_id) REFERENCES Usuario(id) ON DELETE CASCADE
);

CREATE TABLE Contador (
    fk_usuario_id INT PRIMARY KEY,
    FOREIGN KEY (fk_usuario_id) REFERENCES Usuario(id) ON DELETE CASCADE
);

CREATE TABLE Paciente (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    email VARCHAR(100),
    senha_hash VARCHAR(255),
    pin_biometria VARCHAR(255),
    cpf VARCHAR(11) UNIQUE NOT NULL,
    data_nasc DATE NOT NULL,
    status_menor BOOLEAN,
    genero genero_paciente,
    endereco VARCHAR(255),
    telefone VARCHAR(15) NOT NULL,
    contato_wpp BOOLEAN DEFAULT TRUE,
    fk_responsavel_id INT,
    fk_prontuario_id INT,
    FOREIGN KEY (fk_responsavel_id) REFERENCES Responsavel(id),
    FOREIGN KEY (fk_prontuario_id) REFERENCES Prontuario(id)
);

CREATE TABLE Contrato_LGPD (
    id SERIAL PRIMARY KEY,
    data_aceite TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ip_registro VARCHAR(45) NOT NULL,
    termo_tcle TEXT,
    fk_paciente_id INT,
    FOREIGN KEY (fk_paciente_id) REFERENCES Paciente(id)
);

CREATE TABLE Conteudo (
    id SERIAL PRIMARY KEY,
    titulo VARCHAR(100),
    arq_midia VARCHAR(255),
    perfil_interface VARCHAR(50),
    fk_psicologo_id INT,
    FOREIGN KEY (fk_psicologo_id) REFERENCES Psicologo(fk_usuario_id)
);

CREATE TABLE Acessa (
    fk_paciente_id INT,
    fk_conteudo_id INT,
    status_liberacao BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (fk_paciente_id, fk_conteudo_id),
    FOREIGN KEY (fk_paciente_id) REFERENCES Paciente(id),
    FOREIGN KEY (fk_conteudo_id) REFERENCES Conteudo(id)
);

CREATE TABLE Atv_Terap (
    id SERIAL PRIMARY KEY,
    categoria VARCHAR(50),
    enunciado TEXT,
    resposta_pac TEXT,
    feedback_psi TEXT,
    fk_psicologo_id INT,
    fk_paciente_id INT,
    FOREIGN KEY (fk_psicologo_id) REFERENCES Psicologo(fk_usuario_id),
    FOREIGN KEY (fk_paciente_id) REFERENCES Paciente(id)
);

CREATE TABLE Reg_Sessao (
    id SERIAL PRIMARY KEY,
    identificacao_sessao VARCHAR(50),
    queixa TEXT,
    evolucao TEXT,
    obs_clinicas TEXT,
    fk_prontuario_id INT,
    fk_transcricao_id INT,
    FOREIGN KEY (fk_prontuario_id) REFERENCES Prontuario(id),
    FOREIGN KEY (fk_transcricao_id) REFERENCES Transcricao(id)
);

CREATE TABLE Pagamento (
    id SERIAL PRIMARY KEY,
    valor DECIMAL(10,2) NOT NULL,
    metodo VARCHAR(30),
    quitado BOOLEAN DEFAULT FALSE,
    fk_doc_fiscal_id INT,
    FOREIGN KEY (fk_doc_fiscal_id) REFERENCES Doc_Fiscal(id)
);

CREATE TABLE Pacote_Sessao (
    id SERIAL PRIMARY KEY,
    valor_total DECIMAL(10,2),
    total_sessoes INT,
    sessoes_utilizadas INT DEFAULT 0,
    data_validade DATE,
    fk_paciente_id INT,
    FOREIGN KEY (fk_paciente_id) REFERENCES Paciente(id)
);

CREATE TABLE Consulta (
    id SERIAL PRIMARY KEY,
    data_hora TIMESTAMP NOT NULL,
    presenca presenca_consulta DEFAULT 'Agendada',
    modalidade modalidade_consulta NOT NULL,
    fk_paciente_id INT,
    fk_psicologo_id INT,
    fk_pacote_Sessao_id INT,
    fk_pagamento_id INT,
    FOREIGN KEY (fk_paciente_id) REFERENCES Paciente(id),
    FOREIGN KEY (fk_psicologo_id) REFERENCES Psicologo(fk_usuario_id),
    FOREIGN KEY (fk_pacote_Sessao_id) REFERENCES Pacote_Sessao(id),
    FOREIGN KEY (fk_pagamento_id) REFERENCES Pagamento(id)
);

CREATE TABLE Notificacao (
    id SERIAL PRIMARY KEY,
    conteudo_msg TEXT,
    data_hora_disparo TIMESTAMP,
    acao_resposta VARCHAR(50),
    fk_consulta_id INT,
    FOREIGN KEY (fk_consulta_id) REFERENCES Consulta(id)
);
