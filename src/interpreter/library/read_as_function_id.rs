use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::FunctionId;
use crate::interpreter::value::Value;

pub fn read_as_function_id(value: Value) -> Result<FunctionId, Error> {
    let function_id = match value {
        Value::Function(function_id) => function_id,
        _ => return Error::invalid_argument_error("Expected a function.").into(),
    };

    Ok(function_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::interpreter::library;
    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_function() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(function (lambda () 3))",
            "(flookup 'flookup)",
            "(function (macro () 2))",
            "(flookup 'cond)",
        ];

        for code in code_vector {
            let result = interpreter.execute_in_main_environment(code).unwrap();

            library::read_as_function_id(result).unwrap();
        }
    }

    #[test]
    fn returns_invalid_argument_when_not_a_function_value_were_passed() {
        let mut interpreter = Interpreter::new();

        let not_string_values = vec![
            Value::Integer(1),
            Value::Float(1.1),
            Value::Boolean(true),
            Value::Boolean(false),
            interpreter.intern_string_value("test"),
            interpreter.intern_symbol_value("test"),
            interpreter.intern_keyword_value("test"),
            interpreter.make_cons_value(Value::Integer(1), Value::Integer(2)),
            interpreter.make_object_value(),
        ];

        for not_string_value in not_string_values {
            let result = library::read_as_function_id(not_string_value);
            assertion::assert_invalid_argument_error(&result);
        }
    }
}
