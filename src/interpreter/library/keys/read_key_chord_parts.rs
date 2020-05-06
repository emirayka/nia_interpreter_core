use nia_events::KeyChordPart;

use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn read_modifiers(
    interpreter: &mut Interpreter,
) -> Result<Vec<KeyChordPart>, Error> {
    let modifiers_value = library::get_root_variable(interpreter, "modifiers")?;

    library::check_value_is_list(interpreter, modifiers_value)?;

    let modifiers_values =
        library::read_as_vector(interpreter, modifiers_value)?;
    let mut modifiers = Vec::new();

    for modifier_value in modifiers_values {
        let modifier =
            library::read_as_key_chord_part(interpreter, modifier_value)?;

        modifiers.push(modifier);
    }

    Ok(modifiers)
}
