use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn positive_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `is:positive?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Integer(val) => val > 0,
        Value::Float(val) => val > 0.0,
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
    use crate::utils::assertion;

    #[test]
    fn returns_true_when_posative_value_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:positive? -1)", Value::Boolean(false)),
            ("(is:positive? 0)", Value::Boolean(false)),
            ("(is:positive? 1)", Value::Boolean(true)),
            ("(is:positive? -1.1)", Value::Boolean(false)),
            ("(is:positive? 0.0)", Value::Boolean(false)),
            ("(is:positive? 1.1)", Value::Boolean(true)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_false_when_not_an_int_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:positive? #t)", Value::Boolean(false)),
            ("(is:positive? #f)", Value::Boolean(false)),
            ("(is:positive? \"string\")", Value::Boolean(false)),
            ("(is:positive? 'symbol)", Value::Boolean(false)),
            ("(is:positive? :keyword)", Value::Boolean(false)),
            ("(is:positive? (cons 1 2))", Value::Boolean(false)),
            ("(is:positive? {})", Value::Boolean(false)),
            ("(is:positive? #())", Value::Boolean(false)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:positive?)", "(is:positive? 1 2)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector)
    }
}
