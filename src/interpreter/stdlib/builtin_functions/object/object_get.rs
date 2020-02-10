use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn object_get(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `object:get' must take even count of arguments."
        ));
    }

    let mut values = values;
    let object_id = match values.remove(0) {
        Value::Object(object_id) => object_id,
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first argument of built-in function `object:get' must be an object."
        ))
    };

    let symbol = match values.remove(0) {
        Value::Symbol(symbol) => symbol,
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The second argument of built-in function `object:get' must be a symbol."
        ))
    };

    let value = match interpreter.get_object_item(object_id, &symbol) {
        Some(value) => value,
        // todo: must return something other than execution error
        None => return Err(Error::generic_execution_error(
            interpreter,
            &format!("Cannot get item `{}' of object.", symbol.get_name())
        ))
    };

    Ok(value.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn fetchs_item_of_object_correctly() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(let ((obj {:a 1})) (object:get obj 'a))").unwrap()
        )
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:get))"
        );
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:get obj))"
        );
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:get obj 'item 'smth-else))"
        );
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_when_first_argument_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj 2)) (object:get obj 'item))"
        );
        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_invalid_argument_when_second_argument_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:a 1})) (object:get obj 2))"
        );
        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_error_when_fetched_symbol_is_not_in_the_object() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj {:item 1})) (object:get obj 'not-item))"
        );
        assertion::assert_error(&result);
    }
}
