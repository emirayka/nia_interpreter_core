use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn number_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `number?' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Integer(_) => true,
        Value::Float(_) => true,
        _ => false
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_true_when_an_int_or_float_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:number? 1)", Value::Boolean(true)),
            ("(is:number? 1.1)", Value::Boolean(true)),
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
            ("(is:number? #t)", Value::Boolean(false)),
            ("(is:number? #f)", Value::Boolean(false)),
            ("(is:number? \"string\")", Value::Boolean(false)),
            ("(is:number? 'symbol)", Value::Boolean(false)),
            ("(is:number? :keyword)", Value::Boolean(false)),
            ("(is:number? (cons 1 2))", Value::Boolean(false)),
            ("(is:number? {})", Value::Boolean(false)),
            ("(is:number? #())", Value::Boolean(false)),
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
            "(is:number?)",
            "(is:number? 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}
