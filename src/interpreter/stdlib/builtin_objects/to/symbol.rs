use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn symbol(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `to:symbol' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    match values.remove(0) {
        Value::String(string_id) => {
            let string = interpreter.get_string(string_id)?.get_string().clone();

            let symbol = interpreter.intern_symbol_value(&string);

            Ok(symbol)
        },
        symbol @ Value::Symbol(_) => Ok(symbol),
        _ => interpreter.make_invalid_argument_error(
            "Only keywords or strings can be casted to keyword."
        ).into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_string() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(to:symbol \"a\")", "'a"),
            ("(to:symbol \"string\")", "'string"),

            ("(to:symbol 'a)", "'a"),
            ("(to:symbol 'string)", "'string"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_generic_execution_error_when_invalid_conversion() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            "(to:symbol 1)",
            "(to:symbol 1.1)",
            "(to:symbol #t)",
            "(to:symbol #f)",
            "(to:symbol :keyword)",
            "(to:symbol '(1 2 3))",
            "(to:symbol {})",
            "(to:symbol #())",
            "(to:symbol (flookup 'flookup))",
            "(to:symbol (function (macro () 1)))",
            "(to:symbol (flookup 'cond))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(to:symbol)",
            "(to:symbol 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}