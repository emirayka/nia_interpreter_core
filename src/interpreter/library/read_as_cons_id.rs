use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::value::ConsId;
use crate::interpreter::error::Error;

pub fn read_as_cons_id(interpreter: &Interpreter, value: Value) -> Result<ConsId, Error> {
    let cons_id = match value {
        Value::Cons(cons_id) => cons_id,
        _ => return Error::invalid_argument_error(
            "Expected cons cell."
        ).into_result()
    };

    Ok(cons_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_cons_cell() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (Value::Cons(ConsId::new(1)), ConsId::new(1)),
            (Value::Cons(ConsId::new(2)), ConsId::new(2)),
            (Value::Cons(ConsId::new(3)), ConsId::new(3)),
        );

        for (value, expected) in pairs {
            let result = read_as_cons_id(
                &mut interpreter,
                value
            ).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn returns_invalid_argument_when_not_a_cons_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_string_values = vec!(
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value(String::from("test")),
            interpreter.intern_symbol_value("test"),
            interpreter.intern_keyword_value(String::from("test")),
            interpreter.make_object_value(),
            interpreter.execute("#(+ %1 %2)").unwrap()
        );

        for not_string_value in not_string_values {
            let result = read_as_cons_id(
                &mut interpreter,
                not_string_value
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
