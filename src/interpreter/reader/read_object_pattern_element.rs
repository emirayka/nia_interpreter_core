use crate::interpreter::parser::ObjectPatternElement;

use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn read_object_pattern_element(
    interpreter: &mut Interpreter,
    object_pattern_element: ObjectPatternElement,
) -> Result<Value, Error> {
    let values = object_pattern_element.get_values();

    let nil = interpreter.intern_nil_symbol_value();
    let mut last_cons = nil;
    let quote = interpreter.intern_symbol_value("quote");

    for keyword_element in values.into_iter().rev() {
        let name = keyword_element.get_value();

        let value = interpreter.intern_symbol_value(&name);
        let value_cell = interpreter.make_cons_value(value, nil);
        let quoted_value = interpreter.make_cons_value(quote, value_cell);
        let keyword = interpreter.intern_keyword_value(name);

        last_cons = Value::Cons(interpreter.make_cons(quoted_value, last_cons));

        last_cons = Value::Cons(interpreter.make_cons(keyword, last_cons));
    }

    let sym1 = interpreter.intern_symbol_value("object");
    let nil = interpreter.intern_nil_symbol_value();
    let car = Value::Cons(interpreter.make_cons(sym1, nil));

    let keyword = interpreter.intern_keyword_value("make");

    let car = Value::Cons(interpreter.make_cons(keyword, car));

    let cons_id = interpreter.make_cons(car, last_cons);

    Ok(Value::Cons(cons_id))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use std::convert::TryInto;

    use crate::interpreter::parser::KeywordElement;

    #[test]
    fn reads_elements_correctly() {
        let mut interpreter = Interpreter::new();

        let a_symbol_id = interpreter.intern_symbol_id("a");
        let b_symbol_id = interpreter.intern_symbol_id("b");
        let c_symbol_id = interpreter.intern_symbol_id("c");

        let a_symbol_value = a_symbol_id.into();
        let b_symbol_value = b_symbol_id.into();
        let c_symbol_value = c_symbol_id.into();

        let a_keyword_element = KeywordElement::new(String::from("a"));
        let b_keyword_element = KeywordElement::new(String::from("b"));
        let c_keyword_element = KeywordElement::new(String::from("c"));

        let specs = vec![
            (vec![], ObjectPatternElement::new(vec![])),
            (
                vec![(a_symbol_id, a_symbol_value)],
                ObjectPatternElement::new(vec![a_keyword_element.clone()]),
            ),
            (
                vec![
                    (a_symbol_id, a_symbol_value),
                    (b_symbol_id, b_symbol_value),
                ],
                ObjectPatternElement::new(vec![
                    a_keyword_element.clone(),
                    b_keyword_element.clone(),
                ]),
            ),
            (
                vec![
                    (a_symbol_id, a_symbol_value),
                    (b_symbol_id, b_symbol_value),
                    (c_symbol_id, c_symbol_value),
                ],
                ObjectPatternElement::new(vec![
                    a_keyword_element.clone(),
                    b_keyword_element.clone(),
                    c_keyword_element.clone(),
                ]),
            ),
        ];

        for (pairs, object_pattern_element) in specs {
            let object_id = read_object_pattern_element(
                &mut interpreter,
                object_pattern_element,
            )
            .unwrap();

            let object_id = interpreter
                .execute_value(interpreter.get_main_environment_id(), object_id)
                .unwrap()
                .try_into()
                .unwrap();

            for (key_symbol_id, expected_value) in pairs {
                let result_value = interpreter
                    .get_object_property(object_id, key_symbol_id)
                    .unwrap()
                    .unwrap();

                crate::utils::assert_deep_equal(
                    &mut interpreter,
                    expected_value,
                    result_value,
                );
            }
        }
    }
}
