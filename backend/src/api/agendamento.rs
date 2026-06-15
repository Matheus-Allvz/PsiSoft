use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use chrono::NaiveDateTime;
use crate::api::auth::AppState;
use crate::domain::agendamento::ModalidadeConsulta;
use crate::services::agendamento::{
    buscar_horarios_disponiveis, cancelar_agendamento, criar_agendamento, listar_consultas,
    AgendamentoError,
};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAgendamentoRequest {
    pub fk_paciente_id: i32,
    pub fk_psicologo_id: i32,
    pub data_hora: NaiveDateTime,
    pub modalidade: ModalidadeConsulta,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/novo", post(create_agendamento_handler))
        .route("/{id}/cancelar", post(cancel_agendamento_handler))
        .route("/psicologos/{id}/horarios", get(get_horarios_handler))
        .route("/psicologos/{id}/consultas", get(listar_consultas_handler))
        .with_state(state)
}

impl IntoResponse for AgendamentoError {
    fn into_response(self) -> Response {
        let status = match self {
            AgendamentoError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };

        let body = Json(serde_json::json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}

async fn create_agendamento_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAgendamentoRequest>,
) -> Result<impl IntoResponse, AgendamentoError> {
    let consulta = criar_agendamento(
        &state.db,
        payload.fk_paciente_id,
        payload.fk_psicologo_id,
        payload.data_hora,
        payload.modalidade,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(consulta)))
}

async fn cancel_agendamento_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AgendamentoError> {
    cancelar_agendamento(&state.db, id).await?;

    Ok(StatusCode::OK)
}

async fn get_horarios_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AgendamentoError> {
    let horarios = buscar_horarios_disponiveis(&state.db, id).await?;

    Ok((StatusCode::OK, Json(horarios)))
}

async fn listar_consultas_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    axum::extract::Query(query): axum::extract::Query<PaginationQuery>,
) -> Result<impl IntoResponse, AgendamentoError> {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    let consultas = listar_consultas(&state.db, id, limit, offset).await?;

    Ok((StatusCode::OK, Json(consultas)))
}
