use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn unzip(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:unzip' takes one argument exactly."
        ).into();
    }

    let mut values = values;

    let vector = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    let mut first_vector = Vec::new();
    let mut second_vector = Vec::new();

    for value in vector {
        let mut zipped = library::read_as_vector(
            interpreter,
            value
        )?;

        if zipped.len() != 2 {
            return Error::invalid_argument_error(
                "Built-in function `list:unzip' takes list of two-value lists."
            ).into();
        }

        let first_value = zipped.remove(0);
        let second_value = zipped.remove(0);

        first_vector.push(first_value);
        second_vector.push(second_value);
    }

    let first_list = interpreter.vec_to_list(first_vector);
    let second_list = interpreter.vec_to_list(second_vector);

    let result_vector = vec!(first_list, second_list);
    let result_list = interpreter.vec_to_list(result_vector);

    Ok(result_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_concatenated_lists() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            ("(list:unzip '((1 a)))", "'((1) (a))"),
            ("(list:unzip '((1 a) (2 b)))", "'((1 2) (a b))"),
            ("(list:unzip '((1 a) (2 b) (3 c)))", "'((1 2 3) (a b c))"),
            ("(list:unzip '((1 a) (2 b) (3 c) (4 d)))", "'((1 2 3 4) (a b c d))"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:unzip 1)",
            "(list:unzip 1.1)",
            "(list:unzip #t)",
            "(list:unzip #f)",
            "(list:unzip \"string\")",
            "(list:unzip 'symbol)",
            "(list:unzip :keyword)",
            "(list:unzip {})",
            "(list:unzip #())",
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
            "(list:unzip)",
            "(list:unzip 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
