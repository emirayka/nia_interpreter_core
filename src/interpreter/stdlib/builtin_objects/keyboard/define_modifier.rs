use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn define_modifier(
    _interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 1 || values.len() > 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `keyboard:define-modifier' takes one or two arguments exactly.",
        )
        .into();
    }

    // let mut values = values;

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn defines_new_modifiers() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("nia-defined-modifiers", "'()"),
            (
                "(keyboard:define-modifier \"LeftControl\" \"Control\")",
                "nil",
            ),
            ("nia-defined-modifiers", "'(29 \"Control\")"),
            ("(keyboard:define-modifier \"0:LeftMeta\" \"Meta\")", "nil"),
            ("nia-defined-modifiers", "'((0 125) (29 \"Control\"))"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(keyboard:define-modifier 1)",
            "(keyboard:define-modifier 1.1)",
            "(keyboard:define-modifier #t)",
            "(keyboard:define-modifier #f)",
            "(keyboard:define-modifier :keyword)",
            "(keyboard:define-modifier 'symbol)",
            "(keyboard:define-modifier '(list:new))",
            "(keyboard:define-modifier {})",
            "(keyboard:define-modifier #())",
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
            "(keyboard:define-modifier)",
            "(keyboard:define-modifier \"path\" \"name\")",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
