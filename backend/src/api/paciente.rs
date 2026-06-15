use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::api::auth::AppState;
use crate::services::paciente::{cadastrar_paciente, listar_pacientes, CadastroPacienteRequest, PacienteError};

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/novo", post(create_paciente_handler))
        .route("/psicologos/{id}/pacientes", get(listar_pacientes_handler))
        .with_state(state)
}

impl IntoResponse for PacienteError {
    fn into_response(self) -> Response {
        let status = match self {
            PacienteError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PacienteError::ValidationError(_) => StatusCode::BAD_REQUEST,
        };

        let body = Json(serde_json::json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}

async fn create_paciente_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CadastroPacienteRequest>,
) -> Result<impl IntoResponse, PacienteError> {
    let paciente_id = cadastrar_paciente(&state.db, payload).await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": paciente_id }))))
}

async fn listar_pacientes_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, PacienteError> {
    let pacientes = listar_pacientes(&state.db, id).await?;
    Ok(Json(pacientes))
}
