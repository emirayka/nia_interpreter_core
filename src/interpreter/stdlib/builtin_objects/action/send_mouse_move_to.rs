use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn send_mouse_move_to(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-mouse-move-to' takes two arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let mut move_to_x = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    let mut move_to_y = library::read_as_i64(
        interpreter,
        values.remove(0)
    )?;

    let mouse_move_to_symbol_value = interpreter.intern_symbol_value("mouse-move-to");
    let mouse_move_to = interpreter.vec_to_list(vec!(
        mouse_move_to_symbol_value,
        Value::Integer(move_to_x),
        Value::Integer(move_to_y)
    ));

    library::add_value_to_root_list(
        interpreter,
        "--actions",
        mouse_move_to
    )?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn adds_action_to_action_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("--actions", "'()"),
            ("(action:send-mouse-move-to 2 3) --actions", "'((mouse-move-to 2 3))"),
            ("(action:send-mouse-move-to 3 4) --actions", "'((mouse-move-to 3 4) (mouse-move-to 2 3))"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(action:send-mouse-move-to 1.1 1)",
            "(action:send-mouse-move-to #t 1)",
            "(action:send-mouse-move-to #f 1)",
            "(action:send-mouse-move-to \"string\" 1)",
            "(action:send-mouse-move-to 'symbol 1)",
            "(action:send-mouse-move-to :keyword 1)",
            "(action:send-mouse-move-to '(s-expression) 1)",
            "(action:send-mouse-move-to {} 1)",
            "(action:send-mouse-move-to #() 1)",

            "(action:send-mouse-move-to 1 1.1)",
            "(action:send-mouse-move-to 1 #t)",
            "(action:send-mouse-move-to 1 #f)",
            "(action:send-mouse-move-to 1 \"string\")",
            "(action:send-mouse-move-to 1 'symbol)",
            "(action:send-mouse-move-to 1 :keyword)",
            "(action:send-mouse-move-to 1 '(s-expression))",
            "(action:send-mouse-move-to 1 {})",
            "(action:send-mouse-move-to 1 #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(action:send-mouse-move-to)",
            "(action:send-mouse-move-to 1)",
            "(action:send-mouse-move-to 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
