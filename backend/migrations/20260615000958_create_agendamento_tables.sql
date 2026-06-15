DROP TYPE IF EXISTS modalidade_consulta CASCADE;
DROP TYPE IF EXISTS status_consulta CASCADE;
DROP TABLE IF EXISTS Consulta CASCADE;
DROP TABLE IF EXISTS HorarioDisponivel CASCADE;

CREATE TYPE modalidade_consulta AS ENUM ('Presencial', 'Online');
CREATE TYPE status_consulta AS ENUM ('Agendada', 'Cancelada', 'Realizada');

CREATE TABLE Consulta (
    id SERIAL PRIMARY KEY,
    fk_paciente_id INT NOT NULL,
    fk_psicologo_id INT NOT NULL,
    data_hora TIMESTAMP NOT NULL,
    modalidade VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL,
    link_reuniao VARCHAR(255)
);

CREATE TABLE HorarioDisponivel (
    id SERIAL PRIMARY KEY,
    fk_psicologo_id INT NOT NULL,
    dia_semana INT NOT NULL,
    hora_inicio TIME NOT NULL,
    hora_fim TIME NOT NULL
);
