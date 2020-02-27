use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn object_new(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() > 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `object:new' must take zero or one arguments."
        ).into_result();
    }

    let mut values = values;
    let object_id = interpreter.make_object();

    let proto_id = if values.len() > 0 {
        match values.remove(0) {
            Value::Object(proto_id) => Some(proto_id),
            _ => return interpreter.make_invalid_argument_error(
                "The first argument of `object:new' must be an object."
            ).into_result()
        }
    } else {
        None
    };

    match proto_id {
        Some(proto_id) => interpreter.set_object_proto(object_id, proto_id)
            .map_err(|err| interpreter.make_generic_execution_error_caused(
                "",
                err
            ))?,
        None => {}
    }

    Ok(Value::Object(object_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn makes_new_object() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(object:new)");

        assertion::assert_is_object(result.unwrap());
    }

    #[test]
    fn makes_new_object_with_a_proto() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((proto {})) (object:get-proto (object:new proto)))");

        assertion::assert_is_object(result.unwrap());
    }

    #[test]
    fn returns_invalid_argument_count_error_when_odd_count_of_arguments_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(object:new {} 1)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_an_even_argument_is_not_a_keyword() {
        let mut interpreter = Interpreter::new();

        let invalid_arguments = vec!(
            "(object:new 1)",
            "(object:new 1.1)",
            "(object:new #t)",
            "(object:new #f)",
            "(object:new 'symbol)",
            "(object:new \"string\")",
            "(object:new :keyword)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            invalid_arguments
        );
    }
}