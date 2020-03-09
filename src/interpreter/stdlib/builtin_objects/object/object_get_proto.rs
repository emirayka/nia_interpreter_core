use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn object_get_proto(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `object:get-proto' must take only one argument."
        ).into_result();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(
        interpreter,
        values.remove(0)
    )?;

    let proto_id = interpreter.get_object_proto(object_id)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    match proto_id {
        Some(proto_id) => Ok(Value::Object(proto_id)),
        None => Ok(interpreter.intern_nil_symbol_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_nil_when_no_proto_exists() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((obj {})) (object:get-proto obj))", interpreter.intern_nil_symbol_value())
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_proto_when_it_exists() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(let ((obj-1 {}) (obj-2 {})) (object:set-proto! obj-1 obj-2) (object:get-proto obj-1))"
        );

        assertion::assert_is_object(result.unwrap());
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj {:item 1})) (object:get-proto))",
            "(let ((obj {:item 1})) (object:get-proto obj 'smth-other))"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_when_first_argument_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj 2)) (object:get-proto obj))"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
