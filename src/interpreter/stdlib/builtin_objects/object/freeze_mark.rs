use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn freeze_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:freeze!' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(values.remove(0))?;

    let mut object = interpreter.get_object_mut(object_id)?;

    object.freeze()?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn freezes_object() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            // todo: probably change generic execution error to another error symbol
            (
                "(let ((a {})) (object:freeze! a) (object:is-frozen? a))",
                "#t",
            ),
            (
                "(try (let ((a {})) (object:freeze! a) (object:set! a :kek 2)) (catch 'generic-execution-error #t))",
                "#t",
            ),
            (
                "(try (let ((a {:kek nil})) (object:freeze! a) (object:set-proto! a {})) (catch 'generic-execution-error #t))",
                "#t",
            ),
            (
                "(try (let ((a {:kek nil})) (object:freeze! a) (object:set! a :kek 2)) (catch 'generic-execution-error #t))",
                "#t",
            ),
            (
                "(try (let ((a {:kek nil})) (object:freeze! a) (object:delete-property a :kek)) (catch 'generic-execution-error #t))",
                "#t",
            ),
            (
                "(try (let ((a {:kek nil})) (object:freeze! a) (object:set-internable! a :kek #f)) (catch 'generic-execution-error #t))",
                "#t",
            ),
            (
                "(try (let ((a {:kek nil})) (object:freeze! a) (object:set-writable! a :kek #f)) (catch 'generic-execution-error #t))",
                "#t",
            ),
            (
                "(try (let ((a {:kek nil})) (object:freeze! a) (object:set-enumerable! a :kek #f)) (catch 'generic-execution-error #t))",
                "#t",
            ),
            (
                "(try (let ((a {:kek nil})) (object:freeze! a) (object:set-configurable! a :kek #f)) (catch 'generic-execution-error #t))",
                "#t",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(object:freeze!)", "(object:freeze! {} 'val)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_when_not_an_object_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:freeze! 1)",
            "(object:freeze! 1.1)",
            "(object:freeze! #t)",
            "(object:freeze! #f)",
            "(object:freeze! \"string\")",
            "(object:freeze! :keyword)",
            "(object:freeze! 'symbol)",
            "(object:freeze! '(list))",
            "(object:freeze! #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
