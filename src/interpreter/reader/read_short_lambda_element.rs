use std::cmp::max;

use crate::interpreter::reader::read_s_expression_element::read_s_expression_element;

use crate::parser::Element;
use crate::parser::ShortLambdaElement;

use crate::Error;
use crate::Interpreter;
use crate::Value;

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
                    },
                    _ => {},
                }
            },
            Element::Prefix(prefix_element) => {
                candidates.push(prefix_element.get_value_ref());
            },
            Element::SExpression(s_expression_element) => {
                for element in s_expression_element.get_values_ref() {
                    candidates.push(element);
                }
            },
            Element::Object(object_element) => {
                for (_, element) in object_element.get_values_ref() {
                    candidates.push(element)
                }
            },
            _ => {},
        }
    }

    count
}

fn make_short_lambda_argument_list(
    interpreter: &mut Interpreter,
    count: u8,
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

pub fn read_short_lambda_element(
    interpreter: &mut Interpreter,
    short_lambda_element: ShortLambdaElement,
) -> Result<Value, Error> {
    let function = interpreter.intern_symbol_value("function");
    let lambda = interpreter.intern_symbol_value("lambda");
    let nil = interpreter.intern_nil_symbol_value();

    let argument_count =
        count_short_lambda_argument_count(interpreter, &short_lambda_element);
    let code = read_s_expression_element(
        interpreter,
        short_lambda_element.get_value(),
    )?;
    let arguments =
        make_short_lambda_argument_list(interpreter, argument_count);

    let cdr = interpreter.make_cons_value(code, nil);
    let cdr = interpreter.make_cons_value(arguments, cdr);
    let car = interpreter.make_cons_value(lambda, cdr);

    let cdr = interpreter.make_cons_value(car, nil);
    let value = interpreter.make_cons_value(function, cdr);

    Ok(value)
}

// todo: refactor these tests
#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::assert_parsing_reading_result_is_correct;

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

        assert_parsing_reading_result_is_correct(interpreter, expected, code);
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

        assert_short_lambda_valid(
            &mut interpreter,
            arguments,
            body,
            "#(+ 1 %1)",
        );
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

        assert_short_lambda_valid(
            &mut interpreter,
            arguments,
            body,
            "#(+ %1 %2)",
        );
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

        assert_short_lambda_valid(
            &mut interpreter,
            arguments,
            body,
            "#(#(+ %1 %2) %1 %1)",
        );
    }
}
