use crate::Interpreter;
use crate::Value;
use crate::{Error, DEFINED_MODIFIERS_ROOT_VARIABLE_NAME};

use crate::library;

pub fn get_defined_modifiers_as_value(
    interpreter: &mut Interpreter,
) -> Result<Value, Error> {
    let modifiers = library::get_root_variable(
        interpreter,
        DEFINED_MODIFIERS_ROOT_VARIABLE_NAME,
    )
    .map_err(|err| {
        Error::generic_execution_error_caused(
            "Cannot read registered keyboards.",
            err,
        )
    })?;

    Ok(modifiers)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::ModifierDescription;
    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn returns_defined_modifiers_as_value() {
        let mut interpreter = Interpreter::new();

        let result = get_defined_modifiers_as_value(&mut interpreter).unwrap();
        let expected =
            interpreter.execute_in_main_environment(r#"'()"#).unwrap();
        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let modifier = nia_modifier!(3, 22, "");
        nia_assert_is_ok(&library::define_modifier(
            &mut interpreter,
            &modifier,
        ));

        let result = get_defined_modifiers_as_value(&mut interpreter).unwrap();
        let expected = interpreter
            .execute_in_main_environment(r#"'((3 22 ()))"#)
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);

        let modifier = nia_modifier!(2, 33, "mod");
        nia_assert_is_ok(&library::define_modifier(
            &mut interpreter,
            &modifier,
        ));

        let result = get_defined_modifiers_as_value(&mut interpreter).unwrap();
        let expected = interpreter
            .execute_in_main_environment(r#"'((2 33 "mod") (3 22 ()))"#)
            .unwrap();

        crate::utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
