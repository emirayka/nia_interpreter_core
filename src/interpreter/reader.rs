use std::cmp::max;
use std::convert::TryInto;

use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::parser::DelimitedSymbolsElement;
use crate::parser::Element;
use crate::parser::ObjectElement;
use crate::parser::ObjectPatternElement;
use crate::parser::ShortLambdaElement;
use crate::parser::{Prefix, PrefixedElement};
use crate::parser::{SExpressionElement, SymbolElement};

use crate::interpreter::library;

fn read_regular_s_expression(
    interpreter: &mut Interpreter,
    first_value: Element,
    s_expressions: Vec<Element>,
) -> Result<Value, Error> {
    let mut values = vec![read_element(interpreter, first_value)?];

    for s_expression in s_expressions {
        let element = read_element(interpreter, s_expression)?;

        values.push(element);
    }

    let list = interpreter.vec_to_list(values);

    Ok(list)
}

fn read_object_method_invocation_s_expression(
    interpreter: &mut Interpreter,
    delimited_symbols_element: DelimitedSymbolsElement,
    s_expressions: Vec<Element>,
) -> Result<Value, Error> {
    let mut s_expressions = s_expressions;

    let object_method_invocation =
        read_delimited_symbols_element(interpreter, delimited_symbols_element);

    let car = interpreter.get_car(object_method_invocation.try_into()?)?;

    let with_this_symbol_value = interpreter.intern_symbol_value("with-this");

    let result = if library::deep_equal(interpreter, car, with_this_symbol_value)? {
        let cdr = interpreter.get_cdr(object_method_invocation.try_into()?)?;
        let cddr = interpreter.get_cdr(cdr.try_into()?)?;
        let caddr = interpreter.get_car(cddr.try_into()?)?;

        let cdddr = read_elements(interpreter, s_expressions)?;
        let cdddr = interpreter.vec_to_list(cdddr);

        let new_caddr = interpreter.make_cons_value(caddr, cdddr);

        interpreter.set_car(cddr.try_into()?, new_caddr)?;

        object_method_invocation
    } else {
        let cdr = read_elements(interpreter, s_expressions)?;
        let cdr = interpreter.vec_to_list(cdr);

        interpreter.make_cons_value(object_method_invocation, cdr)
    };

    Ok(result)
}

fn read_s_expression(
    interpreter: &mut Interpreter,
    sexp_element: SExpressionElement,
) -> Result<Value, Error> {
    if sexp_element.get_values_ref().len() == 0 {
        return Ok(interpreter.intern_nil_symbol_value());
    }

    let mut s_expressions = sexp_element.get_values();
    let first_element = s_expressions.remove(0);

    if let Element::DelimitedSymbols(delimited_symbols_element) = first_element {
        read_object_method_invocation_s_expression(
            interpreter,
            delimited_symbols_element,
            s_expressions,
        )
    } else {
        read_regular_s_expression(interpreter, first_element, s_expressions)
    }
    // read_regular_s_expression(
    //     interpreter,
    //     first_element,
    //     s_expressions,
    // )
}

fn count_short_lambda_argument_count(
    _interpreter: &mut Interpreter,
    short_lambda_element: &ShortLambdaElement,
) -> u8 {
    let mut candidates = Vec::new();

    for element in short_lambda_element.get_value_ref().get_values_ref() {
        candidates.push(element);
    }

    let mut count = 0;

    loop {
        if candidates.len() == 0 {
            break;
        }

        let candidate = candidates.remove(0);

        match candidate {
            Element::Symbol(symbol_element) => {
                let name = symbol_element.get_value();

                match (&name['%'.len_utf8()..]).parse::<u8>() {
                    Ok(val) => {
                        count = max(count, val);
                    }
                    _ => {}
                }
            }
            Element::Prefix(prefix_element) => {
                candidates.push(prefix_element.get_value_ref());
            }
            Element::SExpression(s_expression_element) => {
                for element in s_expression_element.get_values_ref() {
                    candidates.push(element);
                }
            }
            Element::Object(object_element) => {
                for (_, element) in object_element.get_values_ref() {
                    candidates.push(element)
                }
            }
            _ => {}
        }
    }

    count
}

fn make_short_lambda_argument_list(interpreter: &mut Interpreter, count: u8) -> Value {
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
    short_lambda_element: ShortLambdaElement,
) -> Result<Value, Error> {
    let function = interpreter.intern_symbol_value("function");
    let lambda = interpreter.intern_symbol_value("lambda");
    let nil = interpreter.intern_nil_symbol_value();

    let argument_count = count_short_lambda_argument_count(interpreter, &short_lambda_element);
    let code = read_s_expression(interpreter, short_lambda_element.get_value())?;
    let arguments = make_short_lambda_argument_list(interpreter, argument_count);

    let cdr = interpreter.make_cons_value(code, nil);
    let cdr = interpreter.make_cons_value(arguments, cdr);
    let car = interpreter.make_cons_value(lambda, cdr);

    let cdr = interpreter.make_cons_value(car, nil);
    let value = interpreter.make_cons_value(function, cdr);

    Ok(value)
}

fn read_object(
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

fn read_object_pattern(
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

fn expand_delimited_symbols(interpreter: &mut Interpreter, values: &[SymbolElement]) -> Value {
    let object_symbol_name = values[0].get_value();
    let mut previous_cons = interpreter.intern_symbol_value(object_symbol_name);

    for symbol_element in &values[1..] {
        let symbol_name = symbol_element.get_value();

        let nil = interpreter.intern_nil_symbol_value();
        let current_cons = Value::Cons(interpreter.make_cons(previous_cons, nil));

        let keyword = interpreter.intern_keyword_value(symbol_name);

        let current_cons = Value::Cons(interpreter.make_cons(keyword, current_cons));

        previous_cons = current_cons;
    }

    previous_cons
}

fn read_delimited_symbols_element_as_this_invocation(
    interpreter: &mut Interpreter,
    values: &Vec<SymbolElement>,
) -> Value {
    expand_delimited_symbols(interpreter, values)
}

fn read_delimited_symbols_element_as_object_method_invocation(
    interpreter: &mut Interpreter,
    values: &Vec<SymbolElement>,
) -> Value {
    let value_of_this_object = expand_delimited_symbols(interpreter, &values[..(values.len() - 1)]);

    // construct this invocation
    let this_symbol_value = interpreter.intern_symbol_value("this");
    let this_object_property_keyword =
        interpreter.intern_keyword_value(values.last().unwrap().get_value());

    let this_invocation_value =
        interpreter.vec_to_list(vec![this_object_property_keyword, this_symbol_value]);

    // construct with-this invocation
    let with_this_symbol_value = interpreter.intern_symbol_value("with-this");

    let result = interpreter.vec_to_list(vec![
        with_this_symbol_value,
        value_of_this_object,
        this_invocation_value,
    ]);

    result
}

fn read_delimited_symbols_element(
    interpreter: &mut Interpreter,
    delimited_symbols_element: DelimitedSymbolsElement,
) -> Value {
    let values = delimited_symbols_element.get_symbols();

    if values[0].get_value() == "this" || values[0].get_value() == "super" {
        read_delimited_symbols_element_as_this_invocation(interpreter, values)
    } else {
        read_delimited_symbols_element_as_object_method_invocation(interpreter, values)
    }
}

fn read_quote_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let quote = interpreter.intern_symbol_value("quote");

    let quoted_value = interpreter.vec_to_list(vec![quote, value]);

    Ok(quoted_value)
}

fn read_sharp_quote_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let quote = interpreter.intern_symbol_value("quote");
    let flookup = interpreter.intern_symbol_value("flookup");

    let quoted_value = interpreter.vec_to_list(vec![quote, value]);
    let flooked_up_value = interpreter.vec_to_list(vec![flookup, quoted_value]);

    Ok(flooked_up_value)
}

fn read_graveaccent_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let graveaccent = interpreter.intern_symbol_value("`");

    let nil = interpreter.intern_nil_symbol_value();
    let cdr = Value::Cons(interpreter.make_cons(value, nil));

    let cons_id = interpreter.make_cons(graveaccent, cdr);

    Ok(Value::Cons(cons_id))
}

fn read_comma_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let comma = interpreter.intern_symbol_value(",");

    let nil = interpreter.intern_nil_symbol_value();
    let cdr = Value::Cons(interpreter.make_cons(value, nil));

    let cons_id = interpreter.make_cons(comma, cdr);

    Ok(Value::Cons(cons_id))
}

fn read_commadog_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let commadog = interpreter.intern_symbol_value(",@");

    let nil = interpreter.intern_nil_symbol_value();
    let cdr = Value::Cons(interpreter.make_cons(value, nil));

    let cons_id = interpreter.make_cons(commadog, cdr);

    Ok(Value::Cons(cons_id))
}

fn read_prefix_element(
    interpreter: &mut Interpreter,
    prefix_element: PrefixedElement,
) -> Result<Value, Error> {
    match prefix_element.get_prefix() {
        Prefix::Quote => read_quote_prefixed_element(interpreter, prefix_element.get_value()),
        Prefix::SharpQuote => {
            read_sharp_quote_prefixed_element(interpreter, prefix_element.get_value())
        }
        Prefix::GraveAccent => {
            read_graveaccent_prefixed_element(interpreter, prefix_element.get_value())
        }
        Prefix::Comma => read_comma_prefixed_element(interpreter, prefix_element.get_value()),
        Prefix::CommaDog => read_commadog_prefixed_element(interpreter, prefix_element.get_value()),
    }
}

pub fn read_element(interpreter: &mut Interpreter, element: Element) -> Result<Value, Error> {
    let value = match element {
        Element::Integer(integer_element) => Value::Integer(integer_element.get_value()),
        Element::Float(float_element) => Value::Float(float_element.get_value()),
        Element::Boolean(boolean_element) => Value::Boolean(boolean_element.get_value()),
        Element::String(string_element) => {
            let string = string_element.get_value();

            interpreter.intern_string_value(&string)
        }
        Element::Symbol(symbol_element) => {
            let symbol_name = symbol_element.get_value();
            interpreter.intern_symbol_value(symbol_name)
        }
        Element::Keyword(keyword_element) => {
            let keyword_name = keyword_element.get_value();

            interpreter.intern_keyword_value(keyword_name)
        }
        Element::SExpression(sexp_element) => read_s_expression(interpreter, sexp_element)?,
        Element::Object(object_element) => read_object(interpreter, object_element)?,
        Element::ObjectPattern(object_pattern_element) => {
            read_object_pattern(interpreter, object_pattern_element)?
        }
        Element::DelimitedSymbols(delimited_symbols_element) => {
            read_delimited_symbols_element(interpreter, delimited_symbols_element)
        }
        Element::Prefix(prefix_element) => read_prefix_element(interpreter, prefix_element)?,
        Element::ShortLambda(short_lambda_element) => {
            read_short_lambda(interpreter, short_lambda_element)?
        }
    };

    Ok(value)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::parser::parse;
    #[allow(unused_imports)]
    use crate::utils::assertion;

    macro_rules! assert_reading_result_equal {
        ($expected:expr, $code:expr) => {
            let mut interpreter = Interpreter::new();
            let expected = $expected;

            if let Ok(program) = parse($code) {
                let result = read_elements(&mut interpreter, program.get_elements()).unwrap();

                let len = expected.len();

                nia_assert_equal(len, result.len());

                for i in 0..len {
                    let expected = expected[i];
                    let result = result[i];

                    assertion::assert_deep_equal(&mut interpreter, expected, result);
                }
            }
        };
    }

    #[test]
    fn reads_integer_elements_correctly() {
        assert_reading_result_equal!(vec!(Value::Integer(1)), "1");

        assert_reading_result_equal!(vec!(Value::Integer(1), Value::Integer(2)), "1 2");
    }

    #[test]
    fn reads_float_elements_correctly() {
        assert_reading_result_equal!(vec!(Value::Float(1.2)), "1.2");

        assert_reading_result_equal!(vec!(Value::Float(1.2), Value::Float(3.4)), "1.2 3.4");
    }

    #[test]
    fn reads_boolean_elements_correctly() {
        assert_reading_result_equal!(vec!(Value::Boolean(true)), "#t");

        assert_reading_result_equal!(vec!(Value::Boolean(true), Value::Boolean(false)), "#t #f");
    }

    #[test]
    fn reads_string_elements_correctly() {
        let mut interpreter = Interpreter::new();

        assert_reading_result_equal!(
            vec!(interpreter.intern_string_value("cute string")),
            r#""cute string""#
        );

        let mut interpreter = Interpreter::new();
        assert_reading_result_equal!(
            vec!(
                interpreter.intern_string_value("first cute string"),
                interpreter.intern_string_value("second cute string"),
            ),
            r#""first cute string" "second cute string""#
        );
    }

    #[test]
    fn reads_symbol_elements_correctly() {
        let mut interpreter = Interpreter::new();

        assert_reading_result_equal!(
            vec!(interpreter.intern_symbol_value("cutesymbol")),
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
            vec!(interpreter.intern_keyword_value("cutekeyword")),
            r#":cutekeyword"#
        );

        let mut interpreter = Interpreter::new();
        assert_reading_result_equal!(
            vec!(
                interpreter.intern_keyword_value("cutekeyword1"),
                interpreter.intern_keyword_value("cutekeyword1"),
                interpreter.intern_keyword_value("cutekeyword2")
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
        let nil = interpreter.intern_nil_symbol_value();
        let expected = interpreter.make_cons_value(symbol, nil);
        assert_reading_deeply(&mut interpreter, expected, "(a)");

        let symbol = interpreter.intern_symbol_value("b");
        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(symbol, nil);
        let symbol = interpreter.intern_symbol_value("a");
        let expected = interpreter.make_cons_value(symbol, cdr);
        assert_reading_deeply(&mut interpreter, expected, "(a b)");
    }

    #[cfg(test)]
    mod short_lambda {
        use super::*;

        fn make_short_lambda(
            interpreter: &mut Interpreter,
            arguments: Value,
            body: Value,
        ) -> Value {
            let nil = interpreter.intern_nil_symbol_value();
            let function = interpreter.intern_symbol_value("function");
            let lambda = interpreter.intern_symbol_value("lambda");

            let cdr = interpreter.make_cons_value(body, nil);
            let cdr = interpreter.make_cons_value(arguments, cdr);
            let car = interpreter.make_cons_value(lambda, cdr);

            let cdr = interpreter.make_cons_value(car, nil);
            let expected = interpreter.make_cons_value(function, cdr);

            expected
        }

        fn assert_short_lambda_valid(
            interpreter: &mut Interpreter,
            arguments: Value,
            body: Value,
            code: &str,
        ) {
            let expected = make_short_lambda(interpreter, arguments, body);

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

        #[test]
        fn reads_short_lambda_with_different_count_of_arguments_correctly() {
            let mut interpreter = Interpreter::new();

            let plus = interpreter.intern_symbol_value("+");
            let arg1 = interpreter.intern_symbol_value("%1");
            let arg2 = interpreter.intern_symbol_value("%2");
            let nil = interpreter.intern_nil_symbol_value();

            // inner lambda
            let cdr = interpreter.make_cons_value(arg2, nil);
            let cdr = interpreter.make_cons_value(arg1, cdr);
            let body = interpreter.make_cons_value(plus, cdr);

            let cdr = interpreter.make_cons_value(arg2, nil);
            let arguments = interpreter.make_cons_value(arg1, cdr);

            let inner = make_short_lambda(&mut interpreter, arguments, body);

            // outer lambda
            let cdr = interpreter.make_cons_value(arg1, nil);
            let cdr = interpreter.make_cons_value(arg1, cdr);
            let body = interpreter.make_cons_value(inner, cdr);

            let arguments = interpreter.make_cons_value(arg1, nil);

            assert_short_lambda_valid(&mut interpreter, arguments, body, "#(#(+ %1 %2) %1 %1)");
        }
    }

    fn assert_object_has_items(
        interpreter: &mut Interpreter,
        code: &str,
        expected: Vec<(&str, Value)>,
    ) {
        if let Ok(code) = parse(code) {
            let result = read_elements(interpreter, code.get_elements())
                .unwrap()
                .remove(0);

            let result = interpreter
                .execute_value(interpreter.get_main_environment_id(), result)
                .unwrap();

            match result {
                Value::Object(object_id) => {
                    for (name, value) in expected {
                        let symbol = interpreter.intern(name);

                        let expected = value;
                        let result = interpreter
                            .get_object_property(object_id, symbol)
                            .unwrap()
                            .unwrap();

                        assertion::assert_deep_equal(interpreter, expected, result);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            panic!();
        }
    }

    #[cfg(test)]
    mod object {
        use super::*;

        #[test]
        fn reads_elements_correctly() {
            let mut interpreter = Interpreter::new();

            assert_object_has_items(&mut interpreter, "{}", vec![]);
            assert_object_has_items(&mut interpreter, "{:a 1}", vec![("a", Value::Integer(1))]);
            assert_object_has_items(
                &mut interpreter,
                "{:a 1 :b 2}",
                vec![("a", Value::Integer(1)), ("b", Value::Integer(2))],
            );
        }

        #[test]
        fn evaluates_items_correctly() {
            let mut interpreter = Interpreter::new();

            // todo: uncomment two lines below, and find out why it doesn't work
            //            let keyword_value = interpreter.intern_keyword_value("keyword");
            let symbol_value = interpreter.intern_symbol_value("symbol");
            let string_value = interpreter.intern_string_value("string");

            assert_object_has_items(
                &mut interpreter,
                "{:a 1 :b 1.1 :c #t :d #f :e :keyword :f 'symbol :g \"string\"}",
                vec![
                    ("a", Value::Integer(1)),
                    ("b", Value::Float(1.1)),
                    ("c", Value::Boolean(true)),
                    ("d", Value::Boolean(false)),
                    //                    ("e", keyword_value),
                    ("f", symbol_value),
                    ("g", string_value),
                ],
            );
        }
    }

    #[cfg(test)]
    mod object_pattern {
        use super::*;

        #[test]
        fn reads_elements_correctly() {
            let mut interpreter = Interpreter::new();

            let a = interpreter.intern_symbol_value("a");
            let b = interpreter.intern_symbol_value("b");

            assert_object_has_items(&mut interpreter, "#{}", vec![]);

            assert_object_has_items(&mut interpreter, "#{:a}", vec![("a", a)]);

            assert_object_has_items(&mut interpreter, "#{:a :b}", vec![("a", a), ("b", b)]);
        }
    }

    fn assert_reading_deeply(interpreter: &mut Interpreter, expected: Value, code: &str) {
        if let Ok(program) = parse(code) {
            let result = read_elements(interpreter, program.get_elements())
                .unwrap()
                .remove(0);

            assertion::assert_deep_equal(interpreter, expected, result);
        }
    }

    #[cfg(test)]
    mod delimited_symbols_element {
        use super::*;

        #[test]
        fn reads_delimited_symbols_element_with_this_correctly() {
            let mut interpreter = Interpreter::new();

            let nil_symbol_value = interpreter.intern_nil_symbol_value();
            let this_symbol_value = interpreter.intern_symbol_value("this");
            let value1_keyword_value = interpreter.intern_keyword_value("value1");
            let value2_keyword_value = interpreter.intern_keyword_value("value2");

            let expected = interpreter.vec_to_list(vec![value1_keyword_value, this_symbol_value]);

            assert_reading_deeply(&mut interpreter, expected, "this:value1");

            let expected = interpreter.vec_to_list(vec![value2_keyword_value, expected]);

            assert_reading_deeply(&mut interpreter, expected, "this:value1:value2");

            let expected = interpreter.make_cons_value(expected, nil_symbol_value);
            assert_reading_deeply(&mut interpreter, expected, "(this:value1:value2)");
        }

        #[test]
        fn reads_delimited_symbols_element_with_super_correctly() {
            let mut interpreter = Interpreter::new();

            let nil_symbol_value = interpreter.intern_nil_symbol_value();
            let super_symbol_value = interpreter.intern_symbol_value("super");
            let value1_keyword_value = interpreter.intern_keyword_value("value1");
            let value2_keyword_value = interpreter.intern_keyword_value("value2");

            let expected = interpreter.vec_to_list(vec![value1_keyword_value, super_symbol_value]);

            assert_reading_deeply(&mut interpreter, expected, "super:value1");

            let expected = interpreter.vec_to_list(vec![value2_keyword_value, expected]);

            assert_reading_deeply(&mut interpreter, expected, "super:value1:value2");

            let expected = interpreter.make_cons_value(expected, nil_symbol_value);
            assert_reading_deeply(&mut interpreter, expected, "(super:value1:value2)");
        }

        #[test]
        fn reads_delimited_symbols_element_object_method_invocation_correctly() {
            let mut interpreter = Interpreter::new();

            let nil_symbol_value = interpreter.intern_nil_symbol_value();
            let this_symbol_value = interpreter.intern_symbol_value("this");
            let with_this_symbol_value = interpreter.intern_symbol_value("with-this");
            let object_symbol_value = interpreter.intern_symbol_value("object");
            let value1_keyword_value = interpreter.intern_keyword_value("value1");
            let value2_keyword_value = interpreter.intern_keyword_value("value2");

            let this_invocation =
                interpreter.vec_to_list(vec![value1_keyword_value, this_symbol_value]);

            let expected = interpreter.vec_to_list(vec![
                with_this_symbol_value,
                object_symbol_value,
                this_invocation,
            ]);

            assert_reading_deeply(&mut interpreter, expected, "object:value1");

            let this_invocation =
                interpreter.vec_to_list(vec![value2_keyword_value, this_symbol_value]);

            let this_object_value =
                interpreter.vec_to_list(vec![value1_keyword_value, object_symbol_value]);

            let expected = interpreter.vec_to_list(vec![
                with_this_symbol_value,
                this_object_value,
                this_invocation,
            ]);

            assert_reading_deeply(&mut interpreter, expected, "object:value1:value2");

            let item3 = interpreter.vec_to_list(vec![value2_keyword_value, this_symbol_value]);

            let item3 = interpreter.vec_to_list(vec![item3]);

            let item2 = interpreter.vec_to_list(vec![value1_keyword_value, object_symbol_value]);

            let expected = interpreter.vec_to_list(vec![with_this_symbol_value, item2, item3]);

            assert_reading_deeply(&mut interpreter, expected, "(object:value1:value2)");
        }
    }

    macro_rules! assert_prefix_result_equal {
        ($prefix:expr, $prefix_after:expr, $code: expr) => {
            if let Ok(program) = parse($code) {
                let mut interpreter = Interpreter::new();
                let expected = read_elements(&mut interpreter, program.get_elements()).unwrap()[0];

                let nil = interpreter.intern_nil_symbol_value();
                let expected = interpreter.make_cons_value(expected, nil);

                let symbol = interpreter.intern_symbol_value($prefix_after);
                let expected = interpreter.make_cons_value(symbol, expected);

                let prefixed_code = concat!($prefix, $code);

                assert_reading_result_equal!(vec!(expected), prefixed_code);
            }
        };
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
        };
    }

    #[test]
    fn reads_quoted_values_correctly() {
        assert_prefix_values_works!("'", "quote");
        assert_prefix_values_works!("`", "`");
        assert_prefix_values_works!(",", ",");
        assert_prefix_values_works!(",@", ",@");
    }

    #[test]
    fn reads_sharp_quoted_values_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("'#'1", "'(flookup (quote 1))"),
            ("'#'1.1", "'(flookup (quote 1.1))"),
            ("'#'#t", "'(flookup (quote #t))"),
            ("'#'#f", "'(flookup (quote #f))"),
            ("'#'\"string\"", "'(flookup (quote \"string\"))"),
            ("'#'symbol", "'(flookup (quote symbol))"),
            ("'#':keyword", "'(flookup (quote :keyword))"),
            ("'#''(list)", "'(flookup (quote '(list)))"),
            ("'#'{}", "'(flookup (quote {}))"),
            ("'#'#()", "'(flookup (quote #()))"),
            ("#'flookup", "(flookup (quote flookup))"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn reads_complex_s_expression_correctly() {
        let mut interpreter = Interpreter::new();

        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(Value::Integer(4), nil);

        let car = interpreter.make_cons_value(Value::Integer(3), cdr);

        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(Value::Boolean(false), nil);

        let cdr = interpreter.make_cons_value(car, cdr);

        let cdr = interpreter.make_cons_value(Value::Boolean(true), cdr);

        let cdr = interpreter.make_cons_value(Value::Float(2.3), cdr);

        let cdr = interpreter.make_cons_value(Value::Integer(1), cdr);

        let symbol = interpreter.intern_symbol_value("a");
        let expected = interpreter.make_cons_value(symbol, cdr);

        assert_reading_deeply(&mut interpreter, expected, "(a 1 2.3 #t (3 4) #f)");
    }
}
