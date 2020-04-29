use crate::interpreter::error::Error;
use crate::interpreter::value::Function;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn macro_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `is:macro?' must take exactly one argument."
        ).into();
    }

    let mut values = values;

    let function = match library::read_as_function(
        interpreter,
        values.remove(0)
    ) {
        Ok(function) => function,
       _ => return Ok(Value::Boolean(false))
    };

    let result = match function {
        Function::Macro(_) => true,
        _ => false
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_true_when_an_macro_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:macro? (function (macro () 2)))", Value::Boolean(true)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_not_an_macro_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:macro? 1)", Value::Boolean(false)),
            ("(is:macro? 1.1)", Value::Boolean(false)),
            ("(is:macro? #t)", Value::Boolean(false)),
            ("(is:macro? #f)", Value::Boolean(false)),
            ("(is:macro? \"string\")", Value::Boolean(false)),
            ("(is:macro? 'symbol)", Value::Boolean(false)),
            ("(is:macro? :keyword)", Value::Boolean(false)),
            ("(is:macro? (cons 1 2))", Value::Boolean(false)),
            ("(is:macro? {})", Value::Boolean(false)),
            ("(is:macro? (flookup 'flookup))", Value::Boolean(false)),
            ("(is:macro? (flookup 'cond))", Value::Boolean(false)),
            ("(is:macro? #())", Value::Boolean(false)),
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
            "(is:macro?)",
            "(is:macro? 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}
