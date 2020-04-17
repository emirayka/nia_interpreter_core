use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn cdr(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `cdr' must take exactly two arguments."
        ).into_result();
    }

    let mut values = values;

    let cons_id = library::read_as_cons_id(
        interpreter,
        values.remove(0)
    )?;

    let cdr = interpreter.get_cdr(cons_id)
        .map_err(|err| Error::generic_execution_error_caused(
            "",
            err
        ))?;

    Ok(cdr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_cdr_of_cons() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(cdr (cons 1 1))", Value::Integer(1)),
            ("(cdr (cons 1 1.1))", Value::Float(1.1)),
            ("(cdr (cons 1 #t))", Value::Boolean(true)),
            ("(cdr (cons 1 #f))", Value::Boolean(false)),
            ("(cdr (cons 1 \"string\"))", interpreter.intern_string_value(String::from("string"))),
            ("(cdr (cons 1 'symbol))", interpreter.intern_symbol_value("symbol")),
            ("(cdr (cons 1 :keyword))", interpreter.intern_keyword_value(String::from("keyword"))),
            ("(cdr (cons 1 {}))", interpreter.make_object_value()),
            ("(cdr (cons 1 (cons 1 2)))", interpreter.make_cons_value(Value::Integer(1), Value::Integer(2))),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(cdr)",
            "(cdr (cons 1 2) 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_called_with_a_value_that_is_not_cons() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(cdr 1)",
            "(cdr 1.1)",
            "(cdr #t)",
            "(cdr #f)",
            "(cdr \"string\")",
            "(cdr 'symbol)",
            "(cdr :keyword)",
            "(cdr {})",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
