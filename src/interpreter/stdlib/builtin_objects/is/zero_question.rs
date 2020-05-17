use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn zero_question(
    _interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `is:zero?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Integer(val) => val == 0,
        Value::Float(val) => val == 0.0,
        _ => false,
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_true_when_zero_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:zero? -1)", Value::Boolean(false)),
            ("(is:zero? 0)", Value::Boolean(true)),
            ("(is:zero? 1)", Value::Boolean(false)),
            ("(is:zero? -1.1)", Value::Boolean(false)),
            ("(is:zero? 0.0)", Value::Boolean(true)),
            ("(is:zero? 1.1)", Value::Boolean(false)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_false_when_not_an_int_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:zero? #t)", Value::Boolean(false)),
            ("(is:zero? #f)", Value::Boolean(false)),
            ("(is:zero? \"string\")", Value::Boolean(false)),
            ("(is:zero? 'symbol)", Value::Boolean(false)),
            ("(is:zero? :keyword)", Value::Boolean(false)),
            ("(is:zero? (cons:new 1 2))", Value::Boolean(false)),
            ("(is:zero? {})", Value::Boolean(false)),
            ("(is:zero? #())", Value::Boolean(false)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:zero?)", "(is:zero? 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        )
    }
}
