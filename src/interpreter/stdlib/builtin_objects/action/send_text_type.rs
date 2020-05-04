use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn send_text_type(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-text-type' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;

    let text = library::read_as_string_id(values.remove(0))?;

    let text_type_symbol_value = interpreter.intern_symbol_value("text-type");
    let text_type = interpreter.vec_to_list(vec![text_type_symbol_value, Value::String(text)]);

    library::add_value_to_root_list(interpreter, "--actions", text_type)?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn adds_action_to_action_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("--actions", "'()"),
            (
                "(action:send-text-type \"first\") --actions",
                "'((text-type \"first\"))",
            ),
            (
                "(action:send-text-type \"second\") --actions",
                "'((text-type \"second\") (text-type \"first\"))",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-text-type 1)",
            "(action:send-text-type 1.1)",
            "(action:send-text-type #t)",
            "(action:send-text-type #f)",
            "(action:send-text-type 'symbol)",
            "(action:send-text-type :keyword)",
            "(action:send-text-type '(s-expression))",
            "(action:send-text-type {})",
            "(action:send-text-type #())",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-text-type)",
            "(action:send-text-type \"at\" 3)",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
