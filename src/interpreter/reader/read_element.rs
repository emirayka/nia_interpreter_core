use crate::interpreter::reader::read_boolean_element::read_boolean_element;
use crate::interpreter::reader::read_delimited_symbols_element::read_delimited_symbols_element;
use crate::interpreter::reader::read_float_element::read_float_element;
use crate::interpreter::reader::read_integer_element::read_integer_element;
use crate::interpreter::reader::read_keyword_element::read_keyword_element;
use crate::interpreter::reader::read_object_element::read_object_element;
use crate::interpreter::reader::read_object_pattern_element::read_object_pattern_element;
use crate::interpreter::reader::read_prefixed_element::read_prefixed_element;
use crate::interpreter::reader::read_s_expression_element::read_s_expression_element;
use crate::interpreter::reader::read_short_lambda_element::read_short_lambda_element;
use crate::interpreter::reader::read_string_element::read_string_element;
use crate::interpreter::reader::read_symbol_element::read_symbol_element;

use crate::parser::Element;

use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn read_element(
    interpreter: &mut Interpreter,
    element: Element,
) -> Result<Value, Error> {
    let value = match element {
        Element::Integer(integer_element) => {
            read_integer_element(integer_element)?
        }
        Element::Float(float_element) => read_float_element(float_element)?,
        Element::Boolean(boolean_element) => {
            read_boolean_element(boolean_element)?
        }
        Element::String(string_element) => {
            read_string_element(interpreter, string_element)?
        }
        Element::Symbol(symbol_element) => {
            read_symbol_element(interpreter, symbol_element)?
        }
        Element::Keyword(keyword_element) => {
            read_keyword_element(interpreter, keyword_element)?
        }
        Element::SExpression(sexp_element) => {
            read_s_expression_element(interpreter, sexp_element)?
        }
        Element::Object(object_element) => {
            read_object_element(interpreter, object_element)?
        }
        Element::ObjectPattern(object_pattern_element) => {
            read_object_pattern_element(interpreter, object_pattern_element)?
        }
        Element::DelimitedSymbols(delimited_symbols_element) => {
            read_delimited_symbols_element(
                interpreter,
                delimited_symbols_element,
            )
        }
        Element::Prefix(prefixed_element) => {
            read_prefixed_element(interpreter, prefixed_element)?
        }
        Element::ShortLambda(short_lambda_element) => {
            read_short_lambda_element(interpreter, short_lambda_element)?
        }
    };

    Ok(value)
}
