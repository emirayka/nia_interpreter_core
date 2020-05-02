use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn none_question(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:none?' takes two arguments exactly."
        ).into()
    }

    let mut values = values;

    let function_id = library::read_as_function_id(
        interpreter,
        values.remove(0)
    )?;

    let vector = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    if vector.len() == 0 {
        return Ok(Value::Boolean(true));
    }

    for value in vector {
        let current_result = library::execute_function(
            interpreter,
            environment_id,
            function_id,
            vec!(value)
        )?;

        match current_result {
            Value::Boolean(false) => {},
            Value::Boolean(true) => {
                return Ok(Value::Boolean(false))
            },
            _ => return Error::invalid_argument_error(
                "Built-in function `list:none?' takes a function that returns boolean value."
            ).into()
        }
    }

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn executes_function() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '())", "#t"),
            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '(1))", "#t"),
            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '(1 2))", "#f"),
            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '(1 2 3))", "#f"),
            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '(1 2 3 4))", "#f"),

            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '(2))", "#f"),
            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '(2 4))", "#f"),

            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '(1))", "#t"),
            ("(list:none? (function (lambda (value) (eq? (% value 2) 0))) '(1 3))", "#t"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_predicate_returns_not_a_boolean() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:none? (function (lambda (value) 1)) '(1 2 3 4 5))",
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
            "(list:none? 1 '())",
            "(list:none? 1.1 '())",
            "(list:none? #t '())",
            "(list:none? #f '())",
            "(list:none? \"string\" '())",
            "(list:none? 'symbol '())",
            "(list:none? :keyword '())",
            "(list:none? '(1 2 3) '())",
            "(list:none? {} '())",

            "(list:none? (function (lambda (_1) nil)) 1)",
            "(list:none? (function (lambda (_1) nil)) 1.1)",
            "(list:none? (function (lambda (_1) nil)) #t)",
            "(list:none? (function (lambda (_1) nil)) #f)",
            "(list:none? (function (lambda (_1) nil)) \"string\")",
            "(list:none? (function (lambda (_1) nil)) 'symbol)",
            "(list:none? (function (lambda (_1) nil)) :keyword)",
            "(list:none? (function (lambda (_1) nil)) {})",
            "(list:none? (function (lambda (_1) nil)) #())"
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
            "(list:none?)",
            "(list:none? 1)",
            "(list:none? 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
