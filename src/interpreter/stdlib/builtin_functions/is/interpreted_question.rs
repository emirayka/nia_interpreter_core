use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn interpreted_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `is:interpreted?' must take exactly one argument."
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
                Function::Interpreted(_) => true,
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
    fn returns_true_when_an_interpreted_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:interpreted? #())", Value::Boolean(true)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_not_an_interpreted_function_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:interpreted? 1)", Value::Boolean(false)),
            ("(is:interpreted? 1.1)", Value::Boolean(false)),
            ("(is:interpreted? #t)", Value::Boolean(false)),
            ("(is:interpreted? #f)", Value::Boolean(false)),
            ("(is:interpreted? \"string\")", Value::Boolean(false)),
            ("(is:interpreted? 'symbol)", Value::Boolean(false)),
            ("(is:interpreted? :keyword)", Value::Boolean(false)),
            ("(is:interpreted? (cons 1 2))", Value::Boolean(false)),
            ("(is:interpreted? {})", Value::Boolean(false)),
            ("(is:interpreted? (flookup 'flookup))", Value::Boolean(false)),
            ("(is:interpreted? (flookup 'cond))", Value::Boolean(false)),
            ("(is:interpreted? (function (macro () 2)))", Value::Boolean(false)),
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
            "(is:interpreted?)",
            "(is:interpreted? 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}
