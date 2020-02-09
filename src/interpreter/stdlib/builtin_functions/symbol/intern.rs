use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::builtin_functions::_lib::infect_builtin_function;
use crate::interpreter::environment::EnvironmentId;

fn intern(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `intern' must take exactly one string argument."
        ));
    }

    let mut values = values;

    match values.remove(0) {
        Value::String(string) => Ok(interpreter.intern(&string)),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "Built-in function `intern' must take exactly one string argument."
        ))
    }
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "intern", intern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;

    #[test]
    fn returns_interned_symbol() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            interpreter.intern("test"),
            interpreter.execute(r#"(intern "test")"#).unwrap()
        );
        assert_eq!(
            interpreter.intern("a"),
            interpreter.execute(r#"(intern "a")"#).unwrap()
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(intern)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(intern 1 2)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(intern 1 2 3)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let incorrect_values = vec!(
            "1",
            "1.0",
            "#t",
            "#f",
            "'symbol",
            ":keyword",
            "'(s-expression)",
            "{}",
            "(function (lambda () 1))",
            "(function (macro () 1))",
        );

        for incorrect_value in incorrect_values {
            let incorrect_code = format!("(intern {})", incorrect_value);

            let result = interpreter.execute(&incorrect_code);

            assertion::assert_invalid_argument_error(&result);
        }
    }
}
