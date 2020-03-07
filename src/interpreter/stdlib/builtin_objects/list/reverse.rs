use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn reverse(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:reverse' takes one argument."
        ).into_result();
    }

    let mut values = values;

    let values = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    let values = values.into_iter().rev().collect::<Vec<Value>>();

    Ok(interpreter.vec_to_list(values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_first_element_in_lists() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:reverse '())", "'()"),
            ("(list:reverse '(1))", "'(1)"),
            ("(list:reverse '(2 1))", "'(1 2)"),
            ("(list:reverse '(3 2 1))", "'(1 2 3)"),
            ("(list:reverse '(4 3 2 1))", "'(1 2 3 4)"),
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
            "(list:reverse 1)",
            "(list:reverse 1.1)",
            "(list:reverse #t)",
            "(list:reverse #f)",
            "(list:reverse \"string\")",
            "(list:reverse 'symbol)",
            "(list:reverse :keyword)",
            "(list:reverse {})",
            "(list:reverse #())",
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
            "(list:reverse)",
            "(list:reverse 1 2)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
