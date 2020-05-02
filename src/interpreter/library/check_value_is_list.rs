use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;

pub fn check_value_is_list(
    interpreter: &Interpreter,
    value: Value
) -> Result<(), Error> {
    match value {
        Value::Cons(_) => Ok(()),
        Value::Symbol(symbol_id) => {
            if interpreter.symbol_is_nil(symbol_id)? {
                Ok(())
            } else {
                Error::invalid_argument_error("Expected list")
                    .into()
            }
        },
        _ => Error::invalid_argument_error("Expected list")
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;
    use crate::interpreter::value::ConsId;

    #[test]
    fn returns_nothing_when_a_cons_was_passed() {
        let mut interpreter = Interpreter::new();

        let list_values = vec!(
            Value::Cons(ConsId::new(0)),
            interpreter.intern_nil_symbol_value()
        );

        for list_value in list_values {
            let result = check_value_is_list(
                &mut interpreter,
                list_value
            ).unwrap();

            nia_assert_equal((), result);
        }
    }

    #[test]
    fn returns_invalid_argument_when_not_a_cons_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_list_values = vec!(
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_symbol_value("test"),
            interpreter.intern_string_value("test"),
            interpreter.intern_keyword_value("test"),
            interpreter.make_object_value(),
            interpreter.execute("#(+ %1 %2)").unwrap()
        );

        for not_list_value in not_list_values {
            let result = check_value_is_list(
                &mut interpreter,
                not_list_value
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
