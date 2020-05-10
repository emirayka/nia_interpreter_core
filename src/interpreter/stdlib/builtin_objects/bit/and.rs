use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn and(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `bit:and' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let v1 = library::read_as_i64(values.remove(0))?;

    let v2 = library::read_as_i64(values.remove(0))?;

    let result = v1 & v2;

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
    fn returns_correct_results() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(bit:and 0 0)", "0"),
            ("(bit:and 0 1)", "0"),
            ("(bit:and 1 0)", "0"),
            ("(bit:and 1 1)", "1"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(bit:and 1 1.1)",
            "(bit:and 1 #t)",
            "(bit:and 1 #f)",
            "(bit:and 1 'symbol)",
            "(bit:and 1 \"string\")",
            "(bit:and 1 :keyword)",
            "(bit:and 1 '(s-expression))",
            "(bit:and 1 {})",
            "(bit:and 1 #())",
            "(bit:and 1.1 1)",
            "(bit:and #t 1)",
            "(bit:and #f 1)",
            "(bit:and 'symbol 1)",
            "(bit:and \"string\" 1)",
            "(bit:and :keyword 1)",
            "(bit:and '(s-expression) 1)",
            "(bit:and {} 1)",
            "(bit:and #() 1)",
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

        let code_vector = vec!["(bit:and)", "(bit:and 1)", "(bit:and 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
