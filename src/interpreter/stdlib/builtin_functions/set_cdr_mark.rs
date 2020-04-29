use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn set_cdr_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `set-cdr!' must take exactly one argument."
        ).into();
    }

    let mut values = values;

    let cons_id = library::read_as_cons_id(
        interpreter,
        values.remove(0)
    )?;

    let value = values.remove(0);

    interpreter.set_cdr(cons_id, value)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn sets_cdr() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((c (cons 1 2))) (cdr c))", "2"),
            ("(let ((c (cons 1 2))) (set-cdr! c 3) (cdr c))", "3"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(set-cdr!)",
            "(set-cdr! (cons 1 2))",
            "(set-cdr! (cons 1 2) 3 4)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_called_with_a_value_that_is_not_cons() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(set-cdr! 1 1)",
            "(set-cdr! 1.1 1)",
            "(set-cdr! #t 1)",
            "(set-cdr! #f 1)",
            "(set-cdr! \"string\" 1)",
            "(set-cdr! 'symbol 1)",
            "(set-cdr! :keyword 1)",
            "(set-cdr! {} 1)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
