use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Function;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn special_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `is:special?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let function =
        match library::read_as_function(interpreter, values.remove(0)) {
            Ok(function) => function,
            _ => return Ok(Value::Boolean(false)),
        };

    let result = match function {
        Function::SpecialForm(_) => true,
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
    fn returns_true_when_an_special_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs =
            vec![("(is:special? (flookup 'cond))", Value::Boolean(true))];

        utils::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_false_when_not_an_special_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:special? 1)", Value::Boolean(false)),
            ("(is:special? 1.1)", Value::Boolean(false)),
            ("(is:special? #t)", Value::Boolean(false)),
            ("(is:special? #f)", Value::Boolean(false)),
            ("(is:special? \"string\")", Value::Boolean(false)),
            ("(is:special? 'symbol)", Value::Boolean(false)),
            ("(is:special? :keyword)", Value::Boolean(false)),
            ("(is:special? (cons 1 2))", Value::Boolean(false)),
            ("(is:special? {})", Value::Boolean(false)),
            ("(is:special? #())", Value::Boolean(false)),
            (
                "(is:special? (function (macro () 2)))",
                Value::Boolean(false),
            ),
            ("(is:special? (flookup 'flookup))", Value::Boolean(false)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:special?)", "(is:special? 1 2)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        )
    }
}
