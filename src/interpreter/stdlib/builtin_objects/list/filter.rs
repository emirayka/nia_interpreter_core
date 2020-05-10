use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn filter(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:filter?' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let function_id = library::read_as_function_id(values.remove(0))?;

    let values = library::read_as_vector(interpreter, values.remove(0))?;

    let mut results = Vec::new();

    for value in values {
        let result = library::execute_function(
            interpreter,
            environment_id,
            function_id,
            vec![value],
        )?;

        match result {
            Value::Boolean(true) => {
                results.push(value);
            }
            Value::Boolean(false) => {}
            _ => {
                return Error::invalid_argument_error(
                    "Built-in function `filter' takes a function that returns a boolean value.",
                )
                .into()
            }
        }
    }

    Ok(interpreter.vec_to_list(results))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_mapped_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (
                "(list:filter (function (lambda (value) (eq? (% value 2) 0))) '(1 2 3 4 5))",
                "'(2 4)",
            ),
            (
                "(list:filter (function (lambda (value) (eq? (% value 2) 1))) '(1 2 3 4 5))",
                "'(1 3 5)",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_predicate_returns_not_a_boolean() {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(list:filter (function (lambda (value) 1)) '(1 2 3 4 5))"];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:filter 1 '())",
            "(list:filter 1.1 '())",
            "(list:filter #t '())",
            "(list:filter #f '())",
            "(list:filter \"string\" '())",
            "(list:filter 'symbol '())",
            "(list:filter :keyword '())",
            "(list:filter '(1 2 3) '())",
            "(list:filter {} '())",
            "(list:filter (function (lambda (_1) nil)) 1)",
            "(list:filter (function (lambda (_1) nil)) 1.1)",
            "(list:filter (function (lambda (_1) nil)) #t)",
            "(list:filter (function (lambda (_1) nil)) #f)",
            "(list:filter (function (lambda (_1) nil)) \"string\")",
            "(list:filter (function (lambda (_1) nil)) 'symbol)",
            "(list:filter (function (lambda (_1) nil)) :keyword)",
            "(list:filter (function (lambda (_1) nil)) {})",
            "(list:filter (function (lambda (_1) nil)) #())",
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
            vec!["(list:filter)", "(list:filter 1)", "(list:filter 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
