use crate::parser::DelimitedSymbolsElement;
use crate::parser::SymbolElement;
use crate::Interpreter;
use crate::Value;

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
    let value_of_this_object =
        expand_delimited_symbols(interpreter, &values[..(values.len() - 1)]);

    // construct this invocation
    let this_symbol_value = interpreter.intern_symbol_value("this");
    let this_object_property_keyword =
        interpreter.intern_keyword_value(values.last().unwrap().get_value());

    let this_invocation_value = interpreter
        .vec_to_list(vec![this_object_property_keyword, this_symbol_value]);

    // construct with-this invocation
    let with_this_symbol_value = interpreter.intern_symbol_value("with-this");

    let result = interpreter.vec_to_list(vec![
        with_this_symbol_value,
        value_of_this_object,
        this_invocation_value,
    ]);

    result
}

pub fn read_delimited_symbols_element(
    interpreter: &mut Interpreter,
    delimited_symbols_element: DelimitedSymbolsElement,
) -> Value {
    let values = delimited_symbols_element.get_symbols();

    if values[0].get_value() == "this" || values[0].get_value() == "super" {
        read_delimited_symbols_element_as_this_invocation(interpreter, values)
    } else {
        read_delimited_symbols_element_as_object_method_invocation(
            interpreter,
            values,
        )
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::interpreter::reader::read_element::read_element;
    use crate::parser::parse;

    use crate::utils::assertion::assert_parsing_reading_result_is_correct;

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

        let nil_symbol_value = interpreter.intern_nil_symbol_value();
        let this_symbol_value = interpreter.intern_symbol_value("this");
        let with_this_symbol_value =
            interpreter.intern_symbol_value("with-this");
        let object_symbol_value = interpreter.intern_symbol_value("object");
        let value1_keyword_value = interpreter.intern_keyword_value("value1");
        let value2_keyword_value = interpreter.intern_keyword_value("value2");

        let this_invocation = interpreter
            .vec_to_list(vec![value1_keyword_value, this_symbol_value]);

        let expected = interpreter.vec_to_list(vec![
            with_this_symbol_value,
            object_symbol_value,
            this_invocation,
        ]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "object:value1",
        );

        let this_invocation = interpreter
            .vec_to_list(vec![value2_keyword_value, this_symbol_value]);

        let this_object_value = interpreter
            .vec_to_list(vec![value1_keyword_value, object_symbol_value]);

        let expected = interpreter.vec_to_list(vec![
            with_this_symbol_value,
            this_object_value,
            this_invocation,
        ]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "object:value1:value2",
        );

        let item3 = interpreter
            .vec_to_list(vec![value2_keyword_value, this_symbol_value]);

        let item3 = interpreter.vec_to_list(vec![item3]);

        let item2 = interpreter
            .vec_to_list(vec![value1_keyword_value, object_symbol_value]);

        let expected =
            interpreter.vec_to_list(vec![with_this_symbol_value, item2, item3]);

        assert_parsing_reading_result_is_correct(
            &mut interpreter,
            expected,
            "(object:value1:value2)",
        );
    }
}
