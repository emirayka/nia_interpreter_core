use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn symbol_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `symbol?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Symbol(_) => true,
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

        let pairs = vec![("(is:symbol? 'symbol)", Value::Boolean(true))];

        utils::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_false_when_not_an_symbol_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:symbol? 1)", Value::Boolean(false)),
            ("(is:symbol? 1.1)", Value::Boolean(false)),
            ("(is:symbol? #t)", Value::Boolean(false)),
            ("(is:symbol? #f)", Value::Boolean(false)),
            ("(is:symbol? \"string\")", Value::Boolean(false)),
            ("(is:symbol? :keyword)", Value::Boolean(false)),
            ("(is:symbol? (cons 1 2))", Value::Boolean(false)),
            ("(is:symbol? {})", Value::Boolean(false)),
            ("(is:symbol? #())", Value::Boolean(false)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:symbol?)", "(is:symbol? 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        )
    }
}
