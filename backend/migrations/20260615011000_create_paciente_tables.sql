ALTER TYPE perfil_usuario ADD VALUE IF NOT EXISTS 'paciente';

DROP TABLE IF EXISTS Acessa CASCADE;
DROP TABLE IF EXISTS Atv_Terap CASCADE;
DROP TABLE IF EXISTS Contrato_LGPD CASCADE;
DROP TABLE IF EXISTS Pacote_Sessao CASCADE;
DROP TABLE IF EXISTS Paciente CASCADE;
DROP TABLE IF EXISTS Responsavel CASCADE;
DROP TABLE IF EXISTS ResponsavelLegal CASCADE;
DROP TABLE IF EXISTS ConfiguracaoPaciente CASCADE;
DROP TABLE IF EXISTS ConsentimentoPaciente CASCADE;
DROP TYPE IF EXISTS canal_notificacao CASCADE;
DROP TYPE IF EXISTS modalidade_pref CASCADE;

CREATE TABLE Paciente (
    id SERIAL PRIMARY KEY,
    fk_usuario_id INT NOT NULL,
    fk_psicologo_id INT NOT NULL,
    nome VARCHAR(100) NOT NULL,
    cpf VARCHAR(11) UNIQUE NOT NULL,
    data_nascimento DATE NOT NULL,
    genero VARCHAR(50),
    estado_civil VARCHAR(50),
    telefone VARCHAR(15),
    telefone_alt VARCHAR(15),
    endereco TEXT,
    criado_em TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (fk_usuario_id) REFERENCES Usuario(id) ON DELETE CASCADE,
    FOREIGN KEY (fk_psicologo_id) REFERENCES Psicologo(fk_usuario_id) ON DELETE CASCADE
);

CREATE TABLE ResponsavelLegal (
    id SERIAL PRIMARY KEY,
    fk_paciente_id INT NOT NULL,
    nome VARCHAR(100) NOT NULL,
    cpf VARCHAR(11),
    parentesco VARCHAR(50),
    email VARCHAR(100),
    telefone VARCHAR(15),
    FOREIGN KEY (fk_paciente_id) REFERENCES Paciente(id) ON DELETE CASCADE
);

CREATE TYPE canal_notificacao AS ENUM ('Email', 'WhatsApp', 'SMS');
CREATE TYPE modalidade_pref AS ENUM ('Online', 'Presencial', 'Hibrido');

CREATE TABLE ConfiguracaoPaciente (
    id SERIAL PRIMARY KEY,
    fk_paciente_id INT NOT NULL UNIQUE,
    canal canal_notificacao,
    modalidade modalidade_pref,
    FOREIGN KEY (fk_paciente_id) REFERENCES Paciente(id) ON DELETE CASCADE
);

CREATE TABLE ConsentimentoPaciente (
    id SERIAL PRIMARY KEY,
    fk_paciente_id INT NOT NULL UNIQUE,
    tcle_aceito BOOLEAN DEFAULT FALSE,
    transcricao_aceita BOOLEAN DEFAULT FALSE,
    chatbot_aceito BOOLEAN DEFAULT FALSE,
    metodo_assinatura VARCHAR(50),
    data_hora_aceite TIMESTAMPTZ,
    ip_assinante VARCHAR(45),
    FOREIGN KEY (fk_paciente_id) REFERENCES Paciente(id) ON DELETE CASCADE
);
