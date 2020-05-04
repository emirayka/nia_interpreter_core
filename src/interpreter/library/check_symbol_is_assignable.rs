use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::SymbolId;

pub fn check_symbol_is_assignable(
    interpreter: &Interpreter,
    symbol_id: SymbolId,
) -> Result<(), Error> {
    match interpreter.check_if_symbol_assignable(symbol_id) {
        Ok(true) => {}
        Ok(false) => return Error::invalid_argument_error("").into(),
        Err(error) => return Error::generic_execution_error_caused("", error).into(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_ok_on_ordinary_symbols() {
        let mut interpreter = Interpreter::new();
        let symbol_id = interpreter.intern("test");

        let result = check_symbol_is_assignable(&mut interpreter, symbol_id);

        nia_assert(result.is_ok());
    }

    #[test]
    fn returns_error_on_constants() {
        let mut interpreter = Interpreter::new();

        let mut specs = vec![
            // todo: remainder, when new constants will be introduced, add them here
            interpreter.intern("nil"),
            // todo: remainder, when new special symbols will be introduced, add them here
            interpreter.intern("#opt"),
            interpreter.intern("#rest"),
            interpreter.intern("#keys"),
            // todo: remainder, when new special variable will be introduced, add them here
            interpreter.intern("this"),
            interpreter.intern("super"),
        ];

        for spec in specs {
            let result = check_symbol_is_assignable(&mut interpreter, spec);

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_on_special_symbols() {
        let mut interpreter = Interpreter::new();

        let mut specs = vec![
            // todo: remainder, when new constants will be introduced, add them here
            interpreter.intern("nil"),
            // todo: remainder, when new special symbols will be introduced, add them here
            interpreter.intern("#opt"),
            interpreter.intern("#rest"),
            interpreter.intern("#keys"),
            // todo: remainder, when new special variable will be introduced, add them here
            interpreter.intern("this"),
            interpreter.intern("super"),
        ];

        for spec in specs {
            let symbol_id = spec;
            let result = check_symbol_is_assignable(&mut interpreter, symbol_id);

            assertion::assert_invalid_argument_error(&result);
        }
    }
}