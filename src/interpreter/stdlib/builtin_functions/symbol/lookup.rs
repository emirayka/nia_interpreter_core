use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn lookup(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `lookup' must take exactly one string argument."
        );
    }

    let mut values = values;

    match values.remove(0) {
        Value::Symbol(symbol_id) => {
            let nil = interpreter.intern_nil_symbol_value();

            match interpreter.lookup_variable(
                _environment,
                symbol_id
            ) {
                Ok(value) => Ok(value.clone()),
                _ => Ok(nil)
            }
        }
        _ => return interpreter.make_invalid_argument_error(
            "Built-in function `lookup' must take exactly one string argument."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_associated_value() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((a 1)) (lookup 'a))");

        assert_eq!(Value::Integer(1), result.unwrap());
    }

    #[test]
    fn returns_nil_when_nothing_was_found() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((a 1)) (lookup 'b))");

        assert_eq!(interpreter.intern_nil_symbol_value(), result.unwrap());
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(lookup)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(lookup 1 2)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(lookup 1 2 3)");
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
            "\"string\"",
            ":keyword",
            "'(s-expression)",
            "{}",
            "(function (lambda () 1))",
            "(function (macro () 1))",
        );

        for incorrect_value in incorrect_values {
            let incorrect_code = format!("(lookup {})", incorrect_value);

            let result = interpreter.execute(&incorrect_code);

            assertion::assert_invalid_argument_error(&result);
        }
    }
}
