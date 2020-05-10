use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn _type(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `type' takes one argument exactly .",
        )
        .into();
    }

    let mut values = values;
    let value = values.remove(0);

    let type_string = match value {
        Value::Integer(_) => interpreter.intern_string_value("integer"),
        Value::Float(_) => interpreter.intern_string_value("float"),
        Value::Boolean(_) => interpreter.intern_string_value("boolean"),
        Value::String(_) => interpreter.intern_string_value("string"),
        Value::Keyword(_) => interpreter.intern_string_value("keyword"),
        Value::Symbol(_) => interpreter.intern_string_value("symbol"),
        Value::Cons(_) => interpreter.intern_string_value("cons"),
        Value::Object(_) => interpreter.intern_string_value("object"),
        Value::Function(_) => interpreter.intern_string_value("function"),
    };

    Ok(type_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn evaluates_provided_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
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
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(type)", "(type 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
