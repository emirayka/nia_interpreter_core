use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn quote(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() != 1 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Special form `quote' must be called with exactly one argument."
        ));
    }

    let first_argument = values.remove(0);

    Ok(first_argument)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::cons::cons::Cons;
    use crate::interpreter::lib::assertion;

    #[test]
    fn quote_works_correctly_when_used_quote_special_form() {
        let mut interpreter = Interpreter::new();

        let cons = Value::Cons(Cons::new(
            Value::Integer(1),
            Value::Cons(Cons::new(
                Value::Integer(2),
                interpreter.intern_nil()
            ))
        ));

        assert_eq!(Value::Integer(1), interpreter.execute("(quote 1)").unwrap());
        assert_eq!(Value::Float(1.1), interpreter.execute("(quote 1.1)").unwrap());
        assert_eq!(Value::Boolean(true), interpreter.execute("(quote #t)").unwrap());
        assert_eq!(Value::Boolean(false), interpreter.execute("(quote #f)").unwrap());
        assert_eq!(Value::Keyword(String::from("test")), interpreter.execute("(quote :test)").unwrap());
        assert_eq!(interpreter.intern("cute-symbol"), interpreter.execute("(quote cute-symbol)").unwrap());
        assert_eq!(Value::String(String::from("test")), interpreter.execute("(quote \"test\")").unwrap());
        assert_eq!(cons, interpreter.execute("(quote (1 2))").unwrap());

//        Function(func) - lol, how to test this
    }

    #[test]
    fn quote_works_correctly_when_used_quote_sign() {
        let mut interpreter = Interpreter::new();

        let cons = Value::Cons(Cons::new(
            Value::Integer(1),
            Value::Cons(Cons::new(
                Value::Integer(2),
                interpreter.intern_nil()
            ))
        ));

        assert_eq!(Value::Integer(1), interpreter.execute("'1").unwrap());
        assert_eq!(Value::Float(1.1), interpreter.execute("'1.1").unwrap());
        assert_eq!(Value::Boolean(true), interpreter.execute("'#t").unwrap());
        assert_eq!(Value::Boolean(false), interpreter.execute("'#f").unwrap());
        assert_eq!(Value::Keyword(String::from("test")), interpreter.execute("':test").unwrap());
        assert_eq!(interpreter.intern("cute-symbol"), interpreter.execute("'cute-symbol").unwrap());
        assert_eq!(Value::String(String::from("test")), interpreter.execute("'\"test\"").unwrap());
        assert_eq!(cons, interpreter.execute("'(1 2)").unwrap());

//        Function(func) - lol, how to test this
    }

    #[test]
    fn quote_works_correctly_for_quote_invocation() {
        let mut interpreter = Interpreter::new();

        let cons = Value::Cons(Cons::new(
            interpreter.intern("quote"),
            Value::Cons(Cons::new(
                interpreter.intern("cute-symbol"),
                interpreter.intern_nil()
            ))
        ));

        assert_eq!(cons, interpreter.execute("(quote (quote cute-symbol))").unwrap());
        assert_eq!(cons, interpreter.execute("(quote 'cute-symbol)").unwrap());
        assert_eq!(cons, interpreter.execute("'(quote cute-symbol)").unwrap());
        assert_eq!(cons, interpreter.execute("''cute-symbol").unwrap());

//        Function(func) - lol, how to test this
    }

    #[test]
    fn quote_returns_err_when_improper_count_of_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        assertion::assert_invalid_argument_count_error(&interpreter.execute("(quote)"));
        assertion::assert_invalid_argument_count_error(&interpreter.execute("(quote 1 2)"));
    }
}
