use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn object_get(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `object:get' must take even count of arguments."
        ).into_result();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(
        interpreter,
        values.remove(0)
    )?;

    let symbol_id = library::read_as_symbol_id(
        interpreter,
        values.remove(0)
    )?;

    library::check_if_symbol_assignable(interpreter, symbol_id)?;

    let value = interpreter.get_object_item(object_id, symbol_id)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    match value {
        Some(value) => Ok(value),
        // todo: must return something other than execution error
        None => {
            let message = &format!(
                "Cannot get item `{}' of object.",
                interpreter.get_symbol_name(symbol_id)?
            );

            return interpreter.make_generic_execution_error(
                message
            ).into_result()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::for_special_symbols;

    #[test]
    fn fetchs_item_of_object_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((obj {:a 1})) (object:get obj 'a))", Value::Integer(1))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_invalid_argument_when_attempt_get_item_by_special_symbol() {
        for_special_symbols(|interpreter, string| {
            let result = interpreter.execute(
                &(String::from("(let ((obj {:item 1})) (object:get obj '") + &string +"))")
            );
            assertion::assert_invalid_argument_error(&result);
        })
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj {:item 1})) (object:get))",
            "(let ((obj {:item 1})) (object:get obj))",
            "(let ((obj {:item 1})) (object:get obj 'item 'smth-else))"
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
            "(let ((obj 2)) (object:get obj 'item))"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_when_second_argument_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj {:a 1})) (object:get obj 2))"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_error_when_fetched_symbol_is_not_in_the_object() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj {:item 1})) (object:get obj 'not-item))"
        );

        assertion::assert_results_are_just_errors(
            &mut interpreter,
            code_vector
        );
    }
}
