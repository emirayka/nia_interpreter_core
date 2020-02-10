use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn object_set_proto(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `object:set-proto!' must take only one argument."
        ));
    }

    let mut values = values;
    let object_id = match values.remove(0) {
        Value::Object(object_id) => object_id,
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first argument of built-in function `object:set-proto!' must be an object."
        ))
    };

    let proto_id = match values.remove(0) {
        Value::Object(object_id) => object_id,
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first argument of built-in function `object:set-proto!' must be an object."
        ))
    };

    interpreter.set_object_proto(object_id, proto_id);

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn sets_proto_correctly() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj-1 {}) (obj-2 {:a 1})) (object:set-proto! obj-1 obj-2) obj-1:a)"
        );

        assert_eq!(Value::Integer(1), result.unwrap());
    }

    #[test]
    fn returns_true_when_proto_is_set() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj-1 {}) (obj-2 {:a 1})) (object:set-proto! obj-1 obj-2))"
        );

        assert_eq!(Value::Boolean(true), result.unwrap());
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj-1 {}) (obj-2 {})) (object:set-proto!))"
        );
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute(
            "(let ((obj-1 {}) (obj-2 {})) (object:set-proto! obj-1))"
        );
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute(
            "(let ((obj-1 {}) (obj-2 {})) (object:set-proto! obj-1 obj-2 'sym2))"
        );
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_when_first_argument_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj-1 2) (obj-2 {})) (object:set-proto! obj-1 obj-2))"
        );
        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_invalid_argument_when_second_argument_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
        "(let ((obj-1 {}) (obj-2 2)) (object:set-proto! obj-1 obj-2))"
        );
        assertion::assert_invalid_argument_error(&result);
    }
}
