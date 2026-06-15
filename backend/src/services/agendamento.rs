use crate::domain::agendamento::{Consulta, HorarioDisponivel, ModalidadeConsulta, StatusConsulta};
use crate::domain::user::Psicologo;
use chrono::NaiveDateTime;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgendamentoError {
    #[error("Conflito de horário para este psicólogo")]
    ConflitoHorario,
    #[error("Psicólogo não validado para consultas online")]
    EpsiNaoValidado,
    #[error("Psicólogo não encontrado")]
    PsicologoNaoEncontrado,
    #[error("Erro de banco de dados: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub async fn buscar_horarios_disponiveis(
    pool: &PgPool,
    fk_psicologo_id: i32,
) -> Result<Vec<HorarioDisponivel>, AgendamentoError> {
    let horarios = sqlx::query_as::<_, HorarioDisponivel>(
        "SELECT id, fk_psicologo_id, dia_semana, hora_inicio, hora_fim FROM HorarioDisponivel WHERE fk_psicologo_id = $1",
    )
    .bind(fk_psicologo_id)
    .fetch_all(pool)
    .await?;

    Ok(horarios)
}

pub async fn criar_agendamento(
    pool: &PgPool,
    fk_paciente_id: i32,
    fk_psicologo_id: i32,
    data_hora: NaiveDateTime,
    modalidade: ModalidadeConsulta,
) -> Result<Consulta, AgendamentoError> {
    if let ModalidadeConsulta::Online = modalidade {
        let psicologo = sqlx::query_as::<_, Psicologo>(
            "SELECT crp, validador_epsi, fk_usuario_id FROM Psicologo WHERE fk_usuario_id = $1",
        )
        .bind(fk_psicologo_id)
        .fetch_optional(pool)
        .await?;

        let psicologo = psicologo.ok_or(AgendamentoError::PsicologoNaoEncontrado)?;

        if psicologo.validador_epsi.is_none() {
            return Err(AgendamentoError::EpsiNaoValidado);
        }
    }

    let conflito: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM Consulta WHERE fk_psicologo_id = $1 AND data_hora = $2 AND status != 'Cancelada'",
    )
    .bind(fk_psicologo_id)
    .bind(data_hora)
    .fetch_optional(pool)
    .await?;

    if conflito.is_some() {
        return Err(AgendamentoError::ConflitoHorario);
    }

    let status = StatusConsulta::Agendada;

    let consulta = sqlx::query_as::<_, Consulta>(
        r#"
        INSERT INTO Consulta (fk_paciente_id, fk_psicologo_id, data_hora, modalidade, status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, fk_paciente_id, fk_psicologo_id, data_hora, modalidade, status, link_reuniao
        "#,
    )
    .bind(fk_paciente_id)
    .bind(fk_psicologo_id)
    .bind(data_hora)
    .bind(modalidade)
    .bind(status)
    .fetch_one(pool)
    .await?;

    Ok(consulta)
}

pub async fn cancelar_agendamento(pool: &PgPool, id: i32) -> Result<(), AgendamentoError> {
    sqlx::query("UPDATE Consulta SET status = 'Cancelada' WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn listar_consultas(
    pool: &PgPool,
    fk_psicologo_id: i32,
    limit: i64,
    offset: i64,
) -> Result<Vec<Consulta>, AgendamentoError> {
    let consultas = sqlx::query_as::<_, Consulta>(
        "SELECT id, fk_paciente_id, fk_psicologo_id, data_hora, modalidade, status, link_reuniao FROM Consulta WHERE fk_psicologo_id = $1 ORDER BY data_hora DESC LIMIT $2 OFFSET $3",
    )
    .bind(fk_psicologo_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(consultas)
}
