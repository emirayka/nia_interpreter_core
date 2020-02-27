use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn builtin_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `is:builtin?' must take exactly one argument."
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
                Function::Builtin(_) => true,
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
    fn returns_true_when_an_builtin_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:builtin? (flookup 'flookup))", Value::Boolean(true))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_not_an_builtin_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:builtin? 1)", Value::Boolean(false)),
            ("(is:builtin? 1.1)", Value::Boolean(false)),
            ("(is:builtin? #t)", Value::Boolean(false)),
            ("(is:builtin? #f)", Value::Boolean(false)),
            ("(is:builtin? \"string\")", Value::Boolean(false)),
            ("(is:builtin? 'symbol)", Value::Boolean(false)),
            ("(is:builtin? :keyword)", Value::Boolean(false)),
            ("(is:builtin? (cons 1 2))", Value::Boolean(false)),
            ("(is:builtin? {})", Value::Boolean(false)),
            ("(is:builtin? #())", Value::Boolean(false)),
            ("(is:builtin? (flookup 'cond))", Value::Boolean(false)),
            ("(is:builtin? (function (macro () 2)))", Value::Boolean(false)),
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
            "(is:builtin?)",
            "(is:builtin? 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}
