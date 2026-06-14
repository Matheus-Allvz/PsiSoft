mod api;
mod domain;
mod events;
mod services;

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use crate::api::auth::AppState;
use crate::events::EventBus;
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await?;

    let event_bus = EventBus::new();
    
    // Example: Listen to auth events
    let mut auth_rx = event_bus.auth_tx.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = auth_rx.recv().await {
            match event {
                events::AuthEvent::LoginSuccess { user_id } => {
                    tracing::info!("User logged in: {}", user_id);
                }
            }
        }
    });

    let state = Arc::new(AppState {
        db,
        event_bus,
    });

    let app = Router::new()
        .nest("/auth", api::auth::router(state.clone()))
        .layer(tower_http::cors::CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
