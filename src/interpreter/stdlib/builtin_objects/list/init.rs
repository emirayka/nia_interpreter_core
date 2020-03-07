use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn init(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:init' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    let mut vector = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    if vector.len() > 0 {
        vector.remove(vector.len() - 1);

        Ok(interpreter.vec_to_list(vector))
    } else {
        return interpreter.make_invalid_argument_error(
            "Built-in function `list:init' takes a list, that has one value at least."
        ).into_result()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn return_init_of_the_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:init '(1))", "'()"),
            ("(list:init '(1 2))", "'(1)"),
            ("(list:init '(1 2 3))", "'(1 2)"),
            ("(list:init '(1 2 3 4))", "'(1 2 3)"),
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
            "(list:init '())",
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
            "(list:init 1)",
            "(list:init 1.1)",
            "(list:init #t)",
            "(list:init #f)",
            "(list:init \"string\")",
            "(list:init 'symbol)",
            "(list:init :keyword)",
            "(list:init {})",
            "(list:init #())",
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
            "(list:init)",
            "(list:init 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
