use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn lookup(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `lookup' must take exactly one string argument."
        ).into();
    }

    let mut values = values;

    let symbol_id = library::read_as_symbol_id(
        interpreter,
        values.remove(0)
    )?;

    match interpreter.lookup_variable(
        _environment,
        symbol_id
    )? {
        Some(value) => Ok(value),
        _ => Error::generic_execution_error("Cannot find variable.")
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_associated_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((a 1)) (lookup 'a))", Value::Integer(1))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_nil_when_nothing_was_found() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((a 1)) (lookup 'b))"
        );

        assertion::assert_results_are_just_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(lookup)",
            "(lookup 1 2)",
            "(lookup 1 2 3)"
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
            "(lookup 1)",
            "(lookup 1.0)",
            "(lookup #t)",
            "(lookup #f)",
            "(lookup \"string\")",
            "(lookup :keyword)",
            "(lookup '(s-expression))",
            "(lookup {})",
            "(lookup (function (lambda () 1)))",
            "(lookup (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
