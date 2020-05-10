use std::convert::TryInto;

use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn read_keyboards(
    interpreter: &mut Interpreter,
) -> Result<Vec<(String, String)>, Error> {
    let registered_keyboards = library::get_registered_keyboards(interpreter)?;

    library::check_value_is_cons(interpreter, registered_keyboards)?;

    let registered_keyboards =
        interpreter.list_to_vec(registered_keyboards.try_into()?)?;

    let mut keyboards = Vec::new();

    for registered_keyboard in registered_keyboards {
        library::check_value_is_cons(interpreter, registered_keyboard)?;

        let registered_keyboard_cons_id = registered_keyboard.try_into()?;

        let registered_keyboard =
            interpreter.list_to_vec(registered_keyboard_cons_id)?;

        let path =
            library::read_as_string(interpreter, registered_keyboard[0])?;
        let name =
            library::read_as_string(interpreter, registered_keyboard[1])?;

        keyboards.push((path.clone(), name.clone()))
    }

    Ok(keyboards)
}
