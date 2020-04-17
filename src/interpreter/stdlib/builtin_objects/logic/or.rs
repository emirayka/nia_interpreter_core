use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn or(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `logic:or' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let v1 = library::read_as_bool(
        interpreter,
        values.remove(0)
    )?;

    let v2 = library::read_as_bool(
        interpreter,
        values.remove(0)
    )?;

    match (v1, v2) {
        (false, false) => Ok(Value::Boolean(false)),
        _ => Ok(Value::Boolean(true)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_or_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(logic:or #f #f)", "#f"),
            ("(logic:or #f #t)", "#t"),
            ("(logic:or #t #f)", "#t"),
            ("(logic:or #t #t)", "#t"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_invalid_argument_count_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(logic:or)",
            "(logic:or #t)",
            "(logic:or #t #t #t)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(logic:or 1 #t)",
            "(logic:or 1.1 #t)",
            "(logic:or 'symbol #t)",
            "(logic:or \"string\" #t)",
            "(logic:or :keyword #t)",
            "(logic:or '(s-expression) #t)",
            "(logic:or {} #t)",
            "(logic:or (function (lambda () 1)) #t)",

            "(logic:or #t 1)",
            "(logic:or #t 1.1)",
            "(logic:or #t 'symbol)",
            "(logic:or #t \"string\")",
            "(logic:or #t :keyword)",
            "(logic:or #t '(s-expression))",
            "(logic:or #t {})",
            "(logic:or #t (function (lambda () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
