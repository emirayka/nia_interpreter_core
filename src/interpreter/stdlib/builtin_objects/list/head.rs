use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn head(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `head' takes one argument.",
        )
        .into();
    }

    let mut values = values;

    let mut values = library::read_as_vector(interpreter, values.remove(0))?;

    if values.len() > 0 {
        Ok(values.remove(0))
    } else {
        Error::invalid_argument_error(
            "Built-in function `head' takes one list with values.",
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_first_element_in_lists() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(list:head '(1))", "1"),
            ("(list:head '(2 1))", "2"),
            ("(list:head '(3 2 1))", "3"),
            ("(list:head '(4 3 2 1))", "4"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_list_has_zero_length() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(list:head '())"];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:head 1)",
            "(list:head 1.1)",
            "(list:head #t)",
            "(list:head #f)",
            "(list:head \"string\")",
            "(list:head 'symbol)",
            "(list:head :keyword)",
            "(list:head {})",
            "(list:head #())",
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

        let code_vector = vec!["(list:head)", "(list:head 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
