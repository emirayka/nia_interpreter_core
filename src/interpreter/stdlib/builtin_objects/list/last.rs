use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn last(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:last' takes one argument exactly."
        ).into();
    }

    let mut values = values;

    let vector = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    if let Some(value) = vector.last() {
        Ok(*value)
    } else {
        return Error::invalid_argument_error(
            "Built-in function `list:last' takes a list, that has one value at least."
        ).into()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn return_last_item_of_the_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:last '(1))", "1"),
            ("(list:last '(1 2))", "2"),
            ("(list:last '(1 2 3))", "3"),
            ("(list:last '(1 2 3 4))", "4"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_list_has_zero_length() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:last '())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:last 1)",
            "(list:last 1.1)",
            "(list:last #t)",
            "(list:last #f)",
            "(list:last \"string\")",
            "(list:last 'symbol)",
            "(list:last :keyword)",
            "(list:last {})",
            "(list:last #())",
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
            "(list:last)",
            "(list:last 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
