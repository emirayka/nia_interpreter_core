use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::builtin_functions::_lib::infect_builtin_function;
use crate::interpreter::environment::EnvironmentId;

fn lookup(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `lookup' must take exactly one string argument."
        ));
    }

    let mut values = values;

    match values.remove(0) {
        Value::Symbol(symbol) => {
            let nil = interpreter.intern_nil();

            match interpreter.lookup_variable(
                _environment,
                &symbol
            ) {
                Ok(value) => Ok(value.clone()),
                _ => Ok(nil)
            }
        }
        _ => return Err(Error::invalid_argument(
            interpreter,
            "Built-in function `lookup' must take exactly one string argument."
        ))
    }
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_builtin_function(interpreter, "lookup", lookup)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;

    #[test]
    fn returns_associated_value() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        crate::interpreter::stdlib::special_forms::infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(let ((a 1)) (lookup 'a))");

        assert_eq!(Value::Integer(1), result.unwrap());
    }

    #[test]
    fn returns_nil_when_nothing_was_found() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        crate::interpreter::stdlib::special_forms::infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(let ((a 1)) (lookup 'b))");

        assert_eq!(interpreter.intern_nil(), result.unwrap());
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(lookup)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(lookup 1 2)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(lookup 1 2 3)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::raw();

        crate::interpreter::stdlib::special_forms::infect(&mut interpreter).unwrap();
        infect(&mut interpreter).unwrap();

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
