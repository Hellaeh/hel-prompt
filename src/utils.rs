use std::io::stdin;

pub(crate) fn prompt(msg: impl AsRef<str>) -> String {
	println!("{}", msg.as_ref());

	let mut response = String::with_capacity(128);

	stdin().read_line(&mut response).unwrap();

	response.truncate(response.trim_end().len());

	response
}
