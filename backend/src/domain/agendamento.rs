use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{NaiveDateTime, NaiveTime};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "varchar", rename_all = "PascalCase")]
pub enum ModalidadeConsulta {
    Presencial,
    Online,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "varchar", rename_all = "PascalCase")]
pub enum StatusConsulta {
    Agendada,
    Cancelada,
    Realizada,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Consulta {
    pub id: i32,
    pub fk_paciente_id: i32,
    pub fk_psicologo_id: i32,
    pub data_hora: NaiveDateTime,
    pub modalidade: ModalidadeConsulta,
    pub status: StatusConsulta,
    pub link_reuniao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct HorarioDisponivel {
    pub id: i32,
    pub fk_psicologo_id: i32,
    pub dia_semana: i32,
    pub hora_inicio: NaiveTime,
    pub hora_fim: NaiveTime,
}
