use crate::interpreter::value::Value;
use crate::interpreter::cons::Cons;
use crate::parser::s_expression_element::SExpressionElement;
use crate::parser::prefix_element::{PrefixElement, Prefix};
use crate::parser::Element;

fn preread_s_expression(sexp_element: &SExpressionElement) -> Value {
    let values = sexp_element.get_values();

    if values.len() == 0 {
        return Value::Symbol("nil".to_string());
    }

    // todo: make symbol arena

    let mut root_cons = Cons::new(
        Value::Symbol("nil".to_string()),
        Value::Symbol("nil".to_string())
    );

    let values = sexp_element.get_values();
    let len = values.len();
    let mut current_cons = &mut root_cons;

    for (index, element) in values.iter().enumerate() {
        let value = preread_element(element);
        current_cons.set_car(value);

        if index == len - 1 {
            break;
        }

        let next_cons = Cons::new(
            Value::Symbol("nil".to_string()),
            Value::Symbol("nil".to_string())
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

fn preread_quote_prefix_element(element: &Element) -> Value {
    let value = preread_element(element);

    let cons = Cons::new(
        Value::Symbol("quote".to_string()),
        Value::Cons(Cons::new(
            value,
            Value::Symbol("nil".to_string())
        ))
    );

    Value::Cons(cons)
}

fn preread_graveaccent_prefix_element(element: &Element) -> Value {
    let value = preread_element(element);

    let cons = Cons::new(
        Value::Symbol("`".to_string()),
        Value::Cons(Cons::new(
            value,
            Value::Symbol("nil".to_string())
        ))
    );

    Value::Cons(cons)
}

fn preread_comma_prefix_element(element: &Element) -> Value {
    let value = preread_element(element);

    let cons = Cons::new(
        Value::Symbol(",".to_string()),
        Value::Cons(Cons::new(
            value,
            Value::Symbol("nil".to_string())
        ))
    );

    Value::Cons(cons)
}

fn preread_commadog_prefix_element(element: &Element) -> Value {
    let value = preread_element(element);

    let cons = Cons::new(
        Value::Symbol(",@".to_string()),
        Value::Cons(Cons::new(
            value,
            Value::Symbol("nil".to_string())
        ))
    );

    Value::Cons(cons)
}

fn preread_prefix_element(prefix_element: &PrefixElement) -> Value {
    match prefix_element.get_prefix() {
        Prefix::Quote => preread_quote_prefix_element(prefix_element.get_value()),
        Prefix::GraveAccent => preread_graveaccent_prefix_element(prefix_element.get_value()),
        Prefix::Comma => preread_comma_prefix_element(prefix_element.get_value()),
        Prefix::CommaDog => preread_commadog_prefix_element(prefix_element.get_value()),
    }
}

pub fn preread_element(element: &Element) -> Value {
    use Element::*;

    match element {
        Integer(integer_element) => Value::Integer(integer_element.get_value()),
        Float(float_element) => Value::Float(float_element.get_value()),
        Boolean(boolean_element) => Value::Boolean(boolean_element.get_value().clone()),
        String(string_element) => Value::String(string_element.get_value().clone()),
        Symbol(symbol_element) => Value::Symbol(symbol_element.get_value().clone()),
        Keyword(keyword_element) => Value::Keyword(keyword_element.get_value().clone()),
        SExpression(sexp_element) => preread_s_expression(sexp_element),
        Prefix(prefix_element) => preread_prefix_element(prefix_element)
    }
}

pub fn preread_elements(elements: &Vec<Element>) -> Vec<Value> {
    elements.into_iter().map(|e| preread_element(&e)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_code;

    macro_rules! assert_prereading_result_equal {
        ($expected:expr, $code:expr) => {
            let expected = $expected;

            if let Ok((_, program)) = parse_code($code) {
                let result = preread_elements(program.get_elements());
                println!("{:#?}", result);

                let len = expected.len();

                assert_eq!(len, result.len());

                for i in 0..len {
                    assert_eq!(expected[i], result[i]);
                }
            }
        }
    }


    #[test]
    pub fn test_prereads_integer_elements_correctly() {
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
    pub fn test_prereads_float_elements_correctly() {
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
    pub fn test_prereads_boolean_elements_correctly() {
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
    pub fn test_prereads_string_elements_correctly() {
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
    pub fn test_prereads_symbol_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Symbol("cutesymbol".to_string())
            ),
            r#"cutesymbol"#
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Symbol("cutesymbol1".to_string()),
                Value::Symbol("cutesymbol2".to_string())
            ),
            r#"cutesymbol1 cutesymbol2"#
        );
    }

    #[test]
    pub fn test_prereads_keyword_elements_correctly() {
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
    pub fn test_prereads_s_expression_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Symbol("nil".to_string())
            ),
            "()"
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Cons(
                    Cons::new(
                        Value::Symbol("a".to_string()),
                        Value::Symbol("nil".to_string())
                    )
                )
            ),
            "(a)"
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Cons(
                    Cons::new(
                        Value::Symbol("a".to_string()),
                        Value::Cons(
                            Cons::new(
                                Value::Symbol("b".to_string()),
                                Value::Symbol("nil".to_string())
                            )
                        )
                    )
                )
            ),
            "(a b)"
        );
    }

    macro_rules! assert_prefix_result_equal {
        ($prefix:expr, $prefix_after:expr, $code: expr) => {
            if let Ok((_, program)) = parse_code($code) {
                let expected = preread_elements(&program.get_elements())[0].clone();

                let expected = Value::Cons(Cons::new(
                    Value::Symbol($prefix_after.to_string()),
                    Value::Cons(Cons::new(
                        expected,
                        Value::Symbol("nil".to_string())
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
    pub fn test_reads_quoted_values_correctly() {
        assert_prefix_values_works!("'", "quote");
        assert_prefix_values_works!("`", "`");
        assert_prefix_values_works!(",", ",");
        assert_prefix_values_works!(",@", ",@");
    }

    #[test]
    pub fn test_prereads_complex_s_expression_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Cons(Cons::new(
                    Value::Symbol("a".to_string()),
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
                                            Value::Symbol("nil".to_string())
                                        ))
                                    )),
                                    Value::Cons(Cons::new(
                                        Value::Boolean(false),
                                        Value::Symbol("nil".to_string())
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