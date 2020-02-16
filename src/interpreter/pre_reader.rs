use crate::interpreter::string::string_arena::StringId;
use crate::interpreter::keyword::keyword_arena::KeywordId;
use crate::interpreter::value::Value;
use crate::parser::s_expression_element::SExpressionElement;
use crate::parser::prefix_element::{PrefixElement, Prefix};
use crate::parser::Element;
use crate::interpreter::interpreter::Interpreter;
use crate::parser::object_element::ObjectElement;
use crate::parser::delimited_symbols_element::DelimitedSymbolsElement;

fn preread_s_expression(interpreter: &mut Interpreter, sexp_element: SExpressionElement) -> Value {
    let values = sexp_element.get_values();

    if values.len() == 0 {
        return interpreter.intern_nil();
    }

    let nil = interpreter.intern_nil();

    let root_cons = interpreter.make_cons(
        nil.clone(),
        nil
    );

    let len = values.len();
    let mut current_cons_id = root_cons;

    for (index, element) in values.into_iter().enumerate() {
        let value = preread_element(interpreter, element);

        interpreter.set_car(current_cons_id, value); // todo: check error here

        if index == len - 1 {
            break;
        }

        let nil = interpreter.intern_nil();

        let next_cons_id = interpreter.make_cons(
            nil.clone(),
            nil
        );

        interpreter.set_cdr(current_cons_id, Value::Cons(next_cons_id)); // todo: check error here

        if let Ok(Value::Cons(next_cons)) = interpreter.get_cdr(current_cons_id) {
            current_cons_id = next_cons.clone();
        } else {
            unreachable!(); //todo: check
        }
    }

    Value::Cons(root_cons)
}

fn preread_object(interpreter: &mut Interpreter, object_element: ObjectElement) -> Value {
    let values = object_element.get_values();

    let mut last_cons = interpreter.intern_nil();

    for (keyword_element, element) in values.into_iter().rev() {
        let keyword = interpreter.intern_keyword_value(keyword_element.get_value());
        let value = preread_element(interpreter, element);

        last_cons = Value::Cons(interpreter.make_cons(
            value,
            last_cons
        ));

        last_cons = Value::Cons(interpreter.make_cons(
            keyword,
            last_cons
        ));
    }

    let sym1 = interpreter.intern("object");
    let nil = interpreter.intern_nil();
    let car = Value::Cons(interpreter.make_cons(
        sym1,
        nil
    ));

    let keyword = interpreter.intern_keyword_value(String::from("make"));

    let car = Value::Cons(interpreter.make_cons(
        keyword,
        car
    ));

    Value::Cons(interpreter.make_cons(
        car,
        last_cons
    ))
}

fn preread_delimited_symbols_element(
    interpreter: &mut Interpreter,
    delimited_symbols_element: DelimitedSymbolsElement
) -> Value {
    let values = delimited_symbols_element.get_symbols();

    let object_symbol_name = values[0].get_value();
    let mut previous_cons = interpreter.intern(object_symbol_name);

    for symbol_element in &values[1..] {
        let symbol_name = symbol_element.get_value();

        let nil = interpreter.intern_nil();
        let current_cons = Value::Cons(interpreter.make_cons(
            previous_cons,
            nil
        ));

        let keyword = interpreter.intern_keyword_value(String::from(symbol_name));

        let current_cons = Value::Cons(interpreter.make_cons(
            keyword,
            current_cons
        ));

        previous_cons = current_cons;
    }

    previous_cons
}

fn preread_quote_prefix_element(interpreter: &mut Interpreter, element: Element) -> Value {
    let value = preread_element(interpreter, element);

    let nil = interpreter.intern_nil();
    let cdr = Value::Cons(interpreter.make_cons(
        value,
        nil
    ));

    let quote = interpreter.intern("quote");
    let cons_id = interpreter.make_cons(
        quote,
        cdr
    );

    Value::Cons(cons_id)
}

fn preread_graveaccent_prefix_element(interpreter: &mut Interpreter, element: Element) -> Value {
    let value = preread_element(interpreter, element);

    let graveaccent = interpreter.intern("`");

    let nil = interpreter.intern_nil();
    let cdr = Value::Cons(interpreter.make_cons(
        value,
        nil
    ));

    let cons_id = interpreter.make_cons(
        graveaccent,
        cdr
    );

    Value::Cons(cons_id)
}

fn preread_comma_prefix_element(interpreter: &mut Interpreter, element: Element) -> Value {
    let value = preread_element(interpreter, element);

    let comma = interpreter.intern(",");

    let nil = interpreter.intern_nil();
    let cdr = Value::Cons(interpreter.make_cons(
        value,
        nil
    ));

    let cons_id = interpreter.make_cons(
        comma,
        cdr
    );

    Value::Cons(cons_id)
}

fn preread_commadog_prefix_element(interpreter: &mut Interpreter, element: Element) -> Value {
    let value = preread_element(interpreter, element);

    let commadog = interpreter.intern(",@");

    let nil = interpreter.intern_nil();
    let cdr = Value::Cons(interpreter.make_cons(
        value,
        nil
    ));

    let cons_id = interpreter.make_cons(
        commadog,
        cdr
    );

    Value::Cons(cons_id)
}

fn preread_prefix_element(interpreter: &mut Interpreter, prefix_element: PrefixElement) -> Value {
    match prefix_element.get_prefix() {
        Prefix::Quote => preread_quote_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::GraveAccent => preread_graveaccent_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::Comma => preread_comma_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::CommaDog => preread_commadog_prefix_element(interpreter, prefix_element.get_value()),
    }
}

pub fn preread_element(interpreter: &mut Interpreter, element: Element) -> Value {
    match element {
        Element::Integer(integer_element) =>
            Value::Integer(integer_element.get_value()),
        Element::Float(float_element) =>
            Value::Float(float_element.get_value()),
        Element::Boolean(boolean_element) =>
            Value::Boolean(boolean_element.get_value().clone()),
        Element::String(string_element) => {
            let string = string_element.get_value();

            interpreter.intern_string_value(string)
        },
        Element::Symbol(symbol_element) =>
            interpreter.intern(symbol_element.get_value()),
        Element::Keyword(keyword_element) => {
            let keyword_name = keyword_element.get_value();

            interpreter.intern_keyword_value(keyword_name)
        },
        Element::SExpression(sexp_element) =>
            preread_s_expression(interpreter, sexp_element),
        Element::Object(object_element) =>
            preread_object(interpreter, object_element),
        Element::DelimitedSymbols(delimited_symbols_element) =>
            preread_delimited_symbols_element(interpreter, delimited_symbols_element),
        Element::Prefix(prefix_element) =>
            preread_prefix_element(interpreter, prefix_element),
    }
}

pub fn preread_elements(interpreter: &mut Interpreter, elements: Vec<Element>) -> Vec<Value> {
    elements.into_iter().map(|e| preread_element(interpreter, e)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_code;
    use crate::interpreter::symbol::{SymbolArena, Symbol};
    use crate::interpreter::lib::assertion;

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
                    assert_eq!(&expected[i], &result[i]);
                }
            }
        }
    }


    #[test]
    fn prereads_integer_elements_correctly() {
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
    fn prereads_float_elements_correctly() {
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
    fn prereads_boolean_elements_correctly() {
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
    fn prereads_string_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::String(StringId::new(0))
            ),
            r#""cute string""#
        );

        assert_prereading_result_equal!(
            vec!(
                Value::String(StringId::new(0)),
                Value::String(StringId::new(1))
            ),
            r#""first cute string" "second cute string""#
        );
    }

    #[test]
    fn prereads_symbol_elements_correctly() {
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
    fn prereads_keyword_elements_correctly() {
        assert_prereading_result_equal!(
            vec!(
                Value::Keyword(KeywordId::new(0))
            ),
            r#":cutekeyword"#
        );

        assert_prereading_result_equal!(
            vec!(
                Value::Keyword(KeywordId::new(0)),
                Value::Keyword(KeywordId::new(0)),
                Value::Keyword(KeywordId::new(1))
            ),
            r#":cutekeyword1 :cutekeyword1 :cutekeyword2"#
        );
    }

    #[test]
    fn prereads_s_expression_elements_correctly() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Symbol(new_symbol("nil"));
        assert_prereading_deeply(&mut interpreter, expected, "()");

        let expected = interpreter.make_cons_value(
            Value::Symbol(new_symbol("a")),
            Value::Symbol(new_symbol("nil"))
        );
        assert_prereading_deeply(&mut interpreter, expected, "(a)");

        let cdr = interpreter.make_cons_value(
            Value::Symbol(new_symbol("b")),
            Value::Symbol(new_symbol("nil"))
        );
        let expected = interpreter.make_cons_value(
            Value::Symbol(new_symbol("a")),
            cdr
        );
        assert_prereading_deeply(&mut interpreter, expected, "(a b)");
    }

    #[cfg(test)]
    mod object {
        use super::*;

        macro_rules! assert_object_has_items {
            ($expected:expr, $code:expr) => {
                let mut interpreter = Interpreter::new();

                if let Ok((_, code)) = parse_code($code) {
                    let result = preread_elements(&mut interpreter, code.get_elements()).remove(0);

                    let result = interpreter.evaluate_value(
                        interpreter.get_root_environment(),
                        result
                    ).unwrap();
                    let expected: Vec<(&str, Value)> = $expected;

                    match result {
                        Value::Object(object_id) => {
                            for (name, value) in expected {
                                let symbol = interpreter.intern_symbol(name);

                                let expected = value;
                                let result = interpreter.get_object_item(
                                    object_id,
                                    &symbol
                                ).unwrap().clone();

                                assertion::assert_deep_equal(
                                    &mut interpreter,
                                    expected,
                                    result
                                );
                            }
                        },
                        _ => unreachable!()
                    }

                }

            }
        }

        #[test]
        fn prereads_elements_correctly() {
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

        #[test]
        fn evaluates_items_correctly() {
            assert_object_has_items!(
                vec!(
                    ("a", Value::Integer(1)),
                    ("b", Value::Float(1.1)),
                    ("c", Value::Boolean(true)),
                    ("d", Value::Boolean(false)),
                    ("e", Value::Keyword(KeywordId::new(3))),
                    ("f", Value::Symbol(new_symbol("symbol"))),
                    ("g", Value::String(StringId::new(0))),
                ),
                "{:a 1 :b 1.1 :c #t :d #f :e :keyword :f 'symbol :g \"string\"}"
            );
        }
    }

    fn assert_prereading_deeply(interpreter: &mut Interpreter, expected: Value, code: &str) {
        if let Ok((_, program)) = parse_code(code) {
            let result = preread_elements(
                interpreter,
                program.get_elements()
            ).remove(0);

            assertion::assert_deep_equal(interpreter, expected, result);
        }
    }

    #[test]
    fn prereads_delimited_symbols_element_correctly() {
        let mut interpreter = Interpreter::new();

        let cdr = interpreter.make_cons_value(
            Value::Symbol(new_symbol("object")),
            Value::Symbol(new_symbol("nil"))
        );

        let keyword = interpreter.intern_keyword_value(String::from("value"));
        let expected = interpreter.make_cons_value(
            keyword,
            cdr
        );

        assert_prereading_deeply(&mut interpreter, expected, "object:value");

        let cdr = interpreter.make_cons_value(
            Value::Symbol(new_symbol("object")),
            Value::Symbol(new_symbol("nil"))
        );

        let keyword = interpreter.intern_keyword_value(String::from("value1"));
        let car = interpreter.make_cons_value(
            keyword,
            cdr
        );

        let cdr = interpreter.make_cons_value(
            car,
            Value::Symbol(new_symbol("nil"))
        );

        let keyword = interpreter.intern_keyword_value(String::from("value2"));
        let expected = interpreter.make_cons_value(
            keyword,
            cdr
        );

        assert_prereading_deeply(&mut interpreter, expected, "object:value1:value2");

        let cdr = interpreter.make_cons_value(
            Value::Symbol(new_symbol("object")),
            Value::Symbol(new_symbol("nil"))
        );

        let keyword = interpreter.intern_keyword_value(String::from("value1"));
        let car = interpreter.make_cons_value(
            keyword,
            cdr
        );

        let cdr = interpreter.make_cons_value(
            car,
            Value::Symbol(new_symbol("nil"))
        );

        let keyword = interpreter.intern_keyword_value(String::from("value2"));
        let car = interpreter.make_cons_value(
            keyword,
            cdr
        );

        let expected = interpreter.make_cons_value(
            car,
            Value::Symbol(new_symbol("nil"))
        );

        assert_prereading_deeply(&mut interpreter, expected, "(object:value1:value2)");
    }

    macro_rules! assert_prefix_result_equal {
        ($prefix:expr, $prefix_after:expr, $code: expr) => {
            if let Ok((_, program)) = parse_code($code) {
                let mut interpreter = Interpreter::new();
                let expected = preread_elements(&mut interpreter, program.get_elements())[0].clone();

                let expected = interpreter.make_cons_value(
                    expected,
                    Value::Symbol(new_symbol("nil"))
                );

                let expected = interpreter.make_cons_value(
                    Value::Symbol(new_symbol($prefix_after)),
                    expected
                );

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
    fn prereads_quoted_values_correctly() {
        assert_prefix_values_works!("'", "quote");
        assert_prefix_values_works!("`", "`");
        assert_prefix_values_works!(",", ",");
        assert_prefix_values_works!(",@", ",@");
    }

    #[test]
    fn prereads_complex_s_expression_correctly() {
        let mut interpreter = Interpreter::new();

        let cdr = interpreter.make_cons_value(
            Value::Integer(4),
            Value::Symbol(new_symbol("nil"))
        );

        let car = interpreter.make_cons_value(
            Value::Integer(3),
            cdr
        );

        let cdr = interpreter.make_cons_value(
            Value::Boolean(false),
            Value::Symbol(new_symbol("nil"))
        );

        let cdr = interpreter.make_cons_value(
            car,
            cdr
        );

        let cdr = interpreter.make_cons_value(
            Value::Boolean(true),
            cdr
        );

        let cdr = interpreter.make_cons_value(
            Value::Float(2.3),
            cdr
        );

        let cdr = interpreter.make_cons_value(
            Value::Integer(1),
             cdr
        );

        let expected = interpreter.make_cons_value(
            Value::Symbol(new_symbol("a")),
            cdr
        );

        assert_prereading_deeply(&mut interpreter, expected, "(a 1 2.3 #t (3 4) #f)");
    }
}