use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn cons_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `cons?' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Cons(_) => true,
        _ => false
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_true_when_an_cons_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:cons? (cons 1 nil))", Value::Boolean(true)),
            ("(is:cons? (cons 1 2))", Value::Boolean(true)),
            ("(is:cons? (cons 1 (cons 2 nil)))", Value::Boolean(true)),
            ("(is:cons? (cons 1 (cons 2 3)))", Value::Boolean(true)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_false_when_not_an_cons_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:cons? 1)", Value::Boolean(false)),
            ("(is:cons? 1.1)", Value::Boolean(false)),
            ("(is:cons? #t)", Value::Boolean(false)),
            ("(is:cons? #f)", Value::Boolean(false)),
            ("(is:cons? \"string\")", Value::Boolean(false)),
            ("(is:cons? 'symbol)", Value::Boolean(false)),
            ("(is:cons? :keyword)", Value::Boolean(false)),
            ("(is:cons? {})", Value::Boolean(false)),
            ("(is:cons? #())", Value::Boolean(false)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(is:cons?)",
            "(is:cons? 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}
