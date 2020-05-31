use crate::Error;
use crate::Interpreter;
use crate::Mapping;

use crate::library;
use crate::GLOBAL_MAP_ROOT_VARIABLE_NAME;

pub fn define_global_mapping(
    interpreter: &mut Interpreter,
    mapping: &Mapping,
) -> Result<(), Error> {
    let key_chord_sequence = mapping.get_key_chords();
    let action = mapping.get_action();

    if !library::is_mapping_can_be_defined(interpreter, key_chord_sequence)? {
        return Error::generic_execution_error("Mapping cannot be defined.")
            .into();
    }

    let key_chords_sequence_value =
        library::key_chords_to_list(interpreter, key_chord_sequence);
    let action = library::action_to_list(interpreter, action)?;

    library::add_item_to_root_alist(
        interpreter,
        key_chords_sequence_value,
        action,
        GLOBAL_MAP_ROOT_VARIABLE_NAME,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::Action;
    use crate::KeyChord;

    use crate::utils;

    #[test]
    fn adds_new_global_mapping() {
        let mut interpreter = Interpreter::new();

        let specs = vec![(
            r#"(list:new (cons:new '((1 2 3) ((1 4) (1 5) (2 6))) '(execute-code "(println \"Hello :3\")")))"#,
            vec![
                KeyChord::new(vec![nia_key!(1), nia_key!(2)], nia_key!(3)),
                KeyChord::new(
                    vec![nia_key!(1, 4), nia_key!(1, 5)],
                    nia_key!(2, 6),
                ),
            ],
            Action::ExecuteCode(String::from(r#"(println "Hello :3")"#)),
        )];

        for (expected, key_chord_sequence, action) in specs {
            let expected =
                interpreter.execute_in_main_environment(expected).unwrap();

            let mapping = Mapping::new(key_chord_sequence, action);

            nia_assert_is_ok(&define_global_mapping(
                &mut interpreter,
                &mapping,
            ));

            let result = library::get_root_variable(
                &mut interpreter,
                GLOBAL_MAP_ROOT_VARIABLE_NAME,
            )
            .unwrap();

            utils::assert_deep_equal(&mut interpreter, expected, result);
        }
    }
}
