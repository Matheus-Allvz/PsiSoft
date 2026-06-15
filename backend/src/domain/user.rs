use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub login: String,
    pub senha_hash: String,
    pub perfil: PerfilUsuario,
    pub email: Option<String>,
    pub telefone: Option<String>,
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "perfil_usuario")]
pub enum PerfilUsuario {
    Psicologo,
    Secretaria,
    Contador,
    #[sqlx(rename = "paciente")]
    Paciente,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Psicologo {
    pub crp: String,
    pub validador_epsi: Option<String>,
    pub fk_usuario_id: i32,
}
