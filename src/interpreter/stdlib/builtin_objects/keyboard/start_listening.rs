use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn start_listening(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn executes_function() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:all? (function (lambda (value) (eq? (% value 2) 0))) '())", "#t"),
            ("(list:all? (function (lambda (value) (eq? (% value 2) 0))) '(1))", "#f"),
            ("(list:all? (function (lambda (value) (eq? (% value 2) 0))) '(1 2))", "#f"),
            ("(list:all? (function (lambda (value) (eq? (% value 2) 0))) '(1 2 3))", "#f"),
            ("(list:all? (function (lambda (value) (eq? (% value 2) 0))) '(1 2 3 4))", "#f"),

            ("(list:all? (function (lambda (value) (eq? (% value 2) 0))) '(2))", "#t"),
            ("(list:all? (function (lambda (value) (eq? (% value 2) 0))) '(2 4))", "#t"),
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
            "(list:all? (function (lambda (value) 1)) '(1 2 3 4 5))",
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
            "(list:all? 1 '())",
            "(list:all? 1.1 '())",
            "(list:all? #t '())",
            "(list:all? #f '())",
            "(list:all? \"string\" '())",
            "(list:all? 'symbol '())",
            "(list:all? :keyword '())",
            "(list:all? '(1 2 3) '())",
            "(list:all? {} '())",

            "(list:all? (function (lambda (_1) nil)) 1)",
            "(list:all? (function (lambda (_1) nil)) 1.1)",
            "(list:all? (function (lambda (_1) nil)) #t)",
            "(list:all? (function (lambda (_1) nil)) #f)",
            "(list:all? (function (lambda (_1) nil)) \"string\")",
            "(list:all? (function (lambda (_1) nil)) 'symbol)",
            "(list:all? (function (lambda (_1) nil)) :keyword)",
            "(list:all? (function (lambda (_1) nil)) {})",
            "(list:all? (function (lambda (_1) nil)) #())"
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
            "(list:all?)",
            "(list:all? 1)",
            "(list:all? 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
