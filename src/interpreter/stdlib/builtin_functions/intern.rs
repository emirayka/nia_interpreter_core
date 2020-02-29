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

    #[test]
    fn returns_interned_symbol() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"(intern "test")"#, interpreter.intern_symbol_value("test")),
            (r#"(intern "a")"#, interpreter.intern_symbol_value("a"))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            "(intern)",
            "(intern 1 2)",
            "(intern 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(intern 1)",
            "(intern 1.0)",
            "(intern #t)",
            "(intern #f)",
            "(intern 'symbol)",
            "(intern :keyword)",
            "(intern '(s-expression))",
            "(intern {})",
            "(intern (function (lambda () 1)))",
            "(intern (function (macro () 1)))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
