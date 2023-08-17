use {serde::Deserialize, std::net::SocketAddr};

#[derive(Debug, Deserialize)]
pub struct Environment {
	pub locate: String,
	pub at: SocketAddr,
}

impl From<String> for Environment {
	fn from(value: String) -> Self {
		serde_json::from_str(value.as_str()).unwrap()
	}
}
