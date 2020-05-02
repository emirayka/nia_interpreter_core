use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn repeat(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:repeat' takes two arguments exactly."
        ).into();
    }

    let mut values = values;

    let count = library::read_as_i64(
        interpreter,
        values.remove(0)
    )? as usize;

    let value = values.remove(0);

    let values = std::iter::repeat(value)
        .take(count)
        .collect::<Vec<Value>>();

    Ok(interpreter.vec_to_list(values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_list_that_contains_repeated_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:repeat 0 1)", "'()"),
            ("(list:repeat 1 1)", "'(1)"),
            ("(list:repeat 2 1)", "'(1 1)"),
            ("(list:repeat 3 1)", "'(1 1 1)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:repeat 1.1 1)",
            "(list:repeat #t 1)",
            "(list:repeat #f 1)",
            "(list:repeat \"string\" 1)",
            "(list:repeat 'symbol 1)",
            "(list:repeat :keyword 1)",
            "(list:repeat '(1 2 3) 1)",
            "(list:repeat {} 1)",
            "(list:repeat #() 1)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:repeat)",
            "(list:repeat 1)",
            "(list:repeat 1 2 3)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
