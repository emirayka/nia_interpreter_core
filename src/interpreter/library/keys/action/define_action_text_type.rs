use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn define_action_text_type<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    text_to_type: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let text_to_type = text_to_type.as_ref();

    let action_type_text_type_value =
        interpreter.intern_symbol_value("text-type");
    let text_to_type_value = interpreter.intern_string_value(text_to_type);

    let action_value = interpreter
        .vec_to_list(vec![action_type_text_type_value, text_to_type_value]);

    library::define_action(interpreter, action_name, action_value)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

    #[test]
    fn adds_execute_text_actions_to_action_alist() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "type-nia",
                r#"nia"#,
                r#"(list:new (cons:new "type-nia" (list:new 'text-type "nia")))"#,
            ),
            (
                "type-nya",
                r#"nya"#,
                r#"(list:new (cons:new "type-nya" (list:new 'text-type "nya")) (cons:new "type-nia" (list:new 'text-type "nia")))"#,
            ),
            (
                "type-eh",
                r#"eh"#,
                r#"(list:new (cons:new "type-eh" (list:new 'text-type "eh")) (cons:new "type-nya" (list:new 'text-type "nya")) (cons:new "type-nia" (list:new 'text-type "nia")))"#,
            ),
        ];

        for (action_name, text_to_type, expected) in specs {
            let expected =
                interpreter.execute_in_root_environment(expected).unwrap();

            nia_assert_is_ok(&define_action_text_type(
                &mut interpreter,
                action_name,
                text_to_type,
            ));

            let result = library::get_root_variable(
                &mut interpreter,
                DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
            )
            .unwrap();

            crate::utils::assert_deep_equal(&mut interpreter, expected, result);
        }
    }

    #[test]
    fn returns_error_when_action_with_that_name_already_defined() {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(&define_action_text_type(
            &mut interpreter,
            "type-kek",
            "kek",
        ));

        crate::utils::assert_generic_execution_error(&define_action_text_type(
            &mut interpreter,
            "type-kek",
            "kek",
        ));
    }
}
