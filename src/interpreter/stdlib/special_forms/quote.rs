use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn quote(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `quote' must be called with exactly one argument."
        ).into_result();
    }

    let first_argument = values.remove(0);

    Ok(first_argument)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    // todo: ensure this test is fine
    #[test]
    fn quote_works_correctly_when_used_quote_special_form() {
        let mut interpreter = Interpreter::new();
        let nil = interpreter.intern_nil_symbol_value();

        let cdr = interpreter.make_cons_value(
            Value::Integer(2),
            nil
        );
        let cons = interpreter.make_cons_value(
            Value::Integer(1),
            cdr
        );

        assert_eq!(Value::Integer(1), interpreter.execute("(quote 1)").unwrap());
        assert_eq!(Value::Float(1.1), interpreter.execute("(quote 1.1)").unwrap());
        assert_eq!(Value::Boolean(true), interpreter.execute("(quote #t)").unwrap());
        assert_eq!(Value::Boolean(false), interpreter.execute("(quote #f)").unwrap());

        let expected = interpreter.intern_keyword_value(String::from("test"));
        let result = interpreter.execute("(quote :test)").unwrap();
        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );
        assert_eq!(interpreter.intern_symbol_value("cute-symbol"), interpreter.execute("(quote cute-symbol)").unwrap());

        let expected = interpreter.intern_string_value(String::from("test"));
        let result = interpreter.execute("(quote \"test\")").unwrap();
        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );

        let expected = cons;
        let result = interpreter.execute("(quote (1 2))").unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );

//        Function(func) - lol, how to test this
    }

    // todo: ensure this test is fine
    #[test]
    fn quote_works_correctly_when_used_quote_sign() {
        let mut interpreter = Interpreter::new();

        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            Value::Integer(2),
            nil
        );
        let cons = interpreter.make_cons_value(
            Value::Integer(1),
            cdr
        );

        assert_eq!(Value::Integer(1), interpreter.execute("'1").unwrap());
        assert_eq!(Value::Float(1.1), interpreter.execute("'1.1").unwrap());
        assert_eq!(Value::Boolean(true), interpreter.execute("'#t").unwrap());
        assert_eq!(Value::Boolean(false), interpreter.execute("'#f").unwrap());

        let expected = interpreter.intern_keyword_value(String::from("test"));
        let result = interpreter.execute("':test").unwrap();
        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );
        assert_eq!(interpreter.intern_symbol_value("cute-symbol"), interpreter.execute("'cute-symbol").unwrap());

        let expected = interpreter.intern_string_value(String::from("test"));
        let result = interpreter.execute("'\"test\"").unwrap();
        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );

        let expected = cons;
        let result = interpreter.execute("'(1 2)").unwrap();
        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );

//        Function(func) - lol, how to test this
    }

    // todo: ensure this test is fine
    #[test]
    fn quote_works_correctly_for_quote_invocation() {
        let mut interpreter = Interpreter::new();

        let quote = interpreter.intern_symbol_value("quote");
        let cute_symbol = interpreter.intern_symbol_value("cute-symbol");
        let nil = interpreter.intern_nil_symbol_value();
        let cdr = interpreter.make_cons_value(
            cute_symbol,
            nil
        );
        let expected = interpreter.make_cons_value(
            quote,
            cdr
        );

        let result = interpreter.execute("(quote (quote cute-symbol))").unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);

        let result = interpreter.execute("(quote 'cute-symbol)").unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);

        let result = interpreter.execute("'(quote cute-symbol)").unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);

        let result = interpreter.execute("''cute-symbol").unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);

//        Function(func) - lol, how to test this
    }

    // todo: ensure this test is fine
    #[test]
    fn quote_returns_err_when_improper_count_of_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        assertion::assert_invalid_argument_count_error(&interpreter.execute("(quote)"));
        assertion::assert_invalid_argument_count_error(&interpreter.execute("(quote 1 2)"));
    }
}
