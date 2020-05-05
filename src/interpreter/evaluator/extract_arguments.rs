use crate::ConsId;
use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn extract_arguments(
    interpreter: &mut Interpreter,
    cons_id: ConsId,
) -> Result<Vec<Value>, Error> {
    let cdr = interpreter.get_cdr(cons_id)?;

    library::read_as_vector(interpreter, cdr)
}
