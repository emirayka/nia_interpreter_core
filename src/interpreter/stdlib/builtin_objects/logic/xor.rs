use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn xor(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `logic:xor' takes two arguments exactly."
        ).into();
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
        (true, true) => Ok(Value::Boolean(false)),
        (false, false) => Ok(Value::Boolean(false)),
        _ => Ok(Value::Boolean(true)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_xor_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(logic:xor #f #f)", "#f"),
            ("(logic:xor #f #t)", "#t"),
            ("(logic:xor #t #f)", "#t"),
            ("(logic:xor #t #t)", "#f"),
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
            "(logic:xor)",
            "(logic:xor #t)",
            "(logic:xor #t #t #t)",
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
            "(logic:xor 1 #t)",
            "(logic:xor 1.1 #t)",
            "(logic:xor 'symbol #t)",
            "(logic:xor \"string\" #t)",
            "(logic:xor :keyword #t)",
            "(logic:xor '(s-expression) #t)",
            "(logic:xor {} #t)",
            "(logic:xor (function (lambda () 1)) #t)",

            "(logic:xor #t 1)",
            "(logic:xor #t 1.1)",
            "(logic:xor #t 'symbol)",
            "(logic:xor #t \"string\")",
            "(logic:xor #t :keyword)",
            "(logic:xor #t '(s-expression))",
            "(logic:xor #t {})",
            "(logic:xor #t (function (lambda () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
