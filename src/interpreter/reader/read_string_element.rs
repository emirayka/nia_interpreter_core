use crate::interpreter::parser::StringElement;

use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn read_string_element(
    interpreter: &mut Interpreter,
    string_element: StringElement,
) -> Result<Value, Error> {
    let string_name = string_element.get_value();
    let string_value = interpreter.intern_string_value(string_name);

    Ok(string_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;
    use std::convert::TryInto;

    #[test]
    fn reads_string_elements_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("", StringElement::new(String::from(""))),
            (
                "cute-string",
                StringElement::new(String::from("cute-string")),
            ),
            ("\n\r\t", StringElement::new(String::from("\n\r\t"))),
        ];

        for (string_name, string_element) in specs {
            let string =
                read_string_element(&mut interpreter, string_element).unwrap();
            let string_id = string.try_into().unwrap();
            let string = interpreter.get_string(string_id).unwrap();

            nia_assert_equal(string_name, string.get_string());
        }
    }
}
