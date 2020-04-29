use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn flookup(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `flookup' must take exactly one string argument."
        ).into()
    }

    let mut values = values;

    let symbol_id = library::read_as_symbol_id(
        interpreter,
        values.remove(0)
    )?;

    match interpreter.lookup_function(
        _environment,
        symbol_id
    ) {
        Ok(value) => Ok(value),
        _ => return Error::generic_execution_error("")
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_associated_value() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(flet ((a () 1)) (flookup 'a))",
            "(flet ((a () 1)) (flookup 'flookup))"
        );

        assertion::assert_results_are_functions(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_nil_when_nothing_was_found() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(flet ((a () 1)) (flookup 'b))"
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
            "(flookup)",
            "(flookup 1 2)",
            "(flookup 1 2 3)"
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
            "(flookup 1)",
            "(flookup 1.0)",
            "(flookup #t)",
            "(flookup #f)",
            "(flookup \"string\")",
            "(flookup :keyword)",
            "(flookup '(s-expression))",
            "(flookup {})",
            "(flookup (function (lambda () 1)))",
            "(flookup (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
