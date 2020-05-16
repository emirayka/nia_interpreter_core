use crate::interpreter::parser::FloatElement;

use crate::Error;
use crate::Value;

pub fn read_float_element(float_element: FloatElement) -> Result<Value, Error> {
    Ok(Value::Float(float_element.get_value()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::nia_assert_equal;

    #[test]
    fn reads_float_elements_correctly() {
        let specs = vec![
            (Value::Float(0.0), FloatElement::new(0.0)),
            (Value::Float(1.1), FloatElement::new(1.1)),
            (Value::Float(-11.1), FloatElement::new(-11.1)),
        ];

        for (expected, float_element) in specs {
            let result = read_float_element(float_element);

            nia_assert_equal(Ok(expected), result);
        }
    }
}
