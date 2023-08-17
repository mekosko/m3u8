use axum::{extract::State, http::StatusCode, response::IntoResponse};

type Environment = std::sync::Arc<crate::environment::Environment>;

pub async fn get(State(environment): State<Environment>) -> Result<impl IntoResponse, StatusCode> {
	let mut components = Vec::new();

	let targets = std::fs::read_dir(&environment.target).unwrap().map(|target| target.unwrap());

	for target in targets {
		components.push(target.file_name().into_string().unwrap());
	}

	Ok(format!("{:?}", components))
}
