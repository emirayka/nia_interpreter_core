use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::symbol::SymbolId;
use crate::interpreter::error::Error;

pub fn check_if_symbol_assignable(
    interpreter: &Interpreter,
    symbol_id: SymbolId
) -> Result<(), Error> {
    match interpreter.check_if_symbol_assignable(symbol_id) {
        Ok(true) => {},
        Ok(false) => return interpreter.make_invalid_argument_error("").into_result(),
        Err(error) => return interpreter.make_generic_execution_error_caused(
            "",
            error
        ).into_result()
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::testing_helpers::{for_constants, for_special_symbols};
    use crate::interpreter::library::assertion;

    // todo: ensure this test is fine
    #[test]
    fn returns_ok_on_ordinary_symbols() {
        let mut interpreter = Interpreter::new();
        let symbol_id = interpreter.intern("test");

        let result = check_if_symbol_assignable(&mut interpreter, symbol_id);

        assert!(result.is_ok());
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_on_constants() {
        for_constants(|interpreter, string| {
            let symbol_id = interpreter.intern(&string);

            let result = check_if_symbol_assignable(interpreter, symbol_id);

            assertion::assert_invalid_argument_error(&result);
        })
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_on_special_symbols() {
        for_special_symbols(|interpreter, string| {
            let symbol_id = interpreter.intern(&string);

            let result = check_if_symbol_assignable(interpreter, symbol_id);

            assertion::assert_invalid_argument_error(&result);
        })
    }
}
