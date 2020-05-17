use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::SymbolId;

pub fn check_symbol_is_expected(
    interpreter: &Interpreter,
    symbol_id: SymbolId,
    expected_symbol_name: &str,
    expected_symbol_gensym_id: usize,
) -> Result<(), Error> {
    let symbol = interpreter.get_symbol(symbol_id)?;

    if symbol.get_name() != expected_symbol_name
        || symbol.get_gensym_id() != expected_symbol_gensym_id
    {
        return Error::generic_execution_error(&format!(
            "Expected symbol ({}, {}), got ({}, {}).",
            expected_symbol_name,
            expected_symbol_gensym_id,
            symbol.get_name(),
            symbol.get_gensym_id(),
        ))
        .into();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_ok_on_expected_symbols() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (interpreter.intern_symbol_id("test"), "test", 0),
            (interpreter.intern_symbol_id("test-2"), "test-2", 0),
            (interpreter.gensym("test-3"), "test-3", 1),
            (interpreter.gensym("test-3"), "test-3", 2),
        ];

        for (symbol_id, expected_symbol_name, expected_symbol_gensym_id) in
            specs
        {
            let result = check_symbol_is_expected(
                &mut interpreter,
                symbol_id,
                expected_symbol_name,
                expected_symbol_gensym_id,
            );
            nia_assert_is_ok(&result);
        }
    }

    #[test]
    fn returns_error_on_unexpected_symbols() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (interpreter.intern_symbol_id("test"), "wat", 0),
            (interpreter.intern_symbol_id("wat"), "wat", 1),
        ];

        for (symbol_id, expected_symbol_name, expected_symbol_gensym_id) in
            specs
        {
            let result = check_symbol_is_expected(
                &mut interpreter,
                symbol_id,
                expected_symbol_name,
                expected_symbol_gensym_id,
            );
            nia_assert_is_err(&result);
        }
    }
}
