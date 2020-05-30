use crate::interpreter::parser::DelimitedSymbolsElement;
use crate::interpreter::parser::SymbolElement;

use crate::interpreter::reader::read_elements;

use crate::Interpreter;
use crate::Value;
use crate::{Element, Error};

fn expand_delimited_symbols(
    interpreter: &mut Interpreter,
    values: &[SymbolElement],
) -> Value {
    let object_symbol_name = values[0].get_value();
    let mut previous_cons = interpreter.intern_symbol_value(object_symbol_name);

    for symbol_element in &values[1..] {
        let symbol_name = symbol_element.get_value();

        let nil = interpreter.intern_nil_symbol_value();
        let current_cons =
            Value::Cons(interpreter.make_cons(previous_cons, nil));

        let keyword = interpreter.intern_keyword_value(symbol_name);

        let current_cons =
            Value::Cons(interpreter.make_cons(keyword, current_cons));

        previous_cons = current_cons;
    }

    previous_cons
}

fn read_delimited_symbols_elements_as_value_internation(
    interpreter: &mut Interpreter,
    values: &Vec<SymbolElement>,
) -> Value {
    expand_delimited_symbols(interpreter, values)
}

pub fn read_delimited_symbols_element_as_object_method_invocation(
    interpreter: &mut Interpreter,
    values: &Vec<SymbolElement>,
    method_arguments: Vec<Element>,
) -> Result<Value, Error> {
    let call_with_this_symbol_value =
        interpreter.intern_symbol_value("call-with-this");
    let context_value =
        expand_delimited_symbols(interpreter, &values[..(values.len() - 1)]);
    let function_value = expand_delimited_symbols(interpreter, values);

    let mut arguments = read_elements(interpreter, method_arguments)?;

    arguments.insert(0, function_value);
    arguments.insert(0, context_value);
    arguments.insert(0, call_with_this_symbol_value);

    let result = interpreter.vec_to_list(arguments);
    Ok(result)
}

pub fn read_delimited_symbols_element(
    interpreter: &mut Interpreter,
    delimited_symbols_element: DelimitedSymbolsElement,
) -> Value {
    let values = delimited_symbols_element.get_symbols();

    read_delimited_symbols_elements_as_value_internation(interpreter, values)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::utils::assert_parsing_reading_result_is_correct;

    #[test]
    fn reads_delimited_symbols_element_with_this_correctly() {
        let mut interpreter = Interpreter::new();

        let nil_symbol_value = interpreter.intern_nil_symbol_value();
        let this_symbol_value = interpreter.intern_symbol_value("this");
        let value1_keyword_value = interpreter.intern_keyword_value("value1");
        let value2_keyword_value = interpreter.intern_keyword_value("value2");

        let expected = interpreter
            .vec_to_list(vec![value1_keyword_value, this_symbol_value]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "this:value1",
        );

        let expected =
            interpreter.vec_to_list(vec![value2_keyword_value, expected]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "this:value1:value2",
        );

        let expected = interpreter.make_cons_value(expected, nil_symbol_value);
        (&mut interpreter, expected, "(this:value1:value2)");
    }

    #[test]
    fn reads_delimited_symbols_element_with_super_correctly() {
        let mut interpreter = Interpreter::new();

        let nil_symbol_value = interpreter.intern_nil_symbol_value();
        let super_symbol_value = interpreter.intern_symbol_value("super");
        let value1_keyword_value = interpreter.intern_keyword_value("value1");
        let value2_keyword_value = interpreter.intern_keyword_value("value2");

        let expected = interpreter
            .vec_to_list(vec![value1_keyword_value, super_symbol_value]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "super:value1",
        );

        let expected =
            interpreter.vec_to_list(vec![value2_keyword_value, expected]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "super:value1:value2",
        );

        let expected = interpreter.make_cons_value(expected, nil_symbol_value);
        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "(super:value1:value2)",
        );
    }

    #[test]
    fn reads_delimited_symbols_element_object_method_invocation_correctly() {
        let mut interpreter = Interpreter::new();

        let call_with_this = interpreter.intern_symbol_value("call-with-this");
        let object_symbol_value = interpreter.intern_symbol_value("object");
        let value1_keyword_value = interpreter.intern_keyword_value("value1");
        let value2_keyword_value = interpreter.intern_keyword_value("value2");

        let expected = interpreter
            .vec_to_list(vec![value1_keyword_value, object_symbol_value]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "object:value1",
        );

        let expected =
            interpreter.vec_to_list(vec![value2_keyword_value, expected]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "object:value1:value2",
        );

        let value1 = call_with_this;
        let value2 = interpreter
            .vec_to_list(vec![value1_keyword_value, object_symbol_value]);
        let value3 =
            interpreter.vec_to_list(vec![value2_keyword_value, value2]);

        let expected = interpreter.vec_to_list(vec![value1, value2, value3]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "(object:value1:value2)",
        );
    }
}
