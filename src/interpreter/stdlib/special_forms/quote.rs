use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn quote(
    _interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Special form `quote' must be called with exactly one argument.",
        )
        .into();
    }

    let first_argument = values.remove(0);

    Ok(first_argument)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn quote_works_correctly_when_used_quote_special_form() {
        let mut interpreter = Interpreter::new();
        let nil = interpreter.intern_nil_symbol_value();

        let cdr = interpreter.make_cons_value(Value::Integer(2), nil);
        let cons = interpreter.make_cons_value(Value::Integer(1), cdr);

        let specs = vec![
            ("(quote 1)", Value::Integer(1)),
            ("(quote 1.1)", Value::Float(1.1)),
            ("(quote #t)", Value::Boolean(true)),
            ("(quote #f)", Value::Boolean(false)),
            ("(quote :test)", interpreter.intern_keyword_value("test")),
            (
                "(quote cute-symbol)",
                interpreter.intern_symbol_value("cute-symbol"),
            ),
            ("(quote \"test\")", interpreter.intern_string_value("test")),
            ("(quote (1 2))", cons),
        ];

        utils::assert_results_are_correct(&mut interpreter, specs);
    }

    #[test]
    fn quote_works_correctly_when_used_quote_sign() {
        let mut interpreter = Interpreter::new();
        let nil = interpreter.intern_nil_symbol_value();

        let cdr = interpreter.make_cons_value(Value::Integer(2), nil);
        let cons = interpreter.make_cons_value(Value::Integer(1), cdr);

        let specs = vec![
            ("'1", Value::Integer(1)),
            ("'1.1", Value::Float(1.1)),
            ("'#t", Value::Boolean(true)),
            ("'#f", Value::Boolean(false)),
            ("':test", interpreter.intern_keyword_value("test")),
            (
                "'cute-symbol",
                interpreter.intern_symbol_value("cute-symbol"),
            ),
            ("'\"test\"", interpreter.intern_string_value("test")),
            ("'(1 2)", cons),
        ];

        utils::assert_results_are_correct(&mut interpreter, specs);
    }

    #[test]
    fn quote_returns_err_when_improper_count_of_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(quote 1 2)", "(quote)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs,
        );
    }
}
