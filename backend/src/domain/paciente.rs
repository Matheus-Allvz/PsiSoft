#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Paciente {
    pub id: i32,
    pub fk_usuario_id: i32,
    pub fk_psicologo_id: i32,
    pub nome: String,
    pub cpf: String,
    pub data_nascimento: NaiveDate,
    pub genero: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub telefone_alt: Option<String>,
    pub endereco: Option<String>,
    pub criado_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ResponsavelLegal {
    pub id: i32,
    pub fk_paciente_id: i32,
    pub nome: String,
    pub cpf: Option<String>,
    pub parentesco: Option<String>,
    pub email: Option<String>,
    pub telefone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "canal_notificacao")]
pub enum CanalNotificacao {
    Email,
    WhatsApp,
    SMS,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "modalidade_pref")]
pub enum ModalidadePref {
    Online,
    Presencial,
    Hibrido,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ConfiguracaoPaciente {
    pub id: i32,
    pub fk_paciente_id: i32,
    pub canal: Option<CanalNotificacao>,
    pub modalidade: Option<ModalidadePref>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ConsentimentoPaciente {
    pub id: i32,
    pub fk_paciente_id: i32,
    pub tcle_aceito: Option<bool>,
    pub transcricao_aceita: Option<bool>,
    pub chatbot_aceito: Option<bool>,
    pub metodo_assinatura: Option<String>,
    pub data_hora_aceite: Option<DateTime<Utc>>,
    pub ip_assinante: Option<String>,
}
