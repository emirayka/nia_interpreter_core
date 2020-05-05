use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn map(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `map' takes two arguments exactly.",
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

        results.push(result);
    }

    Ok(interpreter.vec_to_list(results))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_mapped_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (
                "(list:map (function (lambda (value) (* 2 value))) '(1 2 3 4 5))",
                "'(2 4 6 8 10)",
            ),
            (
                "(list:map (function (lambda (value) (* value value))) '(1 2 3 4 5))",
                "'(1 4 9 16 25)",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:map 1 '())",
            "(list:map 1.1 '())",
            "(list:map #t '())",
            "(list:map #f '())",
            "(list:map \"string\" '())",
            "(list:map 'symbol '())",
            "(list:map :keyword '())",
            "(list:map '(1 2 3) '())",
            "(list:map {} '())",
            "(list:map (function (lambda (_1) nil)) 1)",
            "(list:map (function (lambda (_1) nil)) 1.1)",
            "(list:map (function (lambda (_1) nil)) #t)",
            "(list:map (function (lambda (_1) nil)) #f)",
            "(list:map (function (lambda (_1) nil)) \"string\")",
            "(list:map (function (lambda (_1) nil)) 'symbol)",
            "(list:map (function (lambda (_1) nil)) :keyword)",
            "(list:map (function (lambda (_1) nil)) {})",
            "(list:map (function (lambda (_1) nil)) #())",
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
            vec!["(list:map)", "(list:map 1)", "(list:map 1 2 3)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
