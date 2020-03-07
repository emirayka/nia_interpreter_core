use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

use crate::interpreter::library;

pub fn object_set_proto(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `object:set-proto!' must take only one argument."
        ).into_result();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(
        interpreter,
        values.remove(0)
    )?;

    let proto_id = match values.remove(0) {
        Value::Object(object_id) => object_id,
        _ => return interpreter.make_invalid_argument_error(
            "The first argument of built-in function `object:set-proto!' must be an object."
        ).into_result()
    };

    interpreter.set_object_proto(object_id, proto_id)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn sets_proto_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((obj-1 {}) (obj-2 {:a 1})) (object:set-proto! obj-1 obj-2) obj-1:a)", Value::Integer(1))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_true_when_proto_is_set() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((obj-1 {}) (obj-2 {:a 1})) (object:set-proto! obj-1 obj-2))", Value::Boolean(true))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj-1 {}) (obj-2 {})) (object:set-proto!))",
            "(let ((obj-1 {}) (obj-2 {})) (object:set-proto! obj-1))",
            "(let ((obj-1 {}) (obj-2 {})) (object:set-proto! obj-1 obj-2 'sym2))"
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
            "(let ((obj-1 2) (obj-2 {})) (object:set-proto! obj-1 obj-2))"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_when_second_argument_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj-1 {}) (obj-2 2)) (object:set-proto! obj-1 obj-2))"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
