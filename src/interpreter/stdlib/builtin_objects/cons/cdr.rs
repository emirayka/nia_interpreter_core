use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn cdr(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `cons:cdr' must take exactly two arguments.",
        )
        .into();
    }

    let mut values = values;

    let cons_id = library::read_as_cons_id(values.remove(0))?;

    let cdr = interpreter
        .get_cdr(cons_id)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    Ok(cdr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_cdr_of_cons() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(cons:cdr (cons:new 1 1))", Value::Integer(1)),
            ("(cons:cdr (cons:new 1 1.1))", Value::Float(1.1)),
            ("(cons:cdr (cons:new 1 #t))", Value::Boolean(true)),
            ("(cons:cdr (cons:new 1 #f))", Value::Boolean(false)),
            (
                "(cons:cdr (cons:new 1 \"string\"))",
                interpreter.intern_string_value("string"),
            ),
            (
                "(cons:cdr (cons:new 1 'symbol))",
                interpreter.intern_symbol_value("symbol"),
            ),
            (
                "(cons:cdr (cons:new 1 :keyword))",
                interpreter.intern_keyword_value("keyword"),
            ),
            (
                "(cons:cdr (cons:new 1 {}))",
                interpreter.make_object_value(),
            ),
            (
                "(cons:cdr (cons:new 1 (cons:new 1 2)))",
                interpreter
                    .make_cons_value(Value::Integer(1), Value::Integer(2)),
            ),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(cons:cdr)", "(cons:cdr (cons:new 1 2) 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_called_with_a_value_that_is_not_cons(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(cons:cdr 1)",
            "(cons:cdr 1.1)",
            "(cons:cdr #t)",
            "(cons:cdr #f)",
            "(cons:cdr \"string\")",
            "(cons:cdr 'symbol)",
            "(cons:cdr :keyword)",
            "(cons:cdr {})",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
