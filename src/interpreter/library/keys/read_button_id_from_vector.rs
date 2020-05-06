use nia_events::ButtonId;

use crate::{Error, Value};

use crate::library;

pub fn read_button_id_from_vector(
    event_vector: Vec<Value>,
) -> Result<ButtonId, Error> {
    if event_vector.len() == 0 {
        return Error::invalid_argument_error(
            "Invalid event description, expected button id",
        )
        .into();
    }

    let mut event_vector = event_vector;
    let button_id = library::read_as_i64(event_vector.remove(0))?;

    if button_id <= 0 && button_id > 8 {
        return Error::invalid_argument_error(
            "Invalid button id, expected interval [1-8], got: {}",
        )
        .into();
    }

    Ok(ButtonId::new(button_id as u16))
}
