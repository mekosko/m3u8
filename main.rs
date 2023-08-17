use environment::Environment;

mod environment;

fn main() {
	let environment = std::fs::read_to_string("m3u8.json").unwrap();
	let environment = Environment::from(environment.as_str());

	println!("{:?}", environment);
}
