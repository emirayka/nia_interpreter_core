use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::{Error, ErrorKind};
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

fn make_while_environment(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
) -> Result<EnvironmentId, Error> {
    let while_env = interpreter.make_environment(
        environment_id
    )?;

    let break_symbol_id = interpreter.intern("break");
    let break_function_id = interpreter.get_internal_function("break")?;

    let continue_symbol_id = interpreter.intern("continue");
    let continue_function_id = interpreter.get_internal_function("continue")?;

    interpreter.define_function(
        while_env,
        break_symbol_id,
        Value::Function(break_function_id)
    )?;

    interpreter.define_function(
        while_env,
        continue_symbol_id,
        Value::Function(continue_function_id)
    )?;

    Ok(while_env)
}

pub fn _while(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 1 {
        return Error::invalid_argument_count_error(
            "Special form `while' takes one argument at least."
        ).into();
    }

    let while_environment_id = make_while_environment(
        interpreter,
        environment_id
    )?;

    let mut values = values;
    let condition = values.remove(0);
    let code = values;

    let mut condition_result = interpreter.evaluate_value(
        environment_id,
        condition
    )?;

    loop {
        match condition_result {
            Value::Boolean(true) => {},
            Value::Boolean(false) => {
                break;
            },
            _ => return Error::generic_execution_error(
                "Special form while expects booleans only in condition."
            ).into()
        }

        match library::execute_forms(
            interpreter,
            while_environment_id,
            &code
        ) {
            Ok(_) => {},
            Err(error) => {
                match error.get_error_kind() {
                    ErrorKind::Break => {
                        break;
                    },
                    ErrorKind::Continue => {},
                    _ => return Err(error)
                }
            }
        };

        condition_result = interpreter.evaluate_value(
            environment_id,
            condition
        )?;
    }

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn loops() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(defv a 1) (while (< a 10) (set! a (inc a))) a", "10"),
            ("(let ((b 1)) (while (< b 10) (set! b (inc b))) b)", "10"),
            ("(let ((c 1)) (let ((c 1)) (while (< c 10) (set! c (inc c)))) c)", "1"),
            ("(defv d 1) (while (< d 10) (set! d (inc d)) (break)) d", "2"),
            ("(defv e 1) (defv f 1) (while (< e 10) (set! e (inc e)) (continue) (set! f (inc f))) f", "1"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_generic_execution_error_when_condition_evaluated_to_not_a_boolean() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(while 1 1)",
        );

        assertion::assert_results_are_generic_execution_errors(
            &mut interpreter,
            code_vector
        )
    }

    #[test]
    fn returns_invalid_argument_count_when_was_called_with_invalid_argument_count() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(while)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}
