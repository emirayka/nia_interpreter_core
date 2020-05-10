use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn gensym(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() > 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `gensym' must take exactly one string argument.",
        )
        .into();
    }

    let mut values = values;

    let symbol_name = if values.len() == 0 {
        String::from("G")
    } else {
        let string = library::read_as_string(interpreter, values.remove(0))?;

        string.clone()
    };

    if symbol_name.starts_with("#") {
        return Error::invalid_argument_error("Cannot intern special symbols.")
            .into();
    }

    Ok(Value::Symbol(interpreter.gensym(&symbol_name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_gensym_without_provided_name() {
        let mut interpreter = Interpreter::new();

        let gensym1 = interpreter
            .execute_in_main_environment(r#"(gensym)"#)
            .unwrap();
        let gensym2 = interpreter
            .execute_in_main_environment(r#"(gensym)"#)
            .unwrap();
        let gensym3 = interpreter
            .execute_in_main_environment(r#"(gensym)"#)
            .unwrap();

        nia_assert_nequal(gensym1, gensym2);
        nia_assert_nequal(gensym1, gensym3);

        nia_assert_nequal(gensym2, gensym3);
    }

    #[test]
    fn returns_gensym_with_target_name() {
        let mut interpreter = Interpreter::new();

        let interned = interpreter.intern_symbol_value("test");
        let gensym1 = interpreter
            .execute_in_main_environment(r#"(gensym "test")"#)
            .unwrap();
        let gensym2 = interpreter
            .execute_in_main_environment(r#"(gensym "test")"#)
            .unwrap();
        let gensym3 = interpreter
            .execute_in_main_environment(r#"(gensym "test")"#)
            .unwrap();

        nia_assert_nequal(interned, gensym1);
        nia_assert_nequal(interned, gensym2);
        nia_assert_nequal(interned, gensym3);

        nia_assert_nequal(gensym1, gensym2);
        nia_assert_nequal(gensym1, gensym3);

        nia_assert_nequal(gensym2, gensym3);
    }

    #[test]
    fn returns_invalid_argument_error_when_attempts_to_gensym_special_symbols()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(gensym \"#opt\")",
            "(gensym \"#rest\")",
            "(gensym \"#keys\")",
            "(gensym \"#another-special-symbol\")",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(gensym 1 2)", "(gensym 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
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
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }
}
