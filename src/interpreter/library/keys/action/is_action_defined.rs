use crate::Error;
use crate::Interpreter;
use crate::DEFINED_ACTIONS_ROOT_VARIABLE_NAME;

use crate::library;

pub fn is_action_defined<S>(
    interpreter: &mut Interpreter,
    action_name: S,
) -> Result<bool, Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let action_name_value = interpreter.intern_string_value(action_name);

    library::is_root_alist_has_key(
        interpreter,
        action_name_value,
        DEFINED_ACTIONS_ROOT_VARIABLE_NAME,
    )
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::Action;
    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn removes_defined_action() {
        let mut interpreter = Interpreter::new();

        library::define_action(
            &mut interpreter,
            "wait-1-sec",
            &Action::Wait(1000),
        )
        .unwrap();

        library::define_action(
            &mut interpreter,
            "wait-2-sec",
            &Action::Wait(2000),
        )
        .unwrap();

        library::define_action(
            &mut interpreter,
            "wait-3-sec",
            &Action::Wait(3000),
        )
        .unwrap();

        let specs = vec![
            (false, "wait-0-sec"),
            (true, "wait-1-sec"),
            (true, "wait-2-sec"),
            (true, "wait-3-sec"),
            (false, "wait-4-sec"),
        ];

        for (expected, action_name) in specs {
            let result =
                is_action_defined(&mut interpreter, action_name).unwrap();
            nia_assert_equal(expected, result);
        }
    }
}
