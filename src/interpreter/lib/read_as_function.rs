use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::function::function_arena::FunctionId;
use crate::interpreter::error::Error;

use crate::interpreter::lib;
use crate::interpreter::function::Function;

pub fn read_as_function(
    interpreter: &Interpreter,
    value: Value
) -> Result<&Function, Error> {
    let function_id = lib::read_as_function_id(
        interpreter,
        value
    )?;

    let function = match interpreter.get_function(function_id) {
        Ok(function) => function,
        Err(error) => return interpreter.make_generic_execution_error_caused(
            "",
            error
        ).into_result()
    };

    Ok(function)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_correct_function() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(function (lambda () 3))",
            "(flookup 'flookup)",
            "(function (macro () 2))",
            "(flookup 'cond)",
        );

        for code in code_vector {
            let result = interpreter.execute(code).unwrap();
            let function = lib::read_as_function(
                &mut interpreter,
                result
            ).unwrap();
        }
    }

    #[test]
    fn returns_invalid_argument_when_not_a_function_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_string_values = vec!(
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.make_string_value(String::from("test")),
            interpreter.intern_symbol_value("test"),
            interpreter.make_keyword_value(String::from("test")),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
        );

        for not_string_value in not_string_values {
            let result = lib::read_as_function(
                &mut interpreter,
                not_string_value
            );
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
