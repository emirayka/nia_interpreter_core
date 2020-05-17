use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn car(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `cons:car' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let cons_id = library::read_as_cons_id(values.remove(0))?;

    let car = interpreter
        .get_car(cons_id)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    Ok(car)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_car_of_cons() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(cons:car (cons:new 1 1))", Value::Integer(1)),
            ("(cons:car (cons:new 1.1 1))", Value::Float(1.1)),
            ("(cons:car (cons:new #t 1))", Value::Boolean(true)),
            ("(cons:car (cons:new #f 1))", Value::Boolean(false)),
            (
                "(cons:car (cons:new \"string\" 1))",
                interpreter.intern_string_value("string"),
            ),
            (
                "(cons:car (cons:new 'symbol 1))",
                interpreter.intern_symbol_value("symbol"),
            ),
            (
                "(cons:car (cons:new :keyword 1))",
                interpreter.intern_keyword_value("keyword"),
            ),
            (
                "(cons:car (cons:new {} 1))",
                interpreter.make_object_value(),
            ),
            (
                "(cons:car (cons:new (cons:new 1 2) 1))",
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

        let code_vector = vec!["(cons:car)", "(cons:car (cons:new 1 2) 3)"];

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
            "(cons:car 1)",
            "(cons:car 1.1)",
            "(cons:car #t)",
            "(cons:car #f)",
            "(cons:car \"string\")",
            "(cons:car 'symbol)",
            "(cons:car :keyword)",
            "(cons:car {})",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
