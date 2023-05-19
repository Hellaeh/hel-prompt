mod utils;

use std::str::FromStr;

pub trait Prompt: FromStr {
	fn prompt<S: AsRef<str>>(msg: S) -> Result<Self, <Self as FromStr>::Err> {
		<Self as FromStr>::from_str(&utils::prompt(msg))
	}
}

// TODO - once "specialization" is stable feature,
// TODO - impl Prompt for String
impl<T: FromStr> Prompt for T {}

/// Note - this function will create a copy of a string of stdin
/// Use [`prompt_string`] for strings
pub fn prompt<T: Prompt, S: AsRef<str>>(msg: S) -> Result<T, T::Err> {
	T::prompt(msg)
}

/// Returns `true` if first letter of a user response corresponds to "y" or "Y", otherwise `false`
///
/// e.g.
///
/// "Yes", "y", "Year" - `true`
///
/// "", "n", "No", "Anything else" - `false`
pub fn prompt_y<S: AsRef<str>>(msg: S) -> bool {
	let res = utils::prompt(msg);

	!res.is_empty() && unsafe { matches!(*res.as_ptr(), b'y' | b'Y') }
}

pub fn prompt_string<S: AsRef<str>>(msg: S) -> String {
	utils::prompt(msg)
}
