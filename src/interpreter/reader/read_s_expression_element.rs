use std::convert::TryInto;

use crate::interpreter::parser::DelimitedSymbolsElement;
use crate::interpreter::parser::Element;
use crate::interpreter::parser::SExpressionElement;

use crate::interpreter::reader::read_delimited_symbols_element::{
    read_delimited_symbols_element,
    read_delimited_symbols_element_as_object_method_invocation,
};
use crate::interpreter::reader::read_element::read_element;
use crate::interpreter::reader::read_elements::read_elements;

use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

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
    arguments: Vec<Element>,
) -> Result<Value, Error> {
    if delimited_symbols_element.context_needs_to_be_set() {
        let result =
            read_delimited_symbols_element_as_object_method_invocation(
                interpreter,
                delimited_symbols_element.get_symbols(),
                arguments,
            )?;

        Ok(result)
    } else {
        let car = read_delimited_symbols_element(
            interpreter,
            delimited_symbols_element,
        );

        let arguments = read_elements(interpreter, arguments)?;
        let argument_list = interpreter.vec_to_list(arguments);

        let result = interpreter.make_cons_value(car, argument_list);

        Ok(result)
    }
}

pub fn read_s_expression_element(
    interpreter: &mut Interpreter,
    sexp_element: SExpressionElement,
) -> Result<Value, Error> {
    if sexp_element.get_values_ref().len() == 0 {
        return Ok(interpreter.intern_nil_symbol_value());
    }

    let mut s_expressions = sexp_element.get_values();
    let first_element = s_expressions.remove(0);

    if let Element::DelimitedSymbols(delimited_symbols_element) = first_element
    {
        read_object_method_invocation_s_expression(
            interpreter,
            delimited_symbols_element,
            s_expressions,
        )
    } else {
        read_regular_s_expression(interpreter, first_element, s_expressions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::parser::SymbolElement;

    #[test]
    fn reads_s_expression_elements_correctly() {
        let mut interpreter = Interpreter::new();

        let nil_symbol_value = interpreter.intern_nil_symbol_value();
        let a_symbol_value = interpreter.intern_symbol_value("a");
        let b_symbol_value = interpreter.intern_symbol_value("b");
        let c_symbol_value = interpreter.intern_symbol_value("c");

        let empty_list = nil_symbol_value;
        let one_value_list =
            interpreter.make_cons_value(a_symbol_value, nil_symbol_value);
        let two_value_list =
            interpreter.make_cons_value(b_symbol_value, one_value_list);
        let three_value_list =
            interpreter.make_cons_value(c_symbol_value, two_value_list);

        let a_symbol_element =
            Element::Symbol(SymbolElement::new(String::from("a")));
        let b_symbol_element =
            Element::Symbol(SymbolElement::new(String::from("b")));
        let c_symbol_element =
            Element::Symbol(SymbolElement::new(String::from("c")));

        let empty_s_expression = SExpressionElement::new(vec![]);
        let one_value_s_expression =
            SExpressionElement::new(vec![a_symbol_element.clone()]);
        let two_value_s_expression = SExpressionElement::new(vec![
            b_symbol_element.clone(),
            a_symbol_element.clone(),
        ]);
        let three_value_s_expression = SExpressionElement::new(vec![
            c_symbol_element.clone(),
            b_symbol_element.clone(),
            a_symbol_element.clone(),
        ]);

        let specs = vec![
            (empty_list, empty_s_expression),
            (one_value_list, one_value_s_expression),
            (two_value_list, two_value_s_expression),
            (three_value_list, three_value_s_expression),
        ];

        for (expected_value, s_expression_element) in specs {
            let result_value = read_s_expression_element(
                &mut interpreter,
                s_expression_element,
            )
            .unwrap();

            crate::utils::assert_deep_equal(
                &mut interpreter,
                expected_value,
                result_value,
            );
        }
    }

    #[test]
    fn reads_nested_s_expressions_correctly() {
        let mut interpreter = Interpreter::new();

        let nil_symbol_value = interpreter.intern_nil_symbol_value();

        let zero_level_list = nil_symbol_value;
        let one_level_list =
            interpreter.make_cons_value(zero_level_list, nil_symbol_value);
        let two_level_list =
            interpreter.make_cons_value(one_level_list, nil_symbol_value);
        let three_level_list =
            interpreter.make_cons_value(two_level_list, nil_symbol_value);

        let zero_level_s_expression = SExpressionElement::new(vec![]);
        let one_level_s_expression =
            SExpressionElement::new(vec![Element::SExpression(
                zero_level_s_expression.clone(),
            )]);
        let two_level_s_expression =
            SExpressionElement::new(vec![Element::SExpression(
                one_level_s_expression.clone(),
            )]);
        let three_level_s_expression =
            SExpressionElement::new(vec![Element::SExpression(
                two_level_s_expression.clone(),
            )]);

        let specs = vec![
            (zero_level_list, zero_level_s_expression),
            (one_level_list, one_level_s_expression),
            (two_level_list, two_level_s_expression),
            (three_level_list, three_level_s_expression),
        ];

        for (expected_value, s_expression_element) in specs {
            let result_value = read_s_expression_element(
                &mut interpreter,
                s_expression_element,
            )
            .unwrap();

            crate::utils::assert_deep_equal(
                &mut interpreter,
                expected_value,
                result_value,
            );
        }
    }

    // todo: a bit more complex test, smth like that '(a b (c d) e)
}
