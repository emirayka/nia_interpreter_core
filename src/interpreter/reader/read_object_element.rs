use crate::interpreter::reader::read_element::read_element;
use crate::parser::ObjectElement;
use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn read_object_element(
    interpreter: &mut Interpreter,
    object_element: ObjectElement,
) -> Result<Value, Error> {
    let values = object_element.get_values();

    let nil = interpreter.intern_nil_symbol_value();
    let mut last_cons = nil;

    for (keyword_element, element) in values.into_iter().rev() {
        let name = keyword_element.get_value();

        let value = read_element(interpreter, element)?;
        let keyword = interpreter.intern_keyword_value(name);

        last_cons = Value::Cons(interpreter.make_cons(value, last_cons));

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
    use super::*;

    #[cfg(test)]
    mod object {
        use super::*;

        use crate::parser::BooleanElement;
        use crate::parser::Element;
        use crate::parser::FloatElement;
        use crate::parser::IntegerElement;
        use crate::parser::KeywordElement;
        use crate::parser::StringElement;
        use crate::parser::SymbolElement;

        use crate::BuiltinFunction;
        use crate::EnvironmentId;
        use crate::Function;

        #[allow(unused_imports)]
        use nia_basic_assertions::*;
        use std::convert::TryInto;

        #[test]
        fn reads_object_element_correctly() {
            let mut interpreter = Interpreter::new();

            let integer_value = Value::Integer(1);
            let float_value = Value::Float(1.1);
            let true_value = Value::Boolean(true);
            let false_value = Value::Boolean(false);

            let specs = vec![
                (vec![], ObjectElement::new(vec![])),
                (
                    vec![
                        (interpreter.intern_symbol_id("a"), integer_value),
                        (interpreter.intern_symbol_id("b"), float_value),
                        (interpreter.intern_symbol_id("c"), true_value),
                        (interpreter.intern_symbol_id("d"), false_value),
                    ],
                    ObjectElement::new(vec![
                        (
                            KeywordElement::new(String::from("a")),
                            Element::Integer(IntegerElement::new(1)),
                        ),
                        (
                            KeywordElement::new(String::from("b")),
                            Element::Float(FloatElement::new(1.1)),
                        ),
                        (
                            KeywordElement::new(String::from("c")),
                            Element::Boolean(BooleanElement::new(true)),
                        ),
                        (
                            KeywordElement::new(String::from("d")),
                            Element::Boolean(BooleanElement::new(false)),
                        ),
                    ]),
                ),
            ];

            for (items, object_element) in specs {
                let object_id =
                    read_object_element(&mut interpreter, object_element)
                        .unwrap();

                let object_id = interpreter
                    .execute_value(
                        interpreter.get_main_environment_id(),
                        object_id,
                    )
                    .unwrap()
                    .try_into()
                    .unwrap();

                for (key_symbol_id, expected_value) in items {
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

        // todo: move to interpreter
        // #[test]
        // fn evaluates_items_correctly() {
        //     let mut interpreter = Interpreter::new();
        //
        //     // todo: uncomment two lines below, and find out why it doesn't work
        //     //            let keyword_value = interpreter.intern_keyword_value("keyword");
        //     let symbol_value = interpreter.intern_symbol_value("symbol");
        //     let string_value = interpreter.intern_string_value("string");
        //
        //     assert_object_has_items(
        //         &mut interpreter,
        //         "{:a 1 :b 1.1 :c #t :d #f :e :keyword :f 'symbol :g \"string\"}",
        //         vec![
        //             ("a", Value::Integer(1)),
        //             ("b", Value::Float(1.1)),
        //             ("c", Value::Boolean(true)),
        //             ("d", Value::Boolean(false)),
        //             //                    ("e", keyword_value),
        //             ("f", symbol_value),
        //             ("g", string_value),
        //         ],
        //     );
        // }
    }
}
