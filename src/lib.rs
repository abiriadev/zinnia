use std::collections::HashMap;

pub struct Parser {
	flags_long: Vec<String>,
	flags_short: Vec<char>,
	options_long: Vec<String>,
	options_short: Vec<char>,
}

impl Parser {
	pub fn parse<I>(&self, args: I) -> (Vec<String>, HashMap<String, String>)
	where I: Iterator<Item = String> {
		let mut iter = args.into_iter();
		let mut opts = HashMap::new();
		let mut flags = Vec::new();

		while let Some(arg) = iter.next() {
			if arg.starts_with("--") {
				if self
					.flags_long
					.iter()
					.find(|&f| f == &arg[2..])
					.is_some()
				{
					flags.push(arg[2..].to_owned());
				} else if self
					.options_long
					.iter()
					.find(|&f| f == &arg[2..])
					.is_some()
				{
					opts.insert(
						arg[2..].to_owned(),
						iter.next().unwrap(),
					);
				} else {
					panic!()
				}
			}

			panic!()
		}

		(flags, opts)
	}
}

#[test]
fn test() {
	let res = Parser {
		flags_long: todo!(),
		flags_short: todo!(),
		options_long: todo!(),
		options_short: todo!(),
	}
	.parse(
		["--name", "abiria", "-a"]
			.into_iter()
			.map(ToOwned::to_owned),
	);

	assert_eq!(
		res.1.get("name"),
		Some(&"abiria".to_owned())
	);
}
