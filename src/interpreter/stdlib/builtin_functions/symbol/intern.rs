use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn intern(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `intern' must take exactly one string argument."
        ).into_result();
    }

    let mut values = values;

    let string_id = match values.remove(0) {
        Value::String(string_id) => string_id,
        _ => return interpreter.make_invalid_argument_error(
            "Built-in function `intern' must take exactly one string argument."
        ).into_result()
    };

    let symbol_name = match interpreter.get_string(string_id) {
        Ok(string) => String::from(string.get_string()), // todo: fix, looks shitty
        Err(error) => return interpreter.make_generic_execution_error_caused(
            "",
            error
        ).into_result()
    };

    Ok(interpreter.intern_symbol_value(&symbol_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    // todo: ensure this test is fine
    #[test]
    fn returns_interned_symbol() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            interpreter.intern_symbol_value("test"),
            interpreter.execute(r#"(intern "test")"#).unwrap()
        );
        assert_eq!(
            interpreter.intern_symbol_value("a"),
            interpreter.execute(r#"(intern "a")"#).unwrap()
        );
    }

    // todo: ensure this test is fine
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

    // todo: ensure this test is fine
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
