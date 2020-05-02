use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn length(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:length' takes one argument exactly"
        ).into();
    }

    let mut values = values;

    let vector = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    let length = vector.len() as i64;

    Ok(Value::Integer(length))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn return_the_length_of_the_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:length '())", "0"),
            ("(list:length '(1))", "1"),
            ("(list:length '(1 2))", "2"),
            ("(list:length '(1 2 3))", "3"),
            ("(list:length '(1 2 3 4))", "4"),
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
            "(list:length 1)",
            "(list:length 1.1)",
            "(list:length #t)",
            "(list:length #f)",
            "(list:length \"string\")",
            "(list:length 'symbol)",
            "(list:length :keyword)",
            "(list:length {})",
            "(list:length #())",
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
            "(list:length)",
            "(list:length 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
