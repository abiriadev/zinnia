use std::collections::HashMap;

pub fn parse<I>(args: I) -> HashMap<String, String>
where I: Iterator<Item = String> {
	let mut iter = args.into_iter();
	let mut opts = HashMap::new();

	while let Some(arg) = iter.next() {
		if arg.starts_with("--") {
			opts.insert(
				arg[2..].to_owned(),
				iter.next().unwrap(),
			);
		}

		if arg.starts_with("-") {
			opts.insert(
				arg[1..].to_owned(),
				iter.next().unwrap(),
			);
		}
	}

	opts
}
