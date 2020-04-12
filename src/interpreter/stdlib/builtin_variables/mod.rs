use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    registered_keyboards::infect(interpreter)?;

    Ok(())
}

mod registered_keyboards {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        let root_environment_id = interpreter.get_root_environment();
        let symbol_id = interpreter.intern("registered-keyboards");
        let value = interpreter.intern_nil_symbol_value();

        interpreter.define_variable(
            root_environment_id,
            symbol_id,
            value
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
