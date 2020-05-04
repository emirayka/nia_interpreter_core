use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn get(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:get' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(values.remove(0))?;

    let symbol_id = library::read_keyword_or_symbol_as_symbol_id(interpreter, values.remove(0))?;

    library::check_symbol_is_assignable(interpreter, symbol_id)?;

    let value = interpreter
        .get_object_property(object_id, symbol_id)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    match value {
        Some(value) => Ok(value),
        // todo: must return something other than execution error
        None => {
            let message = &format!(
                "Cannot get item `{}' of object.",
                interpreter.get_symbol_name(symbol_id)?
            );

            return Error::generic_execution_error(message).into();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn fetchs_item_of_object_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (
                "(let ((obj {:a 1})) (object:get obj 'a))",
                Value::Integer(1),
            ),
            (
                "(let ((obj {:a 1})) (object:get obj :a))",
                Value::Integer(1),
            ),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_when_attempt_get_item_by_special_symbol() {
        let mut interpreter = Interpreter::new();

        let mut specs = vec![
            // todo: remainder, when new constants will be introduced, add them here
            "(let ((obj {:item 1})) (object:get obj 'nil))",
            // todo: remainder, when new special symbols will be introduced, add them here
            "(let ((obj {:item 1})) (object:get obj '#opt))",
            "(let ((obj {:item 1})) (object:get obj '#rest))",
            "(let ((obj {:item 1})) (object:get obj '#keys))",
            // todo: remainder, when new special variable will be introduced, add them here
            "(let ((obj {:item 1})) (object:get obj 'this))",
            "(let ((obj {:item 1})) (object:get obj 'super))",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, specs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(let ((obj {:item 1})) (object:get))",
            "(let ((obj {:item 1})) (object:get obj))",
            "(let ((obj {:item 1})) (object:get obj 'item 'smth-else))",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_when_arguments_are_invalid() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(let ((obj 2)) (object:get obj 'item))",
            "(let ((obj 2.2)) (object:get obj 'item))",
            "(let ((obj #t)) (object:get obj 'item))",
            "(let ((obj #f)) (object:get obj 'item))",
            "(let ((obj \"string\")) (object:get obj 'item))",
            "(let ((obj 'symbol)) (object:get obj 'item))",
            "(let ((obj :keyword)) (object:get obj 'item))",
            "(let ((obj '(1 2))) (object:get obj 'item))",
            "(let ((obj #())) (object:get obj 'item))",
            "(let ((obj {:a 1})) (object:get obj 2))",
            "(let ((obj {:a 1})) (object:get obj 2.2))",
            "(let ((obj {:a 1})) (object:get obj #t))",
            "(let ((obj {:a 1})) (object:get obj #f))",
            "(let ((obj {:a 1})) (object:get obj \"string\"))",
            "(let ((obj {:a 1})) (object:get obj '(list)))",
            "(let ((obj {:a 1})) (object:get obj {}))",
            "(let ((obj {:a 1})) (object:get obj #()))",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_error_when_fetched_symbol_is_not_in_the_object() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(let ((obj {:item 1})) (object:get obj 'not-item))"];

        assertion::assert_results_are_just_errors(&mut interpreter, code_vector);
    }
}
