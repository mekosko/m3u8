use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Environment {
	pub locate: String,
	pub target: String,
	pub at: String,
}

impl From<String> for Environment {
	fn from(value: String) -> Self {
		serde_json::from_str(value.as_str()).unwrap()
	}
}
