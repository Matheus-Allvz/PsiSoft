use crate::domain::paciente::{CanalNotificacao, ModalidadePref};
use crate::services::auth::hash_password;
use crate::services::email::enviar_email_resend;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacienteError {
    #[error("Erro de banco de dados: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Erro de validação: {0}")]
    #[allow(dead_code)]
    ValidationError(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CadastroPacienteRequest {
    pub fk_psicologo_id: i32,
    pub senha: Option<String>,
    pub nome: String,
    pub email: String,
    pub cpf: Option<String>,
    pub data_nascimento: Option<NaiveDate>,
    pub genero: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub telefone_alt: Option<String>,
    pub endereco: Option<String>,

    // Responsavel
    pub responsavel_nome: Option<String>,
    pub responsavel_cpf: Option<String>,
    pub responsavel_parentesco: Option<String>,
    pub responsavel_email: Option<String>,
    pub responsavel_telefone: Option<String>,

    // Configuracao
    pub config_canal: Option<CanalNotificacao>,
    pub config_modalidade: Option<ModalidadePref>,

    // Consentimento
    pub consent_tcle_aceito: Option<bool>,
    pub consent_transcricao: Option<bool>,
    pub consent_chatbot: Option<bool>,
    pub consent_metodo_assinatura: Option<String>,
    pub consent_ip_assinante: Option<String>,
}

pub async fn cadastrar_paciente(
    pool: &PgPool,
    req: CadastroPacienteRequest,
) -> Result<i32, PacienteError> {
    let mut tx = pool.begin().await?;

    let senha_pura = req.senha.clone().unwrap_or_else(|| "123456".to_string());
    let senha_hash = hash_password(&senha_pura);

    let login = if let Some(cpf) = &req.cpf {
        if !cpf.is_empty() { cpf.clone() } else { req.email.clone() }
    } else {
        req.email.clone()
    };

    let usuario_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO Usuario (nome, login, senha_hash, perfil, email, telefone, status)
        VALUES ($1, $2, $3, 'paciente', $4, $5, true)
        RETURNING id
        "#,
    )
    .bind(&req.nome)
    .bind(&login)
    .bind(&senha_hash)
    .bind(&req.email)
    .bind(&req.telefone)
    .fetch_one(&mut *tx)
    .await?;

    let paciente_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO Paciente (
            fk_usuario_id, fk_psicologo_id, nome, cpf, data_nascimento,
            genero, estado_civil, telefone, telefone_alt, endereco
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id
        "#,
    )
    .bind(usuario_id)
    .bind(req.fk_psicologo_id)
    .bind(&req.nome)
    .bind(&req.cpf)
    .bind(req.data_nascimento)
    .bind(&req.genero)
    .bind(&req.estado_civil)
    .bind(&req.telefone)
    .bind(&req.telefone_alt)
    .bind(&req.endereco)
    .fetch_one(&mut *tx)
    .await?;

    if let Some(resp_nome) = &req.responsavel_nome {
        if !resp_nome.is_empty() {
            sqlx::query(
                r#"
                INSERT INTO ResponsavelLegal (fk_paciente_id, nome, cpf, parentesco, email, telefone)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(paciente_id)
            .bind(resp_nome)
            .bind(&req.responsavel_cpf)
            .bind(&req.responsavel_parentesco)
            .bind(&req.responsavel_email)
            .bind(&req.responsavel_telefone)
            .execute(&mut *tx)
            .await?;
        }
    }

    sqlx::query(
        r#"
        INSERT INTO ConfiguracaoPaciente (fk_paciente_id, canal, modalidade)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(paciente_id)
    .bind(&req.config_canal)
    .bind(&req.config_modalidade)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO ConsentimentoPaciente (
            fk_paciente_id, tcle_aceito, transcricao_aceita, chatbot_aceito,
            metodo_assinatura, data_hora_aceite, ip_assinante
        )
        VALUES ($1, $2, $3, $4, $5, NOW(), $6)
        "#,
    )
    .bind(paciente_id)
    .bind(req.consent_tcle_aceito)
    .bind(req.consent_transcricao)
    .bind(req.consent_chatbot)
    .bind(&req.consent_metodo_assinatura)
    .bind(&req.consent_ip_assinante)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let email_to = req.email.clone();
    let nome_pac = req.nome.clone();
    tokio::spawn(async move {
        let subject = "Bem-vindo(a) à PsiSoft";
        let body = format!(
            "<p>Olá {}, seu cadastro foi realizado com sucesso!</p><p>Seu login é: {}</p><p>Sua senha provisória é: {}</p>",
            nome_pac, login, senha_pura
        );
        if let Err(e) = enviar_email_resend(&email_to, subject, &body).await {
            tracing::error!("Erro ao enviar email para paciente {}: {}", email_to, e);
        }
    });

    Ok(paciente_id)
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PacienteResponse {
    pub id: i32,
    pub nome: String,
    pub cpf: Option<String>,
    pub data_nascimento: Option<NaiveDate>,
    pub telefone: Option<String>,
    pub email: Option<String>,
}

pub async fn listar_pacientes(
    pool: &PgPool,
    fk_psicologo_id: i32,
) -> Result<Vec<PacienteResponse>, PacienteError> {
    let pacientes = sqlx::query_as::<_, PacienteResponse>(
        r#"
        SELECT p.id, p.nome, p.cpf, p.data_nascimento, p.telefone, u.email
        FROM Paciente p
        JOIN Usuario u ON p.fk_usuario_id = u.id
        WHERE p.fk_psicologo_id = $1
        ORDER BY p.nome ASC
        "#,
        fk_psicologo_id
    )
    .fetch_all(pool)
    .await?;

    Ok(pacientes)
}
