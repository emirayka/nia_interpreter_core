use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn object_get_proto(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `object:get-proto' must take only one argument."
        );
    }

    let mut values = values;
    let object_id = match values.remove(0) {
        Value::Object(object_id) => object_id,
        _ => return interpreter.make_invalid_argument_error(
            "The first argument of built-in function `object:get-proto' must be an object."
        )
    };

    let proto_id = interpreter.get_object_proto(object_id);

    match proto_id {
        Some(proto_id) => Ok(Value::Object(proto_id)),
        None => Ok(interpreter.intern_nil_symbol_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_nil_when_no_proto_exists() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((obj {})) (object:get-proto obj))");

        assert_eq!(interpreter.intern_nil_symbol_value(), result.unwrap());
    }

    #[test]
    fn returns_proto_when_it_exists() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj-1 {}) (obj-2 {})) (object:set-proto! obj-1 obj-2) (object:get-proto obj-1))"
        );

        assert!(
            match result.unwrap() {
                Value::Object(_) => true,
                _ => false
            }
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:get-proto))"
        );
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:get-proto obj 'smth-other))"
        );
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_when_first_argument_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj 2)) (object:get-proto obj))"
        );
        assertion::assert_invalid_argument_error(&result);
    }
}
