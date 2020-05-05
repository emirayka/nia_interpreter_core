use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn fset(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `fset!' must be used with exactly two arguments",
        )
        .into();
    }

    let mut values = values;

    let function_symbol_id = match values.remove(0) {
        Value::Symbol(symbol) => symbol,
        _ => return Error::invalid_argument_error(
            "The first argument of built-in function `fset!' must be a symbol.",
        )
        .into(),
    };

    library::check_symbol_is_assignable(interpreter, function_symbol_id)?;

    let value = values.remove(0);

    //            &format!("Cannot execute value: \"{}\""), // todo: add here value description
    let value =
        interpreter
            .execute_value(environment, value)
            .map_err(|err| {
                Error::generic_execution_error_caused(
                    "Cannot execute value: \"{}\"",
                    err,
                )
            })?;

    let target_env = interpreter
        .lookup_environment_by_function(environment, function_symbol_id)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    match target_env {
        Some(target_env) => match interpreter.set_function(
            target_env,
            function_symbol_id,
            value,
        ) {
            Ok(()) => Ok(value),
            Err(error) => {
                let message = &format!(
                    "Cannot set function `{}'",
                    interpreter.get_symbol_name(function_symbol_id)?
                );

                Error::generic_execution_error_caused(message, error).into()
            },
        },
        None => {
            let message = &format!(
                "Cannot find function `{}'",
                interpreter.get_symbol_name(function_symbol_id)?
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

    use crate::interpreter::value::Function;
    use crate::interpreter::value::FunctionArguments;
    use crate::interpreter::value::InterpretedFunction;
    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_value_that_was_set_to_function() {
        let mut interpreter = Interpreter::new();
        let arguments = FunctionArguments::new();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![Value::Integer(2)],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute_in_main_environment(
            "(define-function a (function (lambda () 1))) (fset! a (function (lambda () 2)))",
        );

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result.unwrap(),
        );
    }

    #[test]
    fn sets_to_current_environment_when_function_is_defined_here() {
        let mut interpreter = Interpreter::new();

        interpreter
            .execute_in_main_environment(
                "(define-function a (function (lambda () 0)))",
            )
            .unwrap();

        let specs = vec![
            ("0", "(a)"),
            ("1", "(flet ((a () 1)) (a))"),
            (
                "2",
                "(flet ((a () 1)) (fset! a (function (lambda () 2))) (a))",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn sets_to_parent_environment_when_function_is_defined_here() {
        let mut interpreter = Interpreter::new();

        interpreter
            .execute_in_main_environment(
                "(define-function b (function (lambda () 0)))",
            )
            .unwrap();
        interpreter
            .execute_in_main_environment(
                "(flet ((a () 1)) (fset! b (function (lambda () 2))))",
            )
            .unwrap();

        let result = interpreter.execute_in_main_environment("(b)").unwrap();

        nia_assert_equal(Value::Integer(2), result);
    }

    #[test]
    fn returns_error_when_attempts_to_define_constant_or_special_symbol() {
        let mut interpreter = Interpreter::new();

        let mut specs = vec![
            // todo: when new constants will be, add them here
            "(fset! nil 2)",
            // todo: when new special symbols will be, add them here
            "(fset! #opt 2)",
            "(fset! #rest 2)",
            "(fset! #keys 2)",
            // todo: remainder, when new special variable will be introduced, add them here
            "(fset! this 2)",
            "(fset! super 2)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(fset!)", "(fset! a b c)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_incorrect_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(fset! 1 1)",
            "(fset! 1.1 1)",
            "(fset! #t 1)",
            "(fset! #f 1)",
            "(fset! :keyword 1)",
            "(fset! \"string\" 1)",
            "(fset! '() 1)",
            "(fset! {} 1)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }
}
