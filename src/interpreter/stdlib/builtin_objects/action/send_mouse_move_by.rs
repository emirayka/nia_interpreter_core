use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn send_mouse_move_by(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `action:send-mouse-move-by' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let move_by_x = library::read_as_i64(values.remove(0))?;
    let move_by_y = library::read_as_i64(values.remove(0))?;

    let mouse_move_by_symbol_value =
        interpreter.intern_symbol_value("mouse-move-by");
    let mouse_move_by = interpreter.vec_to_list(vec![
        mouse_move_by_symbol_value,
        Value::Integer(move_by_x),
        Value::Integer(move_by_y),
    ]);

    library::add_value_to_root_list(interpreter, "--actions", mouse_move_by)?;

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
            ("--actions", "'()"),
            (
                "(action:send-mouse-move-by 2 3) --actions",
                "'((mouse-move-by 2 3))",
            ),
            (
                "(action:send-mouse-move-by 3 4) --actions",
                "'((mouse-move-by 3 4) (mouse-move-by 2 3))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(action:send-mouse-move-by 1.1 1)",
            "(action:send-mouse-move-by #t 1)",
            "(action:send-mouse-move-by #f 1)",
            "(action:send-mouse-move-by \"string\" 1)",
            "(action:send-mouse-move-by 'symbol 1)",
            "(action:send-mouse-move-by :keyword 1)",
            "(action:send-mouse-move-by '(s-expression) 1)",
            "(action:send-mouse-move-by {} 1)",
            "(action:send-mouse-move-by #() 1)",
            "(action:send-mouse-move-by 1 1.1)",
            "(action:send-mouse-move-by 1 #t)",
            "(action:send-mouse-move-by 1 #f)",
            "(action:send-mouse-move-by 1 \"string\")",
            "(action:send-mouse-move-by 1 'symbol)",
            "(action:send-mouse-move-by 1 :keyword)",
            "(action:send-mouse-move-by 1 '(s-expression))",
            "(action:send-mouse-move-by 1 {})",
            "(action:send-mouse-move-by 1 #())",
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
            "(action:send-mouse-move-by)",
            "(action:send-mouse-move-by 1)",
            "(action:send-mouse-move-by 1 2 3)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
