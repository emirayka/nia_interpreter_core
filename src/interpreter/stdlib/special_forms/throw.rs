use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn throw(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() > 2 {
        return Error::invalid_argument_count_error(
            "Special form `throw' must be called with no more than two arguments",
        )
        .into();
    }

    let mut values = values;

    let evaluated_first_argument = if values.len() > 0 {
        interpreter.execute_value(environment_id, values.remove(0))?
    } else {
        interpreter.intern_symbol_value("generic-error")
    };

    let symbol_id = library::read_as_symbol_id(evaluated_first_argument)?;

    let message = if values.len() > 0 {
        let value = values.remove(0);

        let string = match value {
            Value::String(string_id) => interpreter.get_string(string_id),
            _ => {
                return Error::invalid_argument_error(
                    "The second argument of special form `throw' (if any) must be a string.",
                )
                .into()
            }
        };

        string
            .map(|string| String::from(string.get_string()))
            .map_err(|err| Error::generic_execution_error_caused("Cannot yield a string", err))?
    } else {
        String::from("")
    };

    let symbol_name = interpreter
        .get_symbol_name(symbol_id)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    Error::generic_error(symbol_name.clone(), &message).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_generic_error_when_no_symbol_was_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute_in_main_environment("(throw)");
        nia_assert_is_err(&result);

        let error = result.err().unwrap();

        nia_assert_equal("generic-error", error.get_symbol_name());
    }

    #[test]
    fn works_inside_other_forms() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(when #t (throw 'err) 2)"];

        assertion::assert_results_are_just_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_error_with_correct_symbol_when_it_was_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute_in_main_environment("(throw 'cute-error-symbol)");
        nia_assert_is_err(&result);

        let error = result.err().unwrap();

        nia_assert_equal("cute-error-symbol", error.get_symbol_name());
    }

    #[test]
    fn returns_error_with_correct_message_when_it_was_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter
            .execute_in_main_environment("(throw 'cute-error-symbol \"Cute error message\")");
        nia_assert_is_err(&result);

        let error = result.err().unwrap();

        nia_assert_equal("Cute error message", error.get_message());
    }
}
