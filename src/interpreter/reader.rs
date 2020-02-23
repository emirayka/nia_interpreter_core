use crate::interpreter::string::string_arena::StringId;
use crate::interpreter::keyword::keyword_arena::KeywordId;
use crate::interpreter::value::Value;
use crate::parser::s_expression_element::SExpressionElement;
use crate::parser::prefix_element::{PrefixElement, Prefix};
use crate::parser::Element;
use crate::interpreter::interpreter::Interpreter;
use crate::parser::object_element::ObjectElement;
use crate::parser::delimited_symbols_element::DelimitedSymbolsElement;
use std::cmp::max;
use crate::parser::short_lambda_element::ShortLambdaElement;
use crate::interpreter::error::Error;

fn read_s_expression(interpreter: &mut Interpreter, sexp_element: SExpressionElement) -> Result<Value, Error> {
    let values = sexp_element.get_values();

    if values.len() == 0 {
        return Ok(interpreter.intern_nil_symbol_value());
    }

    let nil = interpreter.intern_nil_symbol_value();

    let root_cons = interpreter.make_cons(
        nil,
        nil
    );

    let len = values.len();
    let mut current_cons_id = root_cons;

    for (index, element) in values.into_iter().enumerate() {
        let value = read_element(interpreter, element)?;

        interpreter.set_car(current_cons_id, value)?;

        if index == len - 1 {
            break;
        }

        let nil = interpreter.intern_nil_symbol_value();

        let next_cons_id = interpreter.make_cons(
            nil,
            nil
        );

        interpreter.set_cdr(current_cons_id, Value::Cons(next_cons_id))
            .map_err(|err| interpreter.make_generic_execution_error_caused(
                "",
                err
            ))?;

        if let Ok(Value::Cons(next_cons)) = interpreter.get_cdr(current_cons_id) {
            current_cons_id = next_cons;
        } else {
            unreachable!(); //todo: check
        }
    }

    Ok(Value::Cons(root_cons))
}

fn count_short_lambda_argument_count(
    interpreter: &mut Interpreter,
    value: Value
) -> Result<u8, Error> {
    let count = match value {
        Value::Symbol(symbol_id) => {
            let symbol_name = interpreter.get_symbol_name(symbol_id)?;

            match symbol_name.chars().next() {
                Some(c) => {
                    if c == '%' {
                        match (&symbol_name['%'.len_utf8()..]).parse::<u8>() {
                            Ok(count) => count,
                            _ => 0
                        }
                    } else {
                        0
                    }
                },
                _ => 0
            }
        }
        Value::Cons(cons) => {
            let car = interpreter.get_car(cons)?;
            let cdr = interpreter.get_cdr(cons)?;

            let car_count = count_short_lambda_argument_count(interpreter, car)?;
            let cdr_count = count_short_lambda_argument_count(interpreter, cdr)?;

            max(car_count, cdr_count)
        },
        _ => 0,
    };

    Ok(count)
}

fn make_short_lambda_argument_list(
    interpreter: &mut Interpreter,
    count: u8
) -> Value {
    let mut last_cons = interpreter.intern_nil_symbol_value();

    for i in 0..count {
        let current_argument_index = count - i;
        let symbol_name = format!("%{}", current_argument_index);
        let symbol = interpreter.intern_symbol_value(&symbol_name);

        last_cons = interpreter.make_cons_value(symbol, last_cons);
    }

    last_cons
}

fn read_short_lambda(
    interpreter: &mut Interpreter,
    short_lambda_element: ShortLambdaElement
) -> Result<Value, Error> {
    let function = interpreter.intern_symbol_value("function");
    let lambda = interpreter.intern_symbol_value("lambda");
    let nil = interpreter.intern_nil_symbol_value();

    let code = read_s_expression(
        interpreter,
        short_lambda_element.get_value()
    )?;
    let argument_count = count_short_lambda_argument_count(interpreter, code)?;
    let arguments = make_short_lambda_argument_list(interpreter, argument_count);

    let cdr = interpreter.make_cons_value(code, nil);
    let cdr = interpreter.make_cons_value(arguments, cdr);
    let car = interpreter.make_cons_value(lambda, cdr);

    let cdr = interpreter.make_cons_value(car, nil);
    let value = interpreter.make_cons_value(function, cdr);

    Ok(value)
}

fn read_object(interpreter: &mut Interpreter, object_element: ObjectElement) -> Result<Value, Error> {
    let values = object_element.get_values();

    let mut last_cons = interpreter.intern_nil_symbol_value();

    for (keyword_element, element) in values.into_iter().rev() {
        let keyword = interpreter.intern_keyword_value(keyword_element.get_value());
        let value = read_element(interpreter, element)?;

        last_cons = Value::Cons(interpreter.make_cons(
            value,
            last_cons
        ));

        last_cons = Value::Cons(interpreter.make_cons(
            keyword,
            last_cons
        ));
    }

    let sym1 = interpreter.intern_symbol_value("object");
    let nil = interpreter.intern_nil_symbol_value();
    let car = Value::Cons(interpreter.make_cons(
        sym1,
        nil
    ));

    let keyword = interpreter.intern_keyword_value(String::from("make"));

    let car = Value::Cons(interpreter.make_cons(
        keyword,
        car
    ));

    let cons_id = interpreter.make_cons(
        car,
        last_cons
    );

    Ok(Value::Cons(cons_id))
}

fn read_delimited_symbols_element(
    interpreter: &mut Interpreter,
    delimited_symbols_element: DelimitedSymbolsElement
) -> Value {
    let values = delimited_symbols_element.get_symbols();

    let object_symbol_name = values[0].get_value();
    let mut previous_cons = interpreter.intern_symbol_value(object_symbol_name);

    for symbol_element in &values[1..] {
        let symbol_name = symbol_element.get_value();

        let nil = interpreter.intern_nil_symbol_value();
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

fn read_quote_prefix_element(interpreter: &mut Interpreter, element: Element) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let nil = interpreter.intern_nil_symbol_value();
    let cdr = Value::Cons(interpreter.make_cons(
        value,
        nil
    ));

    let quote = interpreter.intern_symbol_value("quote");
    let cons_id = interpreter.make_cons(
        quote,
        cdr
    );

    Ok(Value::Cons(cons_id))
}

fn read_graveaccent_prefix_element(interpreter: &mut Interpreter, element: Element) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let graveaccent = interpreter.intern_symbol_value("`");

    let nil = interpreter.intern_nil_symbol_value();
    let cdr = Value::Cons(interpreter.make_cons(
        value,
        nil
    ));

    let cons_id = interpreter.make_cons(
        graveaccent,
        cdr
    );

    Ok(Value::Cons(cons_id))
}

fn read_comma_prefix_element(interpreter: &mut Interpreter, element: Element) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let comma = interpreter.intern_symbol_value(",");

    let nil = interpreter.intern_nil_symbol_value();
    let cdr = Value::Cons(interpreter.make_cons(
        value,
        nil
    ));

    let cons_id = interpreter.make_cons(
        comma,
        cdr
    );

    Ok(Value::Cons(cons_id))
}

fn read_commadog_prefix_element(interpreter: &mut Interpreter, element: Element) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let commadog = interpreter.intern_symbol_value(",@");

    let nil = interpreter.intern_nil_symbol_value();
    let cdr = Value::Cons(interpreter.make_cons(
        value,
        nil
    ));

    let cons_id = interpreter.make_cons(
        commadog,
        cdr
    );

    Ok(Value::Cons(cons_id))
}

fn read_prefix_element(interpreter: &mut Interpreter, prefix_element: PrefixElement) -> Result<Value, Error> {
    match prefix_element.get_prefix() {
        Prefix::Quote => read_quote_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::GraveAccent => read_graveaccent_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::Comma => read_comma_prefix_element(interpreter, prefix_element.get_value()),
        Prefix::CommaDog => read_commadog_prefix_element(interpreter, prefix_element.get_value()),
    }
}

pub fn read_element(interpreter: &mut Interpreter, element: Element) -> Result<Value, Error> {
    let value = match element {
        Element::Integer(integer_element) =>
            Value::Integer(integer_element.get_value()),
        Element::Float(float_element) =>
            Value::Float(float_element.get_value()),
        Element::Boolean(boolean_element) =>
            Value::Boolean(boolean_element.get_value()),
        Element::String(string_element) => {
            let string = string_element.get_value();

            interpreter.intern_string_value(string)
        },
        Element::Symbol(symbol_element) =>
            interpreter.intern_symbol_value(symbol_element.get_value()),
        Element::Keyword(keyword_element) => {
            let keyword_name = keyword_element.get_value();

            interpreter.intern_keyword_value(keyword_name)
        },
        Element::SExpression(sexp_element) =>
            read_s_expression(interpreter, sexp_element)?,
        Element::Object(object_element) =>
            read_object(interpreter, object_element)?,
        Element::DelimitedSymbols(delimited_symbols_element) =>
            read_delimited_symbols_element(interpreter, delimited_symbols_element),
        Element::Prefix(prefix_element) =>
            read_prefix_element(interpreter, prefix_element)?,
        Element::ShortLambda(short_lambda_element) =>
            read_short_lambda(interpreter, short_lambda_element)?

    };

    Ok(value)
}

pub fn read_elements(interpreter: &mut Interpreter, elements: Vec<Element>) -> Result<Vec<Value>, Error> {
    let mut result = Vec::new();

    for element in elements {
        match read_element(interpreter, element) {
            Ok(value) => result.push(value),
            Err(error) => return Err(error)
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_code;
    use crate::interpreter::symbol::{SymbolId};
    use crate::interpreter::lib::assertion;

    macro_rules! assert_reading_result_equal {
        ($expected:expr, $code:expr) => {
            let mut interpreter = Interpreter::new();
            let expected = $expected;

            if let Ok((_, program)) = parse_code($code) {
                let result = read_elements(&mut interpreter, program.get_elements()).unwrap();

                let len = expected.len();

                assert_eq!(len, result.len());

                for i in 0..len {
                    let expected = expected[i];
                    let result = result[i];

                    assertion::assert_deep_equal(&mut interpreter, expected, result);
                }
            }
        }
    }


    #[test]
    fn reads_integer_elements_correctly() {
        assert_reading_result_equal!(
            vec!(
                Value::Integer(1)
            ),
            "1"
        );

        assert_reading_result_equal!(
            vec!(
                Value::Integer(1),
                Value::Integer(2)
            ),
            "1 2"
        );
    }

    #[test]
    fn reads_float_elements_correctly() {
        assert_reading_result_equal!(
            vec!(
                Value::Float(1.2)
            ),
            "1.2"
        );

        assert_reading_result_equal!(
            vec!(
                Value::Float(1.2),
                Value::Float(3.4)
            ),
            "1.2 3.4"
        );
    }

    #[test]
    fn reads_boolean_elements_correctly() {
        assert_reading_result_equal!(
            vec!(
                Value::Boolean(true)
            ),
            "#t"
        );

        assert_reading_result_equal!(
            vec!(
                Value::Boolean(true),
                Value::Boolean(false)
            ),
            "#t #f"
        );
    }

    #[test]
    fn reads_string_elements_correctly() {
        let mut interpreter = Interpreter::new();

        assert_reading_result_equal!(
            vec!(
                interpreter.intern_string_value(String::from("cute string"))
            ),
            r#""cute string""#
        );

        let mut interpreter = Interpreter::new();
        assert_reading_result_equal!(
            vec!(
                interpreter.intern_string_value(String::from("first cute string")),
                interpreter.intern_string_value(String::from("second cute string")),
            ),
            r#""first cute string" "second cute string""#
        );
    }

    #[test]
    fn reads_symbol_elements_correctly() {
        let mut interpreter = Interpreter::new();

        assert_reading_result_equal!(
            vec!(
                interpreter.intern_symbol_value("cutesymbol")
            ),
            r#"cutesymbol"#
        );

        let mut interpreter = Interpreter::new();
        assert_reading_result_equal!(
            vec!(
                interpreter.intern_symbol_value("cutesymbol1"),
                interpreter.intern_symbol_value("cutesymbol1"),
                interpreter.intern_symbol_value("cutesymbol2")
            ),
            r#"cutesymbol1 cutesymbol1 cutesymbol2"#
        );
    }

    #[test]
    fn reads_keyword_elements_correctly() {
        let mut interpreter = Interpreter::new();

        assert_reading_result_equal!(
            vec!(
                interpreter.intern_keyword_value(String::from("cutekeyword"))
            ),
            r#":cutekeyword"#
        );

        let mut interpreter = Interpreter::new();
        assert_reading_result_equal!(
            vec!(
                interpreter.intern_keyword_value(String::from("cutekeyword1")),
                interpreter.intern_keyword_value(String::from("cutekeyword1")),
                interpreter.intern_keyword_value(String::from("cutekeyword2"))
            ),
            r#":cutekeyword1 :cutekeyword1 :cutekeyword2"#
        );
    }

    #[test]
    fn reads_s_expression_elements_correctly() {
        let mut interpreter = Interpreter::new();

        let expected = interpreter.intern_nil_symbol_value();
        assert_reading_deeply(&mut interpreter, expected, "()");

        let symbol = interpreter.intern_symbol_value("a");
        let nil= interpreter.intern_nil_symbol_value();
        let expected = interpreter.make_cons_value(
            symbol,
            nil
        );
        assert_reading_deeply(&mut interpreter, expected, "(a)");

        let symbol = interpreter.intern_symbol_value("b");
        let nil= interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            symbol,
            nil
        );
        let symbol = interpreter.intern_symbol_value("a");
        let expected = interpreter.make_cons_value(
            symbol,
            cdr
        );
        assert_reading_deeply(&mut interpreter, expected, "(a b)");
    }

    #[cfg(test)]
    mod short_lambda {
        use super::*;

        fn assert_short_lambda_valid(
            interpreter: &mut Interpreter,
            arguments: Value,
            body: Value,
            code: &str
        ) {
            let nil = interpreter.intern_nil_symbol_value();
            let function = interpreter.intern_symbol_value("function");
            let lambda = interpreter.intern_symbol_value("lambda");

            let cdr= interpreter.make_cons_value(body, nil);
            let cdr= interpreter.make_cons_value(arguments, cdr);
            let car = interpreter.make_cons_value(lambda, cdr);

            let cdr = interpreter.make_cons_value(car, nil);
            let expected = interpreter.make_cons_value(function, cdr);

            assert_reading_deeply(interpreter, expected, code);
        }

        #[test]
        fn reads_short_lambda_without_arguments_correctly() {
            let mut interpreter = Interpreter::new();
            let nil = interpreter.intern_nil_symbol_value();

            assert_short_lambda_valid(&mut interpreter, nil, nil, "#()");
        }

        #[test]
        fn reads_short_lambda_with_an_argument_correctly() {
            let mut interpreter = Interpreter::new();

            let plus = interpreter.intern_symbol_value("+");
            let one = Value::Integer(1);
            let arg1 = interpreter.intern_symbol_value("%1");
            let nil = interpreter.intern_nil_symbol_value();

            let cdr = interpreter.make_cons_value(arg1, nil);
            let cdr = interpreter.make_cons_value(one, cdr);
            let body = interpreter.make_cons_value(plus, cdr);

            let arguments = interpreter.make_cons_value(arg1, nil);

            assert_short_lambda_valid(&mut interpreter, arguments, body, "#(+ 1 %1)");
        }

        #[test]
        fn reads_short_lambda_with_two_arguments_correctly() {
            let mut interpreter = Interpreter::new();

            let plus = interpreter.intern_symbol_value("+");
            let arg1 = interpreter.intern_symbol_value("%1");
            let arg2 = interpreter.intern_symbol_value("%2");
            let nil = interpreter.intern_nil_symbol_value();

            let cdr = interpreter.make_cons_value(arg2, nil);
            let cdr = interpreter.make_cons_value(arg1, cdr);
            let body = interpreter.make_cons_value(plus, cdr);

            let cdr = interpreter.make_cons_value(arg2, nil);
            let arguments = interpreter.make_cons_value(arg1, cdr);

            assert_short_lambda_valid(&mut interpreter, arguments, body, "#(+ %1 %2)");
        }
    }

    #[cfg(test)]
    mod object {
        use super::*;

        macro_rules! assert_object_has_items {
            ($expected:expr, $code:expr) => {
                let mut interpreter = Interpreter::new();

                if let Ok((_, code)) = parse_code($code) {
                    let result = read_elements(&mut interpreter, code.get_elements())
                        .unwrap()
                        .remove(0);

                    let result = interpreter.evaluate_value(
                        interpreter.get_root_environment(),
                        result
                    ).unwrap();
                    let expected: Vec<(&str, Value)> = $expected;

                    match result {
                        Value::Object(object_id) => {
                            for (name, value) in expected {
                                let symbol = interpreter.intern(name);

                                let expected = value;
                                let result = interpreter.get_object_item(
                                    object_id,
                                    symbol
                                ).unwrap().unwrap();
                                println!("{:?} {:?}", expected, result);

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
        fn reads_elements_correctly() {
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
                    //("f", Value::Symbol(SymbolId::new(7))),
                    ("f", Value::Symbol(SymbolId::new(41))), // it should be 7
                    ("g", Value::String(StringId::new(0))),
                ),
                "{:a 1 :b 1.1 :c #t :d #f :e :keyword :f 'symbol :g \"string\"}"
            );
        }
    }

    fn assert_reading_deeply(interpreter: &mut Interpreter, expected: Value, code: &str) {
        if let Ok((_, program)) = parse_code(code) {
            let result = read_elements(
                interpreter,
                program.get_elements()
            ).unwrap().remove(0);

            assertion::assert_deep_equal(interpreter, expected, result);
        }
    }

    #[test]
    fn reads_delimited_symbols_element_correctly() {
        let mut interpreter = Interpreter::new();

        let symbol = interpreter.intern_symbol_value("object");
        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            symbol,
            nil
        );

        let keyword = interpreter.intern_keyword_value(String::from("value"));
        let expected = interpreter.make_cons_value(
            keyword,
            cdr
        );

        assert_reading_deeply(&mut interpreter, expected, "object:value");

        let symbol = interpreter.intern_symbol_value("object");
        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            symbol,
            nil
        );

        let keyword = interpreter.intern_keyword_value(String::from("value1"));
        let car = interpreter.make_cons_value(
            keyword,
            cdr
        );

        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            car,
            nil
        );

        let keyword = interpreter.intern_keyword_value(String::from("value2"));
        let expected = interpreter.make_cons_value(
            keyword,
            cdr
        );

        assert_reading_deeply(&mut interpreter, expected, "object:value1:value2");

        let symbol = interpreter.intern_symbol_value("object");
        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            symbol,
            nil
        );

        let keyword = interpreter.intern_keyword_value(String::from("value1"));
        let car = interpreter.make_cons_value(
            keyword,
            cdr
        );

        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            car,
            nil
        );

        let keyword = interpreter.intern_keyword_value(String::from("value2"));
        let car = interpreter.make_cons_value(
            keyword,
            cdr
        );

        let nil = interpreter.intern_nil_symbol_value();
        let expected = interpreter.make_cons_value(
            car,
            nil
        );

        assert_reading_deeply(&mut interpreter, expected, "(object:value1:value2)");
    }

    macro_rules! assert_prefix_result_equal {
        ($prefix:expr, $prefix_after:expr, $code: expr) => {
            if let Ok((_, program)) = parse_code($code) {
                let mut interpreter = Interpreter::new();
                let expected = read_elements(&mut interpreter, program.get_elements()).unwrap()[0];

                let nil = interpreter.intern_nil_symbol_value();
                let expected = interpreter.make_cons_value(
                    expected,
                    nil
                );

                let symbol = interpreter.intern_symbol_value($prefix_after);
                let expected = interpreter.make_cons_value(
                    symbol,
                    expected
                );

                let prefixed_code = concat!($prefix, $code);

                assert_reading_result_equal!(
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
    fn reads_quoted_values_correctly() {
        assert_prefix_values_works!("'", "quote");
        assert_prefix_values_works!("`", "`");
        assert_prefix_values_works!(",", ",");
        assert_prefix_values_works!(",@", ",@");
    }

    #[test]
    fn reads_complex_s_expression_correctly() {
        let mut interpreter = Interpreter::new();

        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            Value::Integer(4),
                nil
        );

        let car = interpreter.make_cons_value(
            Value::Integer(3),
            cdr
        );

        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            Value::Boolean(false),
            nil
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

        let symbol = interpreter.intern_symbol_value("a");
        let expected = interpreter.make_cons_value(
            symbol,
            cdr
        );

        assert_reading_deeply(&mut interpreter, expected, "(a 1 2.3 #t (3 4) #f)");
    }
}