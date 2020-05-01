use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn freeze_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:freeze!' must take only one argument."
        ).into();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(
        interpreter,
        values.remove(0)
    )?;

    let proto_id = match values.remove(0) {
        Value::Object(object_id) => object_id,
        _ => return Error::invalid_argument_error(
            "The first argument of built-in function `object:freeze!' must be an object."
        ).into()
    };

    interpreter.set_object_proto(object_id, proto_id)
        .map_err(|err| Error::generic_execution_error_caused(
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
    fn freezes_object() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(

        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(object:freeze!)",
            "(object:freeze! obj-1 'val)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_when_not_an_object_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(object:freeze! 1)",
            "(object:freeze! 1.1)",
            "(object:freeze! #t)",
            "(object:freeze! #f)",
            "(object:freeze! \"string\")",
            "(object:freeze! :keyword)",
            "(object:freeze! 'symbol)",
            "(object:freeze! '(list))",
            "(object:freeze! #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
