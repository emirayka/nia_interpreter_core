use crate::parser::BooleanElement;

use crate::Error;
use crate::Value;

pub fn read_boolean_element(
    boolean_element: BooleanElement,
) -> Result<Value, Error> {
    Ok(Value::Boolean(boolean_element.get_value()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::nia_assert_equal;

    #[test]
    fn reads_boolean_elements_correctly() {
        let specs = vec![
            (Value::Boolean(true), BooleanElement::new(true)),
            (Value::Boolean(false), BooleanElement::new(false)),
        ];

        for (expected, boolean_element) in specs {
            let result = read_boolean_element(boolean_element);

            nia_assert_equal(Ok(expected), result);
        }
    }
}
