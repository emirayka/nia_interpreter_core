use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::lib;

pub fn remove(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 3 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:remove' takes exactly three arguments."
        ).into_result();
    }

    let mut values = values;

    let index = lib::read_as_i64(
        interpreter,
        values.remove(0)
    )? as usize;

    let value = values.remove(0);

    let mut values = lib::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    if values.len() < index {
        return interpreter.make_invalid_argument_error(
            "Built-in function `list:remove' takes a list that has enough items."
        ).into_result();
    }

    values.remove(index);

    // todo: probably change it because it's not optimal
    Ok(interpreter.vec_to_list(values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn remove() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:remove 0 0 '(1 2 3 4))", "'(2 3 4)"),
            ("(list:remove 1 0 '(1 2 3 4))", "'(1 3 4)"),
            ("(list:remove 2 0 '(1 2 3 4))", "'(1 2 4)"),
            ("(list:remove 3 0 '(1 2 3 4))", "'(1 2 3)"),
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
            "(list:remove 1 1 '())",
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
            "(list:remove 1.1 1 '(1 2 3))",
            "(list:remove #t 1 '(1 2 3))",
            "(list:remove #f 1 '(1 2 3))",
            "(list:remove \"string\" 1 '(1 2 3))",
            "(list:remove 'symbol 1 '(1 2 3))",
            "(list:remove :keyword 1 '(1 2 3))",
            "(list:remove '(1 2 3) 1 '(1 2 3))",
            "(list:remove {} 1 '(1 2 3))",
            "(list:remove #() 1 '(1 2 3))",

            "(list:remove 0 1 1)",
            "(list:remove 0 1 1.1)",
            "(list:remove 0 1 #t)",
            "(list:remove 0 1 #f)",
            "(list:remove 0 1 \"string\")",
            "(list:remove 0 1 'symbol)",
            "(list:remove 0 1 :keyword)",
            "(list:remove 0 1 {})",
            "(list:remove 0 1 #())",
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
            "(list:remove)",
            "(list:remove 1)",
            "(list:remove 1 2)",
            "(list:remove 1 2 3 4)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
