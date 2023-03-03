use crate::utils::prompt;

pub struct Prompt {}

impl Prompt {
	/// Returns `true` if first letter of a user response corresponds to "y" or "Y", otherwise `false`
	///
	/// e.g.
	///
	/// "Yes", "y", "Year" - `true`
	///
	/// "", "n", "No", "Anything else" - `false`
	pub fn y(msg: impl AsRef<str>) -> bool {
		let res = prompt(msg);

		!res.is_empty() && unsafe { matches!(*res.as_ptr(), b'y' | b'Y') }
	}

	/// Returns a [String] response from `Stdin`
	pub fn response(msg: impl AsRef<str>) -> String {
		prompt(msg)
	}
}
