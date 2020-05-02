use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn string(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `to:string' takes one argument exactly."
        ).into();
    }

    let mut values = values;

    let string = library::value_to_string(
        interpreter,
        values.remove(0)
    )?;

    let string_value = interpreter.intern_string_value(&string);

    Ok(string_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(to:string 1)", "\"1\""),
            ("(to:string 1.1)", "\"1.1\""),
            ("(to:string #t)", "\"#t\""),
            ("(to:string #f)", "\"#f\""),
            ("(to:string \"string\")", "\"string\""),
            ("(to:string 'symbol)", "\"symbol\""),
            ("(to:string :keyword)", "\":keyword\""),
            ("(to:string nil)", "\"nil\""),
            ("(to:string '())", "\"nil\""),
            ("(to:string '(1 2 3))", "\"(1 2 3)\""),
            ("(to:string {})", "\"{}\""),
            ("(to:string #())", "\"<function>\""),
            ("(to:string (flookup 'flookup))", "\"<builtin-function>\""),
            ("(to:string (function (macro () 1)))", "\"<macro>\""),
            ("(to:string (flookup 'cond))", "\"<special-form>\""),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(to:string)",
            "(to:string 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
