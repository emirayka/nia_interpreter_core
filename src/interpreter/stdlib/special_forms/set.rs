use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn set(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `set!' must be used with exactly two arguments"
        );
    }

    let mut values = values;

    let variable_name = match values.remove(0) {
        Value::Symbol(symbol) => symbol,
        _ => return interpreter.make_invalid_argument_error(
            "The first argument of special form `set!' must be a symbol."
        )
    };

    let value = values.remove(0);

    let value = match interpreter.execute_value(environment, &value) {
        Ok(value) => value,
        Err(error) => return interpreter.make_generic_execution_error_caused(
//            &format!("Cannot execute value: \"{}\""), // todo: add here value description
            "Cannot execute value: \"{}\"",
            error
        )
    };

    let target_env = interpreter.lookup_environment_by_variable(environment, &variable_name);

    match target_env {
        Some(target_env) => {
            match interpreter.set_variable(target_env, &variable_name, value.clone()) {
                Ok(()) => Ok(value),
                Err(error) => interpreter.make_generic_execution_error_caused(
                        &format!("Cannot set variable `{}'", variable_name.get_name()),
                    error
                )
            }
        },
        None => {
                interpreter.make_generic_execution_error(
                &format!("Cannot find variable `{}'", variable_name.get_name())
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_value_that_was_set_to_variable() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((a 1)) (set! a 2))");

        assert_eq!(Value::Integer(2), result.unwrap());
    }

    #[test]
    fn sets_to_current_environment_when_variable_is_defined_here() {
        let mut interpreter = Interpreter::new();

        let variable_name = interpreter.intern_symbol("a");

        interpreter.define_variable(
            interpreter.get_root_environment(),
            &variable_name,
            Value::Integer(0)
        ).unwrap();

        let result1 = interpreter.execute("(let ((a 1)) a)").unwrap();
        let result2 = interpreter.execute("(let ((a 1)) (set! a 2) a)").unwrap();

        assert_eq!(
            Value::Integer(0),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                &variable_name
            ).unwrap());
        assert_eq!(Value::Integer(1), result1);
        assert_eq!(Value::Integer(2), result2);
    }

    #[test]
    fn sets_to_parent_environment_when_variable_is_defined_here() {
        let mut interpreter = Interpreter::new();

        let variable_name_b = interpreter.intern_symbol("b");

        interpreter.execute("(define-variable b 0)").unwrap();
        interpreter.execute("(let ((a 1)) (set! b 2))").unwrap();

        assert_eq!(
            Value::Integer(2),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                &variable_name_b
            ).unwrap()
        );
    }

    #[test]
    fn returns_err_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(set!)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(set! a b c)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_err_when_incorrect_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let incorrect_variables = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            ":keyword",
            "\"string\"",
        );

        for incorrect_variable in incorrect_variables {
            let result = interpreter.execute(&format!("(set! {} 1)", incorrect_variable));
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
