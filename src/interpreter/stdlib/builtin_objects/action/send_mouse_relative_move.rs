use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::PRIMITIVE_ACTIONS_VARIABLE_NAME;

pub fn send_mouse_relative_move(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-mouse-relative-move' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let move_relative_x = library::read_as_i64(values.remove(0))?;
    let move_relative_y = library::read_as_i64(values.remove(0))?;

    let mouse_move_relative_symbol_value =
        interpreter.intern_symbol_value("mouse-relative-move");
    let mouse_move_relative_list = interpreter.vec_to_list(vec![
        mouse_move_relative_symbol_value,
        Value::Integer(move_relative_x),
        Value::Integer(move_relative_y),
    ]);

    library::add_value_to_root_list(
        interpreter,
        PRIMITIVE_ACTIONS_VARIABLE_NAME,
        mouse_move_relative_list,
    )?;

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
    fn adds_action_to_action_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (PRIMITIVE_ACTIONS_VARIABLE_NAME, "'()"),
            (
                "(action:send-mouse-relative-move 2 3) nia-primitive-actions",
                "'((mouse-relative-move 2 3))",
            ),
            (
                "(action:send-mouse-relative-move 3 4) nia-primitive-actions",
                "'((mouse-relative-move 3 4) (mouse-relative-move 2 3))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-mouse-relative-move 1.1 1)",
            "(action:send-mouse-relative-move #t 1)",
            "(action:send-mouse-relative-move #f 1)",
            "(action:send-mouse-relative-move \"string\" 1)",
            "(action:send-mouse-relative-move 'symbol 1)",
            "(action:send-mouse-relative-move :keyword 1)",
            "(action:send-mouse-relative-move '(s-expression) 1)",
            "(action:send-mouse-relative-move {} 1)",
            "(action:send-mouse-relative-move #() 1)",
            "(action:send-mouse-relative-move 1 1.1)",
            "(action:send-mouse-relative-move 1 #t)",
            "(action:send-mouse-relative-move 1 #f)",
            "(action:send-mouse-relative-move 1 \"string\")",
            "(action:send-mouse-relative-move 1 'symbol)",
            "(action:send-mouse-relative-move 1 :keyword)",
            "(action:send-mouse-relative-move 1 '(s-expression))",
            "(action:send-mouse-relative-move 1 {})",
            "(action:send-mouse-relative-move 1 #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-mouse-relative-move)",
            "(action:send-mouse-relative-move 1)",
            "(action:send-mouse-relative-move 1 2 3)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
