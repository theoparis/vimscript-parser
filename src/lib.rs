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

		let next = result.into_inner().next().unwrap();
		assert_eq!(next.as_rule(), Rule::let_statement);
	}

	#[test]
	fn test_call() {
		let input = "call sign_undefine()";

		let result = ExpressionParser::parse(Rule::statement, input);

		if let Err(err) = result {
			panic!("{}", err);
		}

		let result = result.unwrap().next().unwrap();

		assert_eq!(result.as_rule(), Rule::statement);

		let next = result.into_inner().next().unwrap();
		assert_eq!(next.as_rule(), Rule::call_statement);
	}

	#[test]
	fn test_call_with_arguments() {
		let input = r#"call sdl#OpenWindow("New Window", 800, 600)"#;

		let result = ExpressionParser::parse(Rule::statement, input);

		if let Err(err) = result {
			panic!("{}", err);
		}

		let result = result.unwrap().next().unwrap();

		assert_eq!(result.as_rule(), Rule::statement);

		let next = result.into_inner().next().unwrap();
		assert_eq!(next.as_rule(), Rule::call_statement);
	}
}
