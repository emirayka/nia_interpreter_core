use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn keyword(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `to:keyword' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;

    match values.remove(0) {
        Value::String(string_id) => {
            let string = interpreter.get_string(string_id)?.get_string().clone();

            let keyword = interpreter.intern_keyword_value(&string);

            Ok(keyword)
        }
        keyword @ Value::Keyword(_) => Ok(keyword),
        _ => Error::invalid_argument_error("Only keywords or strings can be casted to keyword.")
            .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(to:keyword \"a\")", ":a"),
            ("(to:keyword \"string\")", ":string"),
            ("(to:keyword :a)", ":a"),
            ("(to:keyword :string)", ":string"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_generic_execution_error_when_invalid_conversion() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            "(to:keyword 1)",
            "(to:keyword 1.1)",
            "(to:keyword #t)",
            "(to:keyword #f)",
            "(to:keyword 'symbol)",
            "(to:keyword nil)",
            "(to:keyword '())",
            "(to:keyword '(1 2 3))",
            "(to:keyword {})",
            "(to:keyword #())",
            "(to:keyword (flookup 'flookup))",
            "(to:keyword (function (macro () 1)))",
            "(to:keyword (flookup 'cond))",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(to:keyword)", "(to:keyword 1 2)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
