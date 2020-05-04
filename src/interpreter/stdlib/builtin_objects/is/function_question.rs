use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn function_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `function?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Function(_) => true,
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
    fn returns_true_when_an_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:function? (flookup 'flookup))", Value::Boolean(true)),
            ("(is:function? #())", Value::Boolean(true)),
            ("(is:function? (flookup 'cond))", Value::Boolean(true)),
            (
                "(is:function? (function (macro () 2)))",
                Value::Boolean(true),
            ),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_false_when_not_an_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:function? 1)", Value::Boolean(false)),
            ("(is:function? 1.1)", Value::Boolean(false)),
            ("(is:function? #t)", Value::Boolean(false)),
            ("(is:function? #f)", Value::Boolean(false)),
            ("(is:function? \"string\")", Value::Boolean(false)),
            ("(is:function? 'symbol)", Value::Boolean(false)),
            ("(is:function? :keyword)", Value::Boolean(false)),
            ("(is:function? (cons 1 2))", Value::Boolean(false)),
            ("(is:function? {})", Value::Boolean(false)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:function?)", "(is:function? 1 2)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector)
    }
}
