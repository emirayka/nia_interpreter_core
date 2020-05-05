use crate::interpreter::reader::read_element::read_element;

use crate::parser::Element;

use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn read_elements(
    interpreter: &mut Interpreter,
    elements: Vec<Element>,
) -> Result<Vec<Value>, Error> {
    let mut result = Vec::new();

    for element in elements {
        let value = read_element(interpreter, element)?;

        result.push(value);
    }

    Ok(result)
}
