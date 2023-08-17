use {serde::Deserialize, std::net::SocketAddr};

#[derive(Deserialize)]
struct RawData {
	socket: Vec<String>,
	locate: Vec<String>,
	target: Vec<String>,
}

#[derive(Debug)]
pub struct Environment {
	pub socket: SocketAddr,
	pub locate: String,
	pub target: String,
}

impl From<String> for Environment {
	fn from(value: String) -> Self {
		let data = serde_json::from_str::<RawData>(value.as_str()).unwrap();

		let socket = format!("{}:{}", data.socket[0], data.socket[1]).parse().unwrap();

		let mut locate = String::new();
		let mut components = data.locate.into_iter();

		locate.push_str(components.next().unwrap().as_str());
		locate.push_str("://");

		for i in components {
			locate.push_str(i.as_str());
			locate.push('/')
		}

		let mut target = String::new();
		let mut components = data.target.into_iter();

		(components.next().unwrap() == "1").then(|| target.push('/'));

		for i in components {
			target.push_str(i.as_str());
			target.push('/')
		}

		Self {
			socket,
			locate,
			target,
		}
	}
}
