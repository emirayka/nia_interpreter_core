use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::stdlib::_lib;

pub fn fset(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `fset!' must be used with exactly two arguments"
        ).into_result();
    }

    let mut values = values;

    let function_symbol_id = match values.remove(0) {
        Value::Symbol(symbol) => symbol,
        _ => return interpreter.make_invalid_argument_error(
            "The first argument of built-in function `fset!' must be a symbol."
        ).into_result()
    };

    _lib::check_if_symbol_assignable(interpreter, function_symbol_id)?;

    let value = values.remove(0);

//            &format!("Cannot execute value: \"{}\""), // todo: add here value description
    let value = interpreter.execute_value(environment, value)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "Cannot execute value: \"{}\"",
            err
        ))?;

    let target_env = interpreter.lookup_environment_by_function(
        environment,
        function_symbol_id
    ).map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    match target_env {
        Some(target_env) => {
            match interpreter.set_function(target_env, function_symbol_id, value) {
                Ok(()) => Ok(value),
                Err(error) => {
                    let message = &format!(
                        "Cannot set function `{}'",
                        interpreter.get_symbol_name(function_symbol_id)?
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
                "Cannot find function `{}'",
                interpreter.get_symbol_name(function_symbol_id)?
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
    use crate::interpreter::lib::assertion;
    use crate::interpreter::function::Function;
    use crate::interpreter::function::interpreted_function::InterpretedFunction;
    use crate::interpreter::lib::testing_helpers::{for_special_symbols, for_constants};

    #[test]
    fn returns_value_that_was_set_to_function() {
        let mut interpreter = Interpreter::new();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            vec!(),
            vec!(
                Value::Integer(2)
            )
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute(
            "(define-function a (function (lambda () 1))) (fset! a (function (lambda () 2)))"
        );

        assertion::assert_deep_equal(&mut interpreter, expected, result.unwrap());
    }

    #[test]
    fn sets_to_current_environment_when_function_is_defined_here() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function a (function (lambda () 0)))").unwrap();

        let result0 = interpreter.execute("(a)").unwrap();
        let result1 = interpreter.execute("(flet ((a () 1)) (a))").unwrap();
        let result2 = interpreter.execute("(flet ((a () 1)) (fset! a (function (lambda () 2))) (a))").unwrap();

        assert_eq!(Value::Integer(0), result0);
        assert_eq!(Value::Integer(1), result1);
        assert_eq!(Value::Integer(2), result2);
    }

    #[test]
    fn sets_to_parent_environment_when_function_is_defined_here() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function b (function (lambda () 0)))").unwrap();
        interpreter.execute("(flet ((a () 1)) (fset! b (function (lambda () 2))))").unwrap();

        let result = interpreter.execute("(b)").unwrap();

        assert_eq!(Value::Integer(2), result);
    }

    #[test]
    fn returns_error_when_attempts_to_define_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(fset! {} 2)", constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(fset! {} 2)", special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_err_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(fset!)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(fset! a b c)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_err_when_incorrect_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let incorrect_functions = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            ":keyword",
            "\"string\"",
        );

        for incorrect_function in incorrect_functions {
            let result = interpreter.execute(&format!("(fset! {} 1)", incorrect_function));
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
