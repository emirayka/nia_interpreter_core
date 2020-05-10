use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::SymbolId;

use crate::library;

pub fn check_interned_symbol_is_expected(
    interpreter: &Interpreter,
    symbol_id: SymbolId,
    expected_symbol_name: &str,
) -> Result<(), Error> {
    library::check_symbol_is_expected(
        interpreter,
        symbol_id,
        expected_symbol_name,
        0,
    )?;

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
            (interpreter.intern_symbol_id("test"), "test"),
            (interpreter.intern_symbol_id("test-2"), "test-2"),
        ];

        for (symbol_id, expected_symbol_name) in specs {
            let result = check_interned_symbol_is_expected(
                &mut interpreter,
                symbol_id,
                expected_symbol_name,
            );
            nia_assert_is_ok(&result);
        }
    }

    #[test]
    fn returns_error_on_unexpected_symbols() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (interpreter.intern_symbol_id("test"), "wat"),
            (interpreter.gensym("test"), "test"),
            (interpreter.gensym("test"), "test"),
        ];

        for (symbol_id, expected_symbol_name) in specs {
            let result = check_interned_symbol_is_expected(
                &mut interpreter,
                symbol_id,
                expected_symbol_name,
            );
            nia_assert_is_err(&result);
        }
    }
}
