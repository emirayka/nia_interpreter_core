use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

pub fn fset(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `fset!' must be used with exactly two arguments"
        ));
    }

    let mut values = values;

    let function_name = match values.remove(0) {
        Value::Symbol(symbol) => symbol,
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first argument of built-in function `fset!' must be a symbol."
        ))
    };

    let value = values.remove(0);

    let value = match interpreter.execute_value(environment, &value) {
        Ok(value) => value,
        Err(error) => return Err(Error::generic_execution_error_caused(
            interpreter,
//            &format!("Cannot execute value: \"{}\""), // todo: add here value description
            "Cannot execute value: \"{}\"",
            error
        ))
    };

    let target_env = interpreter.lookup_environment_by_function(environment, &function_name);

    match target_env {
        Some(target_env) => {
            match interpreter.set_function(target_env, &function_name, value.clone()) {
                Ok(()) => Ok(value),
                Err(error) => Err(Error::generic_execution_error_caused(
                    interpreter,
                    &format!("Cannot set function `{}'", function_name.get_name()),
                    error))
            }
        },
        None => {
            Err(Error::generic_execution_error(
                interpreter,
                &format!("Cannot find function `{}'", function_name.get_name())
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::function::Function;
    use crate::interpreter::function::interpreted_function::InterpretedFunction;

    #[test]
    fn returns_value_that_was_set_to_function() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Function(Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            vec!(),
            vec!(
                Value::Integer(2)
            )
        )));

        let result = interpreter.execute(
            "(define-function a (function (lambda () 1))) (fset! a (function (lambda () 2)))"
        );

        assert_eq!(expected, result.unwrap());
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

        let function_name_b = interpreter.intern_symbol("b");

        interpreter.execute("(define-function b (function (lambda () 0)))").unwrap();
        interpreter.execute("(flet ((a () 1)) (fset! b (function (lambda () 2))))").unwrap();

        let result = interpreter.execute("(b)").unwrap();

        assert_eq!(Value::Integer(2), result);
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