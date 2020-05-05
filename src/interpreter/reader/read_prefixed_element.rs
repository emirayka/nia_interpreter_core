use crate::interpreter::reader::read_element::read_element;

use crate::parser::{Element, Prefix, PrefixedElement};

use crate::Error;
use crate::Interpreter;
use crate::Value;

fn construct_prefixed_value(
    interpreter: &mut Interpreter,
    prefix: &str,
    value: Value,
) -> Result<Value, Error> {
    let prefix_symbol_value = interpreter.intern_symbol_value(prefix);

    let nil = interpreter.intern_nil_symbol_value();
    let cdr = interpreter.make_cons(value, nil).into();
    let cons_id = interpreter.make_cons(prefix_symbol_value, cdr);

    Ok(cons_id.into())
}

fn read_quote_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    construct_prefixed_value(interpreter, "quote", value)
}

fn read_sharp_quote_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    let quoted_value = construct_prefixed_value(interpreter, "quote", value)?;
    let flooked_up_value =
        construct_prefixed_value(interpreter, "flookup", quoted_value)?;

    Ok(flooked_up_value)
}

fn read_grave_accent_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;
    construct_prefixed_value(interpreter, "`", value)
}

fn read_comma_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    construct_prefixed_value(interpreter, ",", value)
}

fn read_comma_dog_prefixed_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = read_element(interpreter, element)?;

    construct_prefixed_value(interpreter, ",@", value)
}

pub fn read_prefixed_element(
    interpreter: &mut Interpreter,
    prefix_element: PrefixedElement,
) -> Result<Value, Error> {
    let prefix = prefix_element.get_prefix();
    let prefixed_element = prefix_element.take_value();

    match prefix {
        Prefix::Quote => {
            read_quote_prefixed_element(interpreter, prefixed_element)
        },
        Prefix::SharpQuote => {
            read_sharp_quote_prefixed_element(interpreter, prefixed_element)
        },
        Prefix::GraveAccent => {
            read_grave_accent_prefixed_element(interpreter, prefixed_element)
        },
        Prefix::Comma => {
            read_comma_prefixed_element(interpreter, prefixed_element)
        },
        Prefix::CommaDog => {
            read_comma_dog_prefixed_element(interpreter, prefixed_element)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::IntegerElement;

    #[test]
    fn reads_prefixed_value_correctly() {
        let mut interpreter = Interpreter::new();

        let integer_value = Value::Integer(1);

        let quote_symbol_value = interpreter.intern_symbol_value("quote");
        let flookup_symbol_value = interpreter.intern_symbol_value("flookup");
        let comma_symbol_value = interpreter.intern_symbol_value(",");
        let comma_dog_symbol_value = interpreter.intern_symbol_value(",@");
        let grave_accent_symbol_value = interpreter.intern_symbol_value("`");

        let quoted_integer_value =
            interpreter.vec_to_list(vec![quote_symbol_value, integer_value]);

        let flookup_integer_value = interpreter
            .vec_to_list(vec![flookup_symbol_value, quoted_integer_value]);

        let comma_integer_value =
            interpreter.vec_to_list(vec![comma_symbol_value, integer_value]);

        let comma_dog_integer_value = interpreter
            .vec_to_list(vec![comma_dog_symbol_value, integer_value]);

        let grave_accent_integer_value = interpreter
            .vec_to_list(vec![grave_accent_symbol_value, integer_value]);

        let specs = vec![
            (
                quoted_integer_value,
                PrefixedElement::new(
                    Prefix::Quote,
                    Element::Integer(IntegerElement::new(1)),
                ),
            ),
            (
                flookup_integer_value,
                PrefixedElement::new(
                    Prefix::SharpQuote,
                    Element::Integer(IntegerElement::new(1)),
                ),
            ),
            (
                comma_integer_value,
                PrefixedElement::new(
                    Prefix::Comma,
                    Element::Integer(IntegerElement::new(1)),
                ),
            ),
            (
                comma_dog_integer_value,
                PrefixedElement::new(
                    Prefix::CommaDog,
                    Element::Integer(IntegerElement::new(1)),
                ),
            ),
            (
                grave_accent_integer_value,
                PrefixedElement::new(
                    Prefix::GraveAccent,
                    Element::Integer(IntegerElement::new(1)),
                ),
            ),
        ];

        for (expected_value, prefix_element) in specs {
            let result_value =
                read_prefixed_element(&mut interpreter, prefix_element)
                    .unwrap();

            interpreter.print_value(expected_value);
            interpreter.print_value(result_value);

            crate::utils::assertion::assert_deep_equal(
                &mut interpreter,
                expected_value,
                result_value,
            );
        }
    }
}
