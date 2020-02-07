use crate::interpreter::value::Value;
use crate::interpreter::cons::Cons;
use crate::parser::s_expression_element::SExpressionElement;
use crate::parser::prefix_element::{PrefixElement, Prefix};
use crate::parser::Element;
use crate::interpreter::interpreter::Interpreter;
use crate::parser::object_element::ObjectElement;

fn preread_s_expression(interpreter: &mut Interpreter, sexp_element: &SExpressionElement) -> Value {
    let values = sexp_element.get_values();

    if values.len() == 0 {
        return interpreter.intern_nil();
    }

    // todo: make symbol arena

    let mut root_cons = Cons::new(
        interpreter.intern_nil(),
        interpreter.intern_nil()
    );

    let values = sexp_element.get_values();
    let len = values.len();
    let mut current_cons = &mut root_cons;

    for (index, element) in values.iter().enumerate() {
        let value = preread_element(interpreter, element);
        current_cons.set_car(value);

        if index == len - 1 {
            break;
        }

        let next_cons = Cons::new(
            interpreter.intern_nil(),
            interpreter.intern_nil()
        );

        current_cons.set_cdr(Value::Cons(next_cons));

        if let Value::Cons(next_cons) = current_cons.get_cdr_mut() {
            current_cons = next_cons;
        } else {
            unreachable!();
        }
    }

    Value::Cons(root_cons)
}

fn preread_object(interpreter: &mut Interpreter, object_element: &ObjectElement) -> Value {
    let values = object_element.get_values();
    let object_id = interpreter.make_object();

    for (keyword_element, element) in values {
        let symbol = interpreter.intern_symbol(keyword_element.get_value());
        let value = preread_element(interpreter, element);

        interpreter.set_object_item(object_id, &symbol, value);
    }

    Value::Object(object_id)
}

fn preread_quote_prefix_element(interpreter: &mut Interpreter, element: &Element) -> Value {
    let value = preread_element(interpreter, element);

    let cons = Cons::new(
        interpreter.intern("quote"),
        Value::Cons(Cons::new(
            value,
            interpreter.intern_nil()
        ))
    );

    Value::Cons(cons)
}

fn preread_graveaccent_prefix_element(interpreter: &mut Interpreter, element: &Element) -> Value {
    let value = preread_element(interpreter, element);

    let cons = Cons::new(
        interpreter.intern("`"),
        Value::Cons(Cons::new(
            value,
            interpreter.intern_nil()
        ))
    );

    Value::Cons(cons)
}

fn preread_comma_prefix_element(interpreter: &mut Interpreter, element: &Element) -> Value {
    let value = preread_element(interpreter, element);

    let cons = Cons::new(
        interpreter.intern(","),
        Value::Cons(Cons::new(
            value,
            interpreter.intern_nil()
        ))
    );

    Value::Cons(cons)
}

fn preread_commadog_prefix_element(interpreter: &mut Interpreter, element: &Element) -> Value {
    let value = preread_element(interpreter, element);

    let cons = Cons::new(
        interpreter.intern(",@"),
        Value::Cons(Cons::new(
            value,
            interpreter.intern_nil()
        ))
    );

    Value::Cons(cons)
}

fn preread_prefix_element(interpreter: &mut Interpreter, prefix_element: &PrefixElement) -> Value {
    match prefix_element.get_prefix() {
        Prefix::Quote => preread_quote_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::GraveAccent => preread_graveaccent_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::Comma => preread_comma_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::CommaDog => preread_commadog_prefix_element(interpreter, prefix_element.get_value()),
    }
}

pub fn preread_element(interpreter: &mut Interpreter, element: &Element) -> Value {
    match element {
        Element::Integer(integer_element) => Value::Integer(integer_element.get_value()),
        Element::Float(float_element) => Value::Float(float_element.get_value()),
        Element::Boolean(boolean_element) => Value::Boolean(boolean_element.get_value().clone()),
        Element::String(string_element) => Value::String(string_element.get_value().clone()),
        Element::Symbol(symbol_element) => interpreter.intern(symbol_element.get_value()),
        Element::Keyword(keyword_element) => Value::Keyword(keyword_element.get_value().clone()),
        Element::SExpression(sexp_element) => preread_s_expression(interpreter, sexp_element),
        Element::Object(object_element) => preread_object(interpreter, object_element),
        Element::Prefix(prefix_element) => preread_prefix_element(interpreter, prefix_element),
    }
}

pub fn preread_elements(interpreter: &mut Interpreter, elements: &Vec<Element>) -> Vec<Value> {
    elements.into_iter().map(|e| preread_element(interpreter, &e)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_code;
    use crate::interpreter::symbol::{SymbolArena, Symbol};

    fn new_symbol(name: &str) -> Symbol {
        let mut symbol_arena = SymbolArena::new();

        symbol_arena.intern(name)
    }

    macro_rules! assert_prereading_result_equal {
        ($expected:expr, $code:expr) => {
            let mut interpreter = Interpreter::new();
            let expected = $expected;

            if let Ok((_, program)) = parse_code($code) {
                let result = preread_elements(&mut interpreter, program.get_elements());

                let len = expected.len();

                assert_eq!(len, result.len());

                for i in 0..len {
                    assert_eq!(expected[i], result[i]);
                }
            }
        }
    }


    #[test]
    pub fn prereads_integer_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Integer(1)
            ),
            "1"
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Integer(1),
                Value::Integer(2)
            ),
            "1 2"
        );
    }

    #[test]
    pub fn prereads_float_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Float(1.2)
            ),
            "1.2"
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Float(1.2),
                Value::Float(3.4)
            ),
            "1.2 3.4"
        );
    }

    #[test]
    pub fn prereads_boolean_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Boolean(true)
            ),
            "#t"
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Boolean(true),
                Value::Boolean(false)
            ),
            "#t #f"
        );
    }

    #[test]
    pub fn prereads_string_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::String("cute string".to_string())
            ),
            r#""cute string""#
        );

        assert_prereading_result_equal!(
            vec!(
                Value::String("first cute string".to_string()),
                Value::String("second cute string".to_string())
            ),
            r#""first cute string" "second cute string""#
        );
    }

    #[test]
    pub fn prereads_symbol_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Symbol(new_symbol("cutesymbol"))
            ),
            r#"cutesymbol"#
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Symbol(new_symbol("cutesymbol1")),
                Value::Symbol(new_symbol("cutesymbol2"))
            ),
            r#"cutesymbol1 cutesymbol2"#
        );
    }

    #[test]
    pub fn prereads_keyword_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Keyword("cutekeyword".to_string())
            ),
            r#":cutekeyword"#
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Keyword("cutekeyword1".to_string()),
                Value::Keyword("cutekeyword2".to_string())
            ),
            r#":cutekeyword1 :cutekeyword2"#
        );
    }

    #[test]
    pub fn prereads_s_expression_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Symbol(new_symbol("nil"))
            ),
            "()"
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Cons(
                    Cons::new(
                        Value::Symbol(new_symbol("a")),
                        Value::Symbol(new_symbol("nil"))
                    )
                )
            ),
            "(a)"
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Cons(
                    Cons::new(
                        Value::Symbol(new_symbol("a")),
                        Value::Cons(
                            Cons::new(
                                Value::Symbol(new_symbol("b")),
                                Value::Symbol(new_symbol("nil"))
                            )
                        )
                    )
                )
            ),
            "(a b)"
        );
    }

    macro_rules! assert_object_has_items {
        ($expected:expr, $code:expr) => {
            let mut interpreter = Interpreter::raw();

            if let Ok((_, code)) = parse_code($code) {
                let result = &preread_elements(&mut interpreter, code.get_elements())[0];
                let expected: Vec<(&str, Value)> = $expected;

                assert!(
                    match &result {
                        Value::Object(_) => true,
                        _ => false
                    }
                );

                match result {
                    Value::Object(object_id) => {
                        for (name, value) in expected {
                            let symbol = interpreter.intern_symbol(name);

                            assert_eq!(&value, interpreter.get_object_item(
                                *object_id,
                                &symbol
                            ).unwrap());
                        }
                    },
                    _ => unreachable!()
                }

            }

        }
    }

    #[test]
    pub fn prereads_object_elements_correctly() {
        assert_object_has_items!(vec!(), "{}");
        assert_object_has_items!(
            vec!(
                ("a", Value::Integer(1)),
            ),
            "{:a 1}"
        );
        assert_object_has_items!(
            vec!(
                ("a", Value::Integer(1)),
                ("b", Value::Integer(2))
            ),
            "{:a 1 :b 2}"
        );
    }

    macro_rules! assert_prefix_result_equal {
        ($prefix:expr, $prefix_after:expr, $code: expr) => {
            if let Ok((_, program)) = parse_code($code) {
                let mut interpreter = Interpreter::new();
                let expected = preread_elements(&mut interpreter, &program.get_elements())[0].clone();

                let expected = Value::Cons(Cons::new(
                    Value::Symbol(new_symbol($prefix_after)),
                    Value::Cons(Cons::new(
                        expected,
                        Value::Symbol(new_symbol("nil"))
                    ))
                ));

                let prefixed_code = concat!($prefix, $code);

                assert_prereading_result_equal!(
                    vec!(expected),
                    prefixed_code
                );
            }
        }
    }

    macro_rules! assert_prefix_values_works {
        ($prefix:expr, $prefix_after:expr) => {
            assert_prefix_result_equal!($prefix, $prefix_after, "1");
            assert_prefix_result_equal!($prefix, $prefix_after, "1.3");
            assert_prefix_result_equal!($prefix, $prefix_after, "#t");
            assert_prefix_result_equal!($prefix, $prefix_after, "#f");
            assert_prefix_result_equal!($prefix, $prefix_after, r#""a""#);
            assert_prefix_result_equal!($prefix, $prefix_after, "a");
            assert_prefix_result_equal!($prefix, $prefix_after, ":a");
            assert_prefix_result_equal!($prefix, $prefix_after, "()");
            assert_prefix_result_equal!($prefix, $prefix_after, "(a)");
            assert_prefix_result_equal!($prefix, $prefix_after, "(a b)");
        }
    }

    #[test]
    pub fn prereads_quoted_values_correctly() {
        assert_prefix_values_works!("'", "quote");
        assert_prefix_values_works!("`", "`");
        assert_prefix_values_works!(",", ",");
        assert_prefix_values_works!(",@", ",@");
    }

    #[test]
    pub fn prereads_complex_s_expression_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Cons(Cons::new(
                    Value::Symbol(new_symbol("a")),
                    Value::Cons(Cons::new(
                        Value::Integer(1),
                        Value::Cons(Cons::new(
                            Value::Float(2.3),
                            Value::Cons(Cons::new(
                                Value::Boolean(true),
                                Value::Cons(Cons::new(
                                    Value::Cons(Cons::new(
                                        Value::Integer(3),
                                        Value::Cons(Cons::new(
                                            Value::Integer(4),
                                            Value::Symbol(new_symbol("nil"))
                                        ))
                                    )),
                                    Value::Cons(Cons::new(
                                        Value::Boolean(false),
                                        Value::Symbol(new_symbol("nil"))
                                    ))
                                ))
                            ))
                        ))
                    ))
                ))
            ),
            "(a 1 2.3 #t (3 4) #f)"
        );
    }
}