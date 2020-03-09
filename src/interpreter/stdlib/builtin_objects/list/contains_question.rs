use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn contains(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:contains?' takes two arguments exactly."
        ).into_result()
    }

    let mut values = values;

    let value_to_find = values.remove(0);

    let vector = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    for value in vector {
        if library::deep_equal(
            interpreter,
            value_to_find,
            value
        )? {
            return Ok(Value::Boolean(true))
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
            ("(list:contains? 1 '())", "#f"),
            ("(list:contains? 1 '(1))", "#t"),
            ("(list:contains? 1 '(1 2))", "#t"),
            ("(list:contains? 1 '(1 2 3))", "#t"),
            ("(list:contains? 1 '(1 2 3 4))", "#t"),

            ("(list:contains? 1 '(2))", "#f"),
            ("(list:contains? 1 '(2 4))", "#f"),

            ("(list:contains? 1 '(1))", "#t"),
            ("(list:contains? 1 '(3))", "#f"),
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
            "(list:contains? 1 1)",
            "(list:contains? 1 1.1)",
            "(list:contains? 1 #t)",
            "(list:contains? 1 #f)",
            "(list:contains? 1 \"string\")",
            "(list:contains? 1 'symbol)",
            "(list:contains? 1 :keyword)",
            "(list:contains? 1 {})",
            "(list:contains? 1 #())"
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
            "(list:contains?)",
            "(list:contains? 1)",
            "(list:contains? 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
