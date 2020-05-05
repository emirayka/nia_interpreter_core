use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn set(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Special form `set!' must be used with exactly two arguments",
        )
        .into();
    }

    let mut values = values;

    let variable_symbol_id =
        match values.remove(0) {
            Value::Symbol(symbol) => symbol,
            _ => return Error::invalid_argument_error(
                "The first argument of special form `set!' must be a symbol.",
            )
            .into(),
        };

    library::check_symbol_is_assignable(interpreter, variable_symbol_id)?;

    let value = values.remove(0);

    //            &format!("Cannot execute value: \"{}\""), // todo: add here value description
    let value =
        interpreter
            .execute_value(environment, value)
            .map_err(|err| {
                return Error::generic_execution_error_caused(
                    "Cannot execute value: \"{}\"",
                    err,
                );
            })?;

    let target_env = interpreter
        .lookup_environment_by_variable(environment, variable_symbol_id)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    match target_env {
        Some(target_env) => match interpreter.set_variable(
            target_env,
            variable_symbol_id,
            value,
        ) {
            Ok(()) => Ok(value),
            Err(error) => {
                let message = &format!(
                    "Cannot set variable `{}'",
                    interpreter.get_symbol_name(variable_symbol_id)?
                );

                Error::generic_execution_error_caused(message, error).into()
            },
        },
        None => {
            let message = &format!(
                "Cannot find variable `{}'",
                interpreter.get_symbol_name(variable_symbol_id)?
            );

            Error::generic_execution_error(message).into()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_value_that_was_set_to_variable() {
        let mut interpreter = Interpreter::new();

        let specs = vec![("(let ((a 1)) (set! a 2))", Value::Integer(2))];

        assertion::assert_results_are_correct(&mut interpreter, specs)
    }

    #[test]
    fn sets_to_current_environment_when_variable_is_defined_here() {
        let mut interpreter = Interpreter::new();

        interpreter
            .execute_in_main_environment("(define-variable a 0)")
            .unwrap();

        let specs = vec![
            ("(let ((a 1)) a)", Value::Integer(1)),
            ("(let ((a 1)) (set! a 2) a)", Value::Integer(2)),
            ("a", Value::Integer(0)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, specs);
    }

    #[test]
    fn sets_to_parent_environment_when_variable_is_defined_here() {
        let mut interpreter = Interpreter::new();

        interpreter
            .execute_in_main_environment("(define-variable b 0)")
            .unwrap();
        interpreter
            .execute_in_main_environment("(let ((a 1)) (set! b 2))")
            .unwrap();

        let specs = vec![("b", Value::Integer(2))];

        assertion::assert_results_are_correct(&mut interpreter, specs);
    }

    #[test]
    fn returns_error_when_attempts_to_define_constant_or_special_symbol() {
        let mut interpreter = Interpreter::new();

        let mut specs = vec![
            // todo: when new constants will be, add them here
            "(set! nil 2)",
            // todo: when new special symbols will be, add them here
            "(set! #opt 2)",
            "(set! #rest 2)",
            "(set! #keys 2)",
            // todo: remainder, when new special variable will be introduced, add them here
            "(set! this 2)",
            "(set! super 2)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(set!)", "(set! a b c)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_incorrect_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(set! 1 1)",
            "(set! 1.1 1)",
            "(set! #t 1)",
            "(set! #f 1)",
            "(set! :keyword 1)",
            "(set! \"string\" 1)",
            "(set! {} 1)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }
}
