use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn any_question(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:any?' takes two arguments exactly."
        ).into_result()
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
                return Ok(Value::Boolean(true))
            },
            _ => return Error::invalid_argument_error(
                "Built-in function `list:any?' takes a function that returns boolean value."
            ).into_result()
        }
    }

    Ok(Value::Boolean(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn executes_function() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:any? #(eq? (% %1 2) 0) '())", "#f"),
            ("(list:any? #(eq? (% %1 2) 0) '(1))", "#f"),
            ("(list:any? #(eq? (% %1 2) 0) '(1 2))", "#t"),
            ("(list:any? #(eq? (% %1 2) 0) '(1 2 3))", "#t"),
            ("(list:any? #(eq? (% %1 2) 0) '(1 2 3 4))", "#t"),

            ("(list:any? #(eq? (% %1 2) 0) '(2))", "#t"),
            ("(list:any? #(eq? (% %1 2) 0) '(2 4))", "#t"),

            ("(list:any? #(eq? (% %1 2) 0) '(1))", "#f"),
            ("(list:any? #(eq? (% %1 2) 0) '(1 3))", "#f"),
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
            "(list:any? (fn (value) 1) '(1 2 3 4 5))",
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
            "(list:any? 1 '())",
            "(list:any? 1.1 '())",
            "(list:any? #t '())",
            "(list:any? #f '())",
            "(list:any? \"string\" '())",
            "(list:any? 'symbol '())",
            "(list:any? :keyword '())",
            "(list:any? '(1 2 3) '())",
            "(list:any? {} '())",

            "(list:any? (function (lambda (_1) nil)) 1)",
            "(list:any? (function (lambda (_1) nil)) 1.1)",
            "(list:any? (function (lambda (_1) nil)) #t)",
            "(list:any? (function (lambda (_1) nil)) #f)",
            "(list:any? (function (lambda (_1) nil)) \"string\")",
            "(list:any? (function (lambda (_1) nil)) 'symbol)",
            "(list:any? (function (lambda (_1) nil)) :keyword)",
            "(list:any? (function (lambda (_1) nil)) {})",
            "(list:any? (function (lambda (_1) nil)) #())"
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
            "(list:any?)",
            "(list:any? 1)",
            "(list:any? 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
