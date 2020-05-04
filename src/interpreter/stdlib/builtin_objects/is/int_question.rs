use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn int_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `int?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Integer(_) => true,
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
    fn returns_true_when_an_int_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![("(is:int? 1)", Value::Boolean(true))];

        assertion::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_false_when_not_an_int_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:int? 1.1)", Value::Boolean(false)),
            ("(is:int? #t)", Value::Boolean(false)),
            ("(is:int? #f)", Value::Boolean(false)),
            ("(is:int? \"string\")", Value::Boolean(false)),
            ("(is:int? 'symbol)", Value::Boolean(false)),
            ("(is:int? :keyword)", Value::Boolean(false)),
            ("(is:int? (cons 1 2))", Value::Boolean(false)),
            ("(is:int? {})", Value::Boolean(false)),
            ("(is:int? #())", Value::Boolean(false)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:int?)", "(is:int? 1 2)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector)
    }
}
