use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn throw(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() > 2 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `throw' must be called with no more than two arguments"
        ).into_result();
    }

    let mut values = values;

    let symbol = if values.len() > 0 {
        let value = values.remove(0);

        match value {
            Value::Symbol(symbol) => symbol,
            _ => return interpreter.make_invalid_argument_error(
                "The first argument of special form `throw' (if any) must be a symbol."
            ).into_result()
        }
    } else {
        interpreter.intern("generic-error")
    };

    let message = if values.len() > 0 {
        let value = values.remove(0);

        let string = match value {
            Value::String(string_id) => interpreter.get_string(string_id),
            _ => return interpreter.make_invalid_argument_error(
                "The second argument of special form `throw' (if any) must be a string."
            ).into_result()
        };

        string
            .map(|string| String::from(string.get_string()))
            .map_err(|err| interpreter.make_generic_execution_error_caused(
                "Cannot yield a string",
                err
            ))?
    } else {
        String::from("")
    };

    let symbol_name = interpreter.get_symbol_name(
        symbol
    ).map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    interpreter.make_generic_error(
        symbol_name.clone(),
        &message
    ).into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_generic_error_when_no_symbol_was_provided() {
        let mut interpreter = Interpreter::new();

        let result= interpreter.execute("(throw)");
        assertion::assert_error(&result);

        let error = result.err().unwrap();

        assert_eq!(
            "generic-error",
            error.get_symbol_name()
        );
    }

    #[test]
    fn returns_error_with_correct_symbol_when_it_was_provided() {
        let mut interpreter = Interpreter::new();

        let result= interpreter.execute("(throw cute-error-symbol)");
        assertion::assert_error(&result);

        let error = result.err().unwrap();

        assert_eq!(
            "cute-error-symbol",
            error.get_symbol_name()
        );
    }

    #[test]
    fn returns_error_with_correct_message_when_it_was_provided() {
        let mut interpreter = Interpreter::new();

        let result= interpreter.execute("(throw cute-error-symbol \"Cute error message\")");
        assertion::assert_error(&result);

        let error = result.err().unwrap();

        assert_eq!(
            "Cute error message",
            error.get_message()
        );
    }
}
