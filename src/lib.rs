#![no_std]

extern crate alloc;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "vimscript.pest"]
pub struct ExpressionParser;

#[cfg(test)]
mod tests {
	use pest::Parser;

	use crate::{ExpressionParser, Rule};

	#[test]
	fn test_let() {
		let input = "let l:my_var = 1";

		let result = ExpressionParser::parse(Rule::statement, input);

		if let Err(err) = result {
			panic!("{}", err);
		}

		let result = result.unwrap().next().unwrap();

		assert_eq!(result.as_rule(), Rule::statement);
	}
}
