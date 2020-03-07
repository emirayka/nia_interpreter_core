use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::lib;

pub fn all_question(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:all?' takes two arguments exactly."
        ).into_result()
    }

    let mut values = values;

    let function_id = lib::read_as_function_id(
        interpreter,
        values.remove(0)
    )?;

    let vector = lib::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    for value in vector {
        let current_result = lib::execute_function(
            interpreter,
            environment_id,
            function_id,
            vec!(value)
        )?;

        match current_result {
            Value::Boolean(true) => {},
            Value::Boolean(false) => {
                return Ok(Value::Boolean(false))
            },
            _ => return interpreter.make_invalid_argument_error(
                "Built-in function `list:all?' takes a function that returns boolean value."
            ).into_result()
        }
    }
    
    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

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
