use crate::Error;
use crate::Interpreter;
use crate::NamedAction;
use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

use crate::library;

pub fn get_defined_actions(
    interpreter: &mut Interpreter,
) -> Result<Vec<NamedAction>, Error> {
    let actions = library::get_root_variable(
        interpreter,
        DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
    )?;

    let action_values = library::read_as_vector(interpreter, actions)?;

    let mut result = Vec::new();

    for action_value in action_values {
        let action_value_cons_id = library::read_as_cons_id(action_value)?;

        let action_name = interpreter.get_car(action_value_cons_id)?;
        let action_name =
            library::read_as_string(interpreter, action_name)?.clone();

        let action_list = interpreter.get_cdr(action_value_cons_id)?;
        let action = library::list_to_action(interpreter, action_list)?;

        result.push(NamedAction::new(action, action_name));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::Action;
    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn returns_defined_actions() {
        let mut interpreter = Interpreter::new();

        #[rustfmt::skip]
        let actions = vec![
            ("key-click", Action::KeyClick(33)),
            ("key-press", Action::KeyPress(33)),
            ("key-release", Action::KeyRelease(33)),
            
            ("mouse-button-click", Action::MouseButtonClick(1)),
            ("mouse-button-press", Action::MouseButtonPress(1)),
            ("mouse-button-release", Action::MouseButtonRelease(1)),
            
            ("mouse-absolute-move", Action::MouseAbsoluteMove(100, 100)),
            ("mouse-relative-move", Action::MouseRelativeMove(100, 100)),
            
            ("text-type", Action::TextType(String::from("cat"))),
            ("execute-code", Action::ExecuteCode(String::from(r#"(println "lambert, lambert")"#))),
            ("execute-function", Action::ExecuteFunction(String::from("function"))),
            ("execute-os-command", Action::ExecuteOSCommand(String::from(r#"echo "cat""#))),
            ("wait", Action::Wait(1000)),
        ];

        for (action_name, action) in actions {
            library::define_action(&mut interpreter, action_name, &action)
                .unwrap();
        }

        #[rustfmt::skip]
        let expected = vec![
            NamedAction::new(Action::Wait(1000), "wait"),
            NamedAction::new(Action::ExecuteOSCommand(String::from("echo \"cat\"")), "execute-os-command"),
            NamedAction::new(Action::ExecuteFunction(String::from("function")), "execute-function"),
            NamedAction::new(Action::ExecuteCode(String::from( "(println \"lambert, lambert\")")), "execute-code"),
            NamedAction::new(Action::TextType(String::from("cat")), "text-type"),
            
            NamedAction::new(Action::MouseRelativeMove(100, 100), "mouse-relative-move"),
            NamedAction::new(Action::MouseAbsoluteMove(100, 100), "mouse-absolute-move"),
            
            NamedAction::new(Action::MouseButtonRelease(1), "mouse-button-release"),
            NamedAction::new(Action::MouseButtonPress(1), "mouse-button-press"),
            NamedAction::new(Action::MouseButtonClick(1), "mouse-button-click"),
            
            NamedAction::new(Action::KeyRelease(33), String::from("key-release")),
            NamedAction::new(Action::KeyPress(33), "key-press"),
            NamedAction::new(Action::KeyClick(33), "key-click"),
        ];

        let result = get_defined_actions(&mut interpreter).unwrap();
        nia_assert_equal(expected.len(), result.len());

        for (expected, result) in expected.into_iter().zip(result.into_iter()) {
            nia_assert_equal(expected, result);
        }
    }
}
