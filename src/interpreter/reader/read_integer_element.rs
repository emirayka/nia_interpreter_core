use crate::parser::IntegerElement;

use crate::Error;
use crate::Value;

pub fn read_integer_element(
    integer_element: IntegerElement,
) -> Result<Value, Error> {
    Ok(Value::Integer(integer_element.get_value()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::nia_assert_equal;

    #[test]
    fn reads_integer_elements_correctly() {
        let specs = vec![
            (Value::Integer(1), IntegerElement::new(1)),
            (Value::Integer(0), IntegerElement::new(0)),
            (Value::Integer(-1), IntegerElement::new(-1)),
        ];

        for (expected, integer_element) in specs {
            let result = read_integer_element(integer_element);

            nia_assert_equal(Ok(expected), result);
        }
    }
}
