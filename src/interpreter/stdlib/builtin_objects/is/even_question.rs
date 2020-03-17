use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn even_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `is:even?' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Integer(int) => int % 2 == 0,
        _ => false
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_true_when_an_even_int_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:even? 1)", Value::Boolean(false)),
            ("(is:even? 2)", Value::Boolean(true)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_not_an_int_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:even? 1.1)", Value::Boolean(false)),
            ("(is:even? #t)", Value::Boolean(false)),
            ("(is:even? #f)", Value::Boolean(false)),
            ("(is:even? \"string\")", Value::Boolean(false)),
            ("(is:even? 'symbol)", Value::Boolean(false)),
            ("(is:even? :keyword)", Value::Boolean(false)),
            ("(is:even? (cons 1 2))", Value::Boolean(false)),
            ("(is:even? {})", Value::Boolean(false)),
            ("(is:even? #())", Value::Boolean(false)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(is:even?)",
            "(is:even? 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}
