use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn object_set(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 3 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `object:set!' must take even count of arguments."
        ).into_result();
    }

    let mut values = values;
    let object_id = match values.remove(0) {
        Value::Object(object_id) => object_id,
        _ => return interpreter.make_invalid_argument_error(
            "The first argument of built-in function `object:set!' must be an object."
        ).into_result()
    };

    let symbol_id = match values.remove(0) {
        Value::Symbol(symbol) => symbol,
        _ => return interpreter.make_invalid_argument_error(
            "The second argument of built-in function `object:set!' must be a symbol."
        ).into_result()
    };

    let value = values.remove(0);

    interpreter.set_object_item(
        object_id,
        symbol_id,
        value
    )?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn sets_item_to_object() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:a 1})) (object:set! obj 'a 2) (object:get obj 'a))"
        ).unwrap();

        assert_eq!(Value::Integer(2), result);
    }

    #[test]
    fn returns_value_that_were_set() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:a 1})) (object:set! obj 'a 2))"
        ).unwrap();

        assert_eq!(Value::Integer(2), result);
    }

    #[test]
    fn able_to_set_values_that_were_not_in_the_object_initially() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:a 1})) (object:set! obj 'b 2) (object:get obj 'b))"
        ).unwrap();

        assert_eq!(Value::Integer(2), result);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:set!))"
        );
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:set! obj))"
        );
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:set! obj 'item))"
        );
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:set! obj 'item 'sym2 'sym3))"
        );
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_when_first_argument_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj 2)) (object:set! obj 'item 2))"
        );
        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_invalid_argument_when_second_argument_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:a 1})) (object:set! obj 2 2))"
        );
        assertion::assert_invalid_argument_error(&result);
    }
}
