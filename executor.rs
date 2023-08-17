use axum::{extract::State, http::StatusCode, response::IntoResponse};

type Environment = std::sync::Arc<crate::environment::Environment>;

pub async fn get(State(environment): State<Environment>) -> Result<impl IntoResponse, StatusCode> {
	Ok(format!("{:?}", environment))
}
