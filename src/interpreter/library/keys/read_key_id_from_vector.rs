use nia_events::KeyId;

use crate::{Error, Value};

use crate::library;

pub fn read_key_id_from_vector(
    event_vector: Vec<Value>,
) -> Result<KeyId, Error> {
    if event_vector.len() == 0 {
        return Error::invalid_argument_error(
            "Invalid event description, expected key identifier.",
        )
        .into();
    }

    let mut event_vector = event_vector;

    let key_id =
        library::read_as_i64(event_vector.remove(0)).map_err(|error| {
            Error::invalid_argument_error_caused("Cannot parse key id.", error)
        })?;

    Ok(KeyId::new(key_id as u16))
}
