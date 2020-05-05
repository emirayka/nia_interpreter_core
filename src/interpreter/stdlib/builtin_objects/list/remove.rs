use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn remove(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:remove' takes exactly two arguments.",
        )
        .into();
    }

    let mut values = values;

    let index = library::read_as_i64(values.remove(0))? as usize;

    let mut values = library::read_as_vector(interpreter, values.remove(0))?;

    if values.len() < index {
        return Error::invalid_argument_error(
            "Built-in function `list:remove' takes a list that has enough items.",
        )
        .into();
    }

    values.remove(index);

    // todo: probably change it because it's not optimal
    Ok(interpreter.vec_to_list(values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn remove() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(list:remove 0 '(1 2 3 4))", "'(2 3 4)"),
            ("(list:remove 1 '(1 2 3 4))", "'(1 3 4)"),
            ("(list:remove 2 '(1 2 3 4))", "'(1 2 4)"),
            ("(list:remove 3 '(1 2 3 4))", "'(1 2 3)"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_list_has_zero_length() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(list:remove 1 '())"];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:remove 1.1 '(1 2 3))",
            "(list:remove #t '(1 2 3))",
            "(list:remove #f '(1 2 3))",
            "(list:remove \"string\" '(1 2 3))",
            "(list:remove 'symbol '(1 2 3))",
            "(list:remove :keyword '(1 2 3))",
            "(list:remove '(1 2 3) '(1 2 3))",
            "(list:remove {} '(1 2 3))",
            "(list:remove #() '(1 2 3))",
            "(list:remove 0 1)",
            "(list:remove 0 1.1)",
            "(list:remove 0 #t)",
            "(list:remove 0 #f)",
            "(list:remove 0 \"string\")",
            "(list:remove 0 'symbol)",
            "(list:remove 0 :keyword)",
            "(list:remove 0 {})",
            "(list:remove 0 #())",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(list:remove)", "(list:remove 1)", "(list:remove 1 2 3)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
