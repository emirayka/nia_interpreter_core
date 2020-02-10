use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn gensym(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() > 1 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `gensym' must take exactly one string argument."
        ));
    }

    let mut values = values;

    let name = if values.len() == 0 {
        String::from("G")
    } else {
        match values.remove(0) {
            Value::String(string) => string,
            _ => return Err(Error::invalid_argument(
                interpreter,
                "Built-in function `gensym' must take exactly one string argument."
            ))
        }
    };

    Ok(Value::Symbol(interpreter.gensym(&name)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_gensym_without_provided_name() {
        let mut interpreter = Interpreter::new();

        let gensym1 = interpreter.execute(r#"(gensym)"#).unwrap();
        let gensym2 = interpreter.execute(r#"(gensym)"#).unwrap();
        let gensym3 = interpreter.execute(r#"(gensym)"#).unwrap();

        assert_ne!(gensym1, gensym2);
        assert_ne!(gensym1, gensym3);

        assert_ne!(gensym2, gensym3);
    }

    #[test]
    fn returns_gensym_with_target_name() {
        let mut interpreter = Interpreter::new();

        let interned = interpreter.intern("test");
        let gensym1 = interpreter.execute(r#"(gensym "test")"#).unwrap();
        let gensym2 = interpreter.execute(r#"(gensym "test")"#).unwrap();
        let gensym3 = interpreter.execute(r#"(gensym "test")"#).unwrap();

        assert_ne!(interned, gensym1);
        assert_ne!(interned, gensym2);
        assert_ne!(interned, gensym3);

        assert_ne!(gensym1, gensym2);
        assert_ne!(gensym1, gensym3);

        assert_ne!(gensym2, gensym3);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(gensym 1 2)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(gensym 1 2 3)");
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
            let incorrect_code = format!("(gensym {})", incorrect_value);

            let result = interpreter.execute(&incorrect_code);

            assertion::assert_invalid_argument_error(&result);
        }
    }
}
