use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::domain::user::{Usuario};
use crate::services::auth::{hash_password, verify_password, generate_jwt};
use crate::events::{EventBus, AuthEvent};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub nome: String,
    pub login: String,
    pub senha: String,
    pub email: Option<String>,
    pub telefone: Option<String>,
    pub crp: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub senha: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub struct AppState {
    pub db: PgPool,
    pub event_bus: EventBus,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state)
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AuthError> {
    let mut tx = state.db.begin().await.map_err(|_| AuthError::DatabaseError)?;

    let senha_hash = hash_password(&payload.senha);

    // No Postgres usamos $1, $2... e para pegar o ID gerado usamos RETURNING id
    let row: (i32,) = sqlx::query_as(
        r#"
        INSERT INTO Usuario (nome, login, senha_hash, perfil, email, telefone, status)
        VALUES ($1, $2, $3, 'Psicologo', $4, $5, $6)
        RETURNING id
        "#
    )
    .bind(&payload.nome)
    .bind(&payload.login)
    .bind(&senha_hash)
    .bind(&payload.email)
    .bind(&payload.telefone)
    .bind(true)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Registration error: {:?}", e);
        AuthError::DatabaseError
    })?;

    let user_id = row.0;

    sqlx::query(
        r#"
        INSERT INTO Psicologo (crp, fk_usuario_id)
        VALUES ($1, $2)
        "#
    )
    .bind(&payload.crp)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Psicologo insert error: {:?}", e);
        AuthError::DatabaseError
    })?;

    tx.commit().await.map_err(|_| AuthError::DatabaseError)?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Usuário registrado com sucesso"
        }))
    ))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthError> {
    let user = sqlx::query_as::<_, Usuario>(
        r#"
        SELECT id, nome, login, senha_hash, perfil, email, telefone, status
        FROM Usuario
        WHERE login = $1
        "#
    )
    .bind(&payload.login)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Login database error: {:?}", e);
        AuthError::DatabaseError
    })?
    .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(&payload.senha, &user.senha_hash) {
        return Err(AuthError::InvalidCredentials);
    }

    let token = generate_jwt(user.id);

    // Dispatch event
    let _ = state.event_bus.auth_tx.send(AuthEvent::LoginSuccess { user_id: user.id });

    Ok(Json(AuthResponse { token }))
}

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    DatabaseError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };

        let body = Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
