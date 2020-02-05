use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::stdlib::special_forms::_lib::infect_special_form;

fn throw(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() > 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Special form `throw' must be called with no more than two arguments"
        ));
    }

    let mut values = values;

    let symbol = if values.len() > 0 {
        let value = values.remove(0);

        match value {
            Value::Symbol(symbol) => symbol,
            _ => return Err(Error::invalid_argument(
                interpreter,
                "The first argument of special form `throw' (if any) must be a symbol."
            ))
        }
    } else {
        interpreter.intern_symbol("generic-error")
    };

    let message = if values.len() > 0 {
        let value = values.remove(0);

        match value {
            Value::String(string) => string,
            _ => return Err(Error::invalid_argument(
                interpreter,
                "The second argument of special form `throw' (if any) must be a string."
            ))
        }
    } else {
        String::from("")
    };

    Err(Error::generic_error(
        symbol,
        &message
    ))
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_special_form(interpreter, "throw", throw)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;

    #[test]
    fn returns_generic_error_when_no_symbol_was_provided() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result= interpreter.execute("(throw)");

        assertion::assert_error(&result);

        let error = result.err().unwrap();

        assert_eq!(
            interpreter.intern_symbol("generic-error"),
            error.get_symbol().unwrap()
        );
    }

    #[test]
    fn returns_error_with_correct_symbol_when_it_was_provided() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result= interpreter.execute("(throw cute-error-symbol)");

        assertion::assert_error(&result);

        let error = result.err().unwrap();

        assert_eq!(
            interpreter.intern_symbol("cute-error-symbol"),
            error.get_symbol().unwrap()
        );
    }

    #[test]
    fn returns_error_with_correct_message_when_it_was_provided() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result= interpreter.execute("(throw cute-error-symbol \"Cute error message\")");

        assertion::assert_error(&result);

        let error = result.err().unwrap();

        assert_eq!(
            "Cute error message",
            error.get_message()
        );
    }
}
