use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn nil_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `is:nil?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Symbol(symbol_id) => interpreter.symbol_is_nil(symbol_id)?,
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
    fn returns_true_when_an_symbol_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![("(is:nil? nil)", Value::Boolean(true))];

        utils::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_false_when_not_an_symbol_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:nil? 1)", Value::Boolean(false)),
            ("(is:nil? 1.1)", Value::Boolean(false)),
            ("(is:nil? #t)", Value::Boolean(false)),
            ("(is:nil? #f)", Value::Boolean(false)),
            ("(is:nil? \"string\")", Value::Boolean(false)),
            ("(is:nil? :keyword)", Value::Boolean(false)),
            ("(is:nil? 'symbol)", Value::Boolean(false)),
            ("(is:nil? (cons:new 1 2))", Value::Boolean(false)),
            ("(is:nil? {})", Value::Boolean(false)),
            ("(is:nil? #())", Value::Boolean(false)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:nil?)", "(is:nil? 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        )
    }
}
