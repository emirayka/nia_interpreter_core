use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::library;

pub fn set(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `set!' must be used with exactly two arguments"
        ).into_result();
    }

    let mut values = values;

    let variable_symbol_id = match values.remove(0) {
        Value::Symbol(symbol) => symbol,
        _ => return interpreter.make_invalid_argument_error(
            "The first argument of special form `set!' must be a symbol."
        ).into_result()
    };

    library::check_if_symbol_assignable(interpreter, variable_symbol_id)?;

    let value = values.remove(0);

//            &format!("Cannot execute value: \"{}\""), // todo: add here value description
    let value = interpreter.execute_value(environment, value)
        .map_err(|err| return interpreter.make_generic_execution_error_caused(
            "Cannot execute value: \"{}\"",
            err
        ))?;

    let target_env = interpreter.lookup_environment_by_variable(
        environment,
        variable_symbol_id
    ).map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    match target_env {
        Some(target_env) => {
            match interpreter.set_variable(target_env, variable_symbol_id, value) {
                Ok(()) => Ok(value),
                Err(error) => {
                    let message = &format!(
                        "Cannot set variable `{}'",
                        interpreter.get_symbol_name(variable_symbol_id)?
                    );

                    interpreter.make_generic_execution_error_caused(
                        message,
                        error
                    ).into_result()
                }
            }
        },
        None => {
            let message = &format!(
                "Cannot find variable `{}'",
                interpreter.get_symbol_name(variable_symbol_id)?
            );

            interpreter.make_generic_execution_error(
                message
            ).into_result()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::{for_constants, for_special_symbols};

    #[test]
    fn returns_value_that_was_set_to_variable() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("(let ((a 1)) (set! a 2))", Value::Integer(2)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            specs
        )
    }

    #[test]
    fn sets_to_current_environment_when_variable_is_defined_here() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable a 0)").unwrap();

        let specs = vec!(
            ("(let ((a 1)) a)", Value::Integer(1)),
            ("(let ((a 1)) (set! a 2) a)", Value::Integer(2)),
            ("a", Value::Integer(0))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn sets_to_parent_environment_when_variable_is_defined_here() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable b 0)").unwrap();
        interpreter.execute("(let ((a 1)) (set! b 2))").unwrap();

        let specs = vec!(
            ("b", Value::Integer(2)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_error_when_attempts_to_define_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(set! {} 2)", constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(set! {} 2)", special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_err_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(set!)",
            "(set! a b c)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_err_when_incorrect_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(set! 1 1)",
            "(set! 1.1 1)",
            "(set! #t 1)",
            "(set! #f 1)",
            "(set! :keyword 1)",
            "(set! \"string\" 1)",
            "(set! {} 1)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs
        );
    }
}
