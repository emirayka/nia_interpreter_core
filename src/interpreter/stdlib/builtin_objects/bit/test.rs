use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn test(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `bit:test' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let index = library::read_as_i64(values.remove(0))?;

    if index < 0 || index > 63 {
        return Error::invalid_argument_error(
            "Built-in function `bit:test' takes value between [0; 64) as bit index.",
        )
        .into();
    }

    let value = library::read_as_i64(values.remove(0))?;

    let result = ((0x1 << index) & value) != 0;

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_test_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(bit:test 0 0)", "#f"),
            ("(bit:test 0 1)", "#t"),
            ("(bit:test 0 2)", "#f"),
            ("(bit:test 0 4)", "#f"),
            ("(bit:test 1 0)", "#f"),
            ("(bit:test 1 1)", "#f"),
            ("(bit:test 1 2)", "#t"),
            ("(bit:test 1 4)", "#f"),
            ("(bit:test 2 0)", "#f"),
            ("(bit:test 2 1)", "#f"),
            ("(bit:test 2 2)", "#f"),
            ("(bit:test 2 4)", "#t"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(bit:test 1 1.1)",
            "(bit:test 1 #t)",
            "(bit:test 1 #f)",
            "(bit:test 1 'symbol)",
            "(bit:test 1 \"string\")",
            "(bit:test 1 :keyword)",
            "(bit:test 1 '(s-expression))",
            "(bit:test 1 {})",
            "(bit:test 1 #())",
            "(bit:test 1.1 1)",
            "(bit:test #t 1)",
            "(bit:test #f 1)",
            "(bit:test 'symbol 1)",
            "(bit:test \"string\" 1)",
            "(bit:test :keyword 1)",
            "(bit:test '(s-expression) 1)",
            "(bit:test {} 1)",
            "(bit:test #() 1)",
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

        let code_vector =
            vec!["(bit:test)", "(bit:test 1)", "(bit:test 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
