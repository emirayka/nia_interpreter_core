use nia_events::KeyChord;

use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn read_mappings(
    interpreter: &mut Interpreter,
) -> Result<Vec<(Vec<KeyChord>, Value)>, Error> {
    let mappings = library::get_root_variable(interpreter, "global-map")?;

    let mappings_values = library::read_as_vector(interpreter, mappings)?;
    let mut mappings = Vec::new();

    for mapping_value in mappings_values {
        let mapping = library::read_mapping(interpreter, mapping_value)?;

        mappings.push(mapping);
    }

    Ok(mappings)
}
