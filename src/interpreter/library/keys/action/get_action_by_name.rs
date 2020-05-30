use crate::Action;
use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn get_action_by_name<S>(
    interpreter: &mut Interpreter,
    action_name: S,
) -> Result<Action, Error>
where
    S: AsRef<str>,
{
    let actions = library::get_defined_actions(interpreter)?;

    for action in actions {
        if action.get_action_name() == action_name.as_ref() {
            return Ok(action.take_action());
        }
    }

    Error::generic_execution_error("Action was not found").into()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::NamedAction;
    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn returns_defined_action() {
        let mut interpreter = Interpreter::new();

        let action_1 = NamedAction::new(Action::Wait(1000), "action-1");
        let action_2 = NamedAction::new(Action::Wait(2000), "action-2");
        let action_3 = NamedAction::new(Action::Wait(3000), "action-3");

        nia_assert_is_ok(&library::define_action(
            &mut interpreter,
            action_1.get_action_name(),
            action_1.get_action(),
        ));
        nia_assert_is_ok(&library::define_action(
            &mut interpreter,
            action_2.get_action_name(),
            action_2.get_action(),
        ));
        nia_assert_is_ok(&library::define_action(
            &mut interpreter,
            action_3.get_action_name(),
            action_3.get_action(),
        ));

        nia_assert_equal(
            action_1.get_action(),
            &library::get_action_by_name(
                &mut interpreter,
                action_1.get_action_name(),
            )
            .unwrap(),
        );
        nia_assert_equal(
            action_2.get_action(),
            &library::get_action_by_name(
                &mut interpreter,
                action_2.get_action_name(),
            )
            .unwrap(),
        );
        nia_assert_equal(
            action_3.get_action(),
            &library::get_action_by_name(
                &mut interpreter,
                action_3.get_action_name(),
            )
            .unwrap(),
        );
    }

    #[test]
    fn returns_error_when_action_were_not_defined() {
        let mut interpreter = Interpreter::new();

        nia_assert_is_err(&get_action_by_name(
            &mut interpreter,
            "not-existing-action",
        ))
    }
}
