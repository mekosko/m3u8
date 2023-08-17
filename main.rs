use axum::{routing::get, Router};
use environment::Environment;

mod environment;
mod executor;

#[tokio::main]
async fn main() {
	let environment = std::fs::read_to_string("m3u8.json").unwrap();
	let environment = std::sync::Arc::new(Environment::from(environment));

	let app = Router::new().route("/", get(executor::get)).with_state(environment.clone());

	let out = axum::Server::bind(&environment.socket).serve(app.into_make_service()).await;

	out.unwrap();
	()
}
