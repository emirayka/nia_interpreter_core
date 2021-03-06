use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn append(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:append' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let item = values.remove(0);

    let mut values = library::read_as_vector(interpreter, values.remove(0))?;

    values.push(item);

    Ok(interpreter.vec_to_list(values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_heads() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(list:append 0 '(1 2 3 4 5))", "'(1 2 3 4 5 0)"),
            ("(list:append 1 '(1 2 3 4 5))", "'(1 2 3 4 5 1)"),
            ("(list:append 2 '(1 2 3 4 5))", "'(1 2 3 4 5 2)"),
            ("(list:append 3 '(1 2 3 4 5))", "'(1 2 3 4 5 3)"),
            ("(list:append 4 '(1 2 3 4 5))", "'(1 2 3 4 5 4)"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:append 1 1)",
            "(list:append 1 1.1)",
            "(list:append 1 #t)",
            "(list:append 1 #f)",
            "(list:append 1 \"string\")",
            "(list:append 1 'symbol)",
            "(list:append 1 :keyword)",
            "(list:append 1 {})",
            "(list:append 1 #())",
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
            vec!["(list:append)", "(list:append 1)", "(list:append 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
