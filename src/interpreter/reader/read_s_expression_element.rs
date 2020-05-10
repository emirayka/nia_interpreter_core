use std::convert::TryInto;

use crate::interpreter::reader::read_delimited_symbols_element::read_delimited_symbols_element;
use crate::interpreter::reader::read_element::read_element;
use crate::interpreter::reader::read_elements::read_elements;

use crate::parser::DelimitedSymbolsElement;
use crate::parser::Element;
use crate::parser::SExpressionElement;

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
    s_expressions: Vec<Element>,
) -> Result<Value, Error> {
    let mut s_expressions = s_expressions;

    let object_method_invocation =
        read_delimited_symbols_element(interpreter, delimited_symbols_element);

    let car = interpreter.get_car(object_method_invocation.try_into()?)?;

    let with_this_symbol_value = interpreter.intern_symbol_value("with-this");

    let result =
        if library::deep_equal(interpreter, car, with_this_symbol_value)? {
            let cdr =
                interpreter.get_cdr(object_method_invocation.try_into()?)?;
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
    use crate::parser::SymbolElement;

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
        let a_symbol_value = interpreter.intern_symbol_value("a");
        let b_symbol_value = interpreter.intern_symbol_value("b");
        let c_symbol_value = interpreter.intern_symbol_value("c");

        let zero_level_list = nil_symbol_value;
        let one_level_list =
            interpreter.make_cons_value(zero_level_list, nil_symbol_value);
        let two_level_list =
            interpreter.make_cons_value(one_level_list, nil_symbol_value);
        let three_level_list =
            interpreter.make_cons_value(two_level_list, nil_symbol_value);

        let a_symbol_element =
            Element::Symbol(SymbolElement::new(String::from("a")));
        let b_symbol_element =
            Element::Symbol(SymbolElement::new(String::from("b")));
        let c_symbol_element =
            Element::Symbol(SymbolElement::new(String::from("c")));

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
