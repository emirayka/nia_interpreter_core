use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn define_action_execute_code<S>(
    interpreter: &mut Interpreter,
    action_name: S,
    action_code: S,
) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let action_name = action_name.as_ref();
    let action_code = action_code.as_ref();

    let action_name_string_value = interpreter.intern_string_value(action_name);

    let action_type_execute_code_string_value =
        interpreter.intern_string_value("execute");
    let action_code_string_value = interpreter.intern_string_value(action_code);

    let action_value = interpreter.vec_to_list(vec![
        action_type_execute_code_string_value,
        action_code_string_value,
    ]);

    library::add_item_to_root_alist(
        interpreter,
        action_name_string_value,
        action_value,
        "",
    )
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn adds_execute_code_actions_to_action_alist() {}
}
