use axum::{
	routing::{delete, get, post},
	Router,
};
use dotenv::dotenv;
use routes::{helloworld::hello_world, *};
use std::sync::Arc;

mod db;
mod error;
mod routes;
mod state;

pub(crate) use anyhow::Context;
pub(crate) use axum::extract::{Json, State};
pub(crate) use error::{AnyhowError, AppError, Errors};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv().ok();
	tracing_subscriber::fmt::init();

	let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

	let app = init_router().await?;

	let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
		.await
		.context(format!("Failed to bind to port {}", port))?;

	tracing::info!("listening on http://localhost:{}", port);

	axum::serve(listener, app).await?;

	Ok(())
}

async fn init_router() -> anyhow::Result<Router> {
	let db = db::db().await?;
	let state = AppState { db: Arc::new(db) };
	let router: Router =
		Router::new().route("/", get(hello_world)).with_state(state);

	Ok(router)
}
