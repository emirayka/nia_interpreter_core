use crate::interpreter::parser::SymbolElement;

use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn read_symbol_element(
    interpreter: &mut Interpreter,
    symbol_element: SymbolElement,
) -> Result<Value, Error> {
    let symbol_name = symbol_element.get_value();
    let symbol_value = interpreter.intern_symbol_value(symbol_name);

    Ok(symbol_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn reads_symbol_elements_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                ("cute-symbol", 0),
                SymbolElement::new(String::from("cute-symbol")),
            ),
            (
                ("cute-symbol", 0),
                SymbolElement::new(String::from("cute-symbol")),
            ),
            (("#opt", 0), SymbolElement::new(String::from("#opt"))),
            (("#rest", 0), SymbolElement::new(String::from("#rest"))),
            (("#keys", 0), SymbolElement::new(String::from("#keys"))),
        ];

        for ((symbol_name, symbol_gensym_id), symbol_element) in specs {
            let symbol =
                read_symbol_element(&mut interpreter, symbol_element).unwrap();
            let symbol_id = symbol.try_into().unwrap();
            let symbol = interpreter.get_symbol(symbol_id).unwrap();

            nia_assert_equal(symbol_name, symbol.get_name());
            nia_assert_equal(symbol_gensym_id, symbol.get_gensym_id());
        }
    }
}
