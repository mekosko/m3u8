use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Environment<'a> {
	locate: &'a str,
	target: &'a str,
	listen: &'a str,
}

impl<'a> From<&'a str> for Environment<'a> {
	fn from(value: &'a str) -> Self {
		serde_json::from_str(value).unwrap()
	}
}
