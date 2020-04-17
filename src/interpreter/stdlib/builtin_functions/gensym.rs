use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn gensym(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() > 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `gensym' must take exactly one string argument."
        ).into_result();
    }

    let mut values = values;

    let name = if values.len() == 0 {
        String::from("G")
    } else {
        let string = library::read_as_string(
            interpreter,
            values.remove(0)
        )?;

        string.clone()
    };

    Ok(Value::Symbol(interpreter.gensym(&name)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

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

        let interned = interpreter.intern_symbol_value("test");
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

        let code_vector = vec!(
            "(gensym 1 2)",
            "(gensym 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(gensym 1)",
            "(gensym 1.0)",
            "(gensym #t)",
            "(gensym #f)",
            "(gensym 'symbol)",
            "(gensym :keyword)",
            "(gensym '(s-expression))",
            "(gensym {})",
            "(gensym (function (lambda () 1)))",
            "(gensym (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        )
    }
}
