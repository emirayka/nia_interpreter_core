use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn or(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `bit:or' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let v1 = library::read_as_i64(values.remove(0))?;

    let v2 = library::read_as_i64(values.remove(0))?;

    let result = v1 | v2;

    Ok(Value::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_or_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(bit:or 0 0)", "0"),
            ("(bit:or 0 1)", "1"),
            ("(bit:or 1 0)", "1"),
            ("(bit:or 1 1)", "1"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(bit:or 1 1.1)",
            "(bit:or 1 #t)",
            "(bit:or 1 #f)",
            "(bit:or 1 'symbol)",
            "(bit:or 1 \"string\")",
            "(bit:or 1 :keyword)",
            "(bit:or 1 '(s-expression))",
            "(bit:or 1 {})",
            "(bit:or 1 #())",
            "(bit:or 1.1 1)",
            "(bit:or #t 1)",
            "(bit:or #f 1)",
            "(bit:or 'symbol 1)",
            "(bit:or \"string\" 1)",
            "(bit:or :keyword 1)",
            "(bit:or '(s-expression) 1)",
            "(bit:or {} 1)",
            "(bit:or #() 1)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(bit:or)", "(bit:or 1)", "(bit:or 1 2 3)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
