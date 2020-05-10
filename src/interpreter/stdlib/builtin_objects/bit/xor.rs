use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn xor(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `bit:xor' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let v1 = library::read_as_i64(values.remove(0))?;

    let v2 = library::read_as_i64(values.remove(0))?;

    let result = v1 ^ v2;

    Ok(Value::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_xor_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(bit:xor 0 0)", "0"),
            ("(bit:xor 0 1)", "1"),
            ("(bit:xor 0 2)", "2"),
            ("(bit:xor 0 3)", "3"),
            ("(bit:xor 1 0)", "1"),
            ("(bit:xor 1 1)", "0"),
            ("(bit:xor 1 2)", "3"),
            ("(bit:xor 1 3)", "2"),
            ("(bit:xor 2 0)", "2"),
            ("(bit:xor 2 1)", "3"),
            ("(bit:xor 2 2)", "0"),
            ("(bit:xor 2 3)", "1"),
            ("(bit:xor 3 0)", "3"),
            ("(bit:xor 3 1)", "2"),
            ("(bit:xor 3 2)", "1"),
            ("(bit:xor 3 3)", "0"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(bit:xor 1 1.1)",
            "(bit:xor 1 #t)",
            "(bit:xor 1 #f)",
            "(bit:xor 1 'symbol)",
            "(bit:xor 1 \"string\")",
            "(bit:xor 1 :keyword)",
            "(bit:xor 1 '(s-expression))",
            "(bit:xor 1 {})",
            "(bit:xor 1 #())",
            "(bit:xor 1.1 1)",
            "(bit:xor #t 1)",
            "(bit:xor #f 1)",
            "(bit:xor 'symbol 1)",
            "(bit:xor \"string\" 1)",
            "(bit:xor :keyword 1)",
            "(bit:xor '(s-expression) 1)",
            "(bit:xor {} 1)",
            "(bit:xor #() 1)",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(bit:xor)", "(bit:xor 1)", "(bit:xor 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
