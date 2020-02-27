use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn special_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `is:special?' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Function(function_id) => {
            let function = match interpreter.get_function(function_id) {
                Ok(function) => function,
                Err(error) => return interpreter.make_generic_execution_error_caused(
                    "",
                    error
                ).into_result()
            };

            match function {
                Function::SpecialForm(_) => true,
                _ => false
            }
        },
        _ => false
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_true_when_an_special_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:special? (flookup 'cond))", Value::Boolean(true)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_not_an_special_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
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
            ("(is:special? (function (macro () 2)))", Value::Boolean(false)),
            ("(is:special? (flookup 'flookup))", Value::Boolean(false))
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
            "(is:special?)",
            "(is:special? 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}