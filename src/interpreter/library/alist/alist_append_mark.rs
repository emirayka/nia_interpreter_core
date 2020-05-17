use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::SymbolId;
use crate::Value;

use crate::library;

pub fn alist_append_mark(
    interpreter: &mut Interpreter,
) -> Result<Value, Error> {
    let new_alist = interpreter.intern_nil_symbol_value();

    Ok(new_alist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_new_alist() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern_nil_symbol_value();
        let result = alist_new(&mut interpreter).unwrap();

        utils::assert_deep_equal(&mut interpreter, expected, result);
    }
}
