use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn cons(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `cons' must take exactly two arguments.",
        )
        .into();
    }

    let mut values = values;

    Ok(interpreter.make_cons_value(values.remove(0), values.remove(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_a_cons_cell() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                interpreter
                    .make_cons_value(Value::Integer(1), Value::Integer(2)),
                "(cons 1 2)",
            ),
            (
                interpreter
                    .make_cons_value(Value::Float(1.1), Value::Float(2.2)),
                "(cons 1.1 2.2)",
            ),
        ];

        for spec in specs {
            let expected = spec.0;
            let result =
                interpreter.execute_in_main_environment(spec.1).unwrap();

            utils::assert_deep_equal(&mut interpreter, expected, result)
        }
    }

    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(cons)", "(cons 1)", "(cons 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
