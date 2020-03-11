use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn _type(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `type' takes one argument exactly ."
        ).into_result();
    }

    let mut values = values;
    let value = values.remove(0);

    let type_string = match value {
        Value::Integer(_) => interpreter.intern_string_value(String::from("integer")),
        Value::Float(_) => interpreter.intern_string_value(String::from("float")),
        Value::Boolean(_) => interpreter.intern_string_value(String::from("boolean")),
        Value::String(_) => interpreter.intern_string_value(String::from("string")),
        Value::Keyword(_) => interpreter.intern_string_value(String::from("keyword")),
        Value::Symbol(_) => interpreter.intern_string_value(String::from("symbol")),
        Value::Cons(_) => interpreter.intern_string_value(String::from("cons")),
        Value::Object(_) => interpreter.intern_string_value(String::from("object")),
        Value::Function(_) => interpreter.intern_string_value(String::from("function")),
    };

    Ok(type_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn evaluates_provided_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(type 1)", "\"integer\""),
            ("(type 1.1)", "\"float\""),
            ("(type #f)", "\"boolean\""),
            ("(type #t)", "\"boolean\""),
            ("(type \"s\")", "\"string\""),
            ("(type 'symbol)", "\"symbol\""),
            ("(type :keyword)", "\"keyword\""),
            ("(type '(1 2))", "\"cons\""),
            ("(type {})", "\"object\""),
            ("(type #())", "\"function\""),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(type)",
            "(type 1 2)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
