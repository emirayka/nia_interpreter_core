use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::FunctionId;
use crate::interpreter::function::Function;

pub fn execute_function(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    function_id: FunctionId,
    arguments: Vec<Value>
) -> Result<Value, Error> {
    let function = interpreter.get_function(function_id)?
        .clone();

    match function {
        Function::Interpreted(interpreted_function) => {
            interpreter.evaluate_interpreted_function_invocation(
                &interpreted_function,
                arguments
            )
        },
        Function::Builtin(builtin_function) => {
            interpreter.evaluate_builtin_function_invocation(
                &builtin_function,
                environment_id,
                arguments
            )
        },
        _ => interpreter.make_invalid_argument_error(
            ""
        ).into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    fn execute_forms(interpreter: &mut Interpreter, values: Vec<&str>) -> Vec<Value> {
        let mut results = Vec::new();

        for value in values {
            let result = interpreter.execute(value).unwrap();

            results.push(result);
        }

        results
    }

    #[test]
    fn returns_result_for_functions() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("(flookup '+)", vec!("1", "2", "3"), "6"),
            ("(function (lambda (a b c) (+ a b c)))", vec!("1", "2", "3"), "6"),
        );

        for spec in specs {
            let function_id = interpreter.execute(spec.0)
                .unwrap()
                .as_function();

            let arguments = execute_forms(&mut interpreter, spec.1);
            let expected = interpreter.execute(spec.2).unwrap();
            let environment_id = interpreter.get_root_environment();

            let result = execute_function(
                &mut interpreter,
                environment_id,
                function_id,
                arguments
            ).unwrap();

            assertion::assert_deep_equal(
                &mut interpreter,
                expected,
                result
            );
        }
    }

    #[test]
    fn returns_error_for_macros_and_special_forms() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("(flookup 'quote)", vec!("1")),
            ("(function (macro (a b c) 2))", vec!("1", "2", "3")),
        );

        for spec in specs {
            let function_id = interpreter.execute(spec.0)
                .unwrap()
                .as_function();

            let arguments = execute_forms(&mut interpreter, spec.1);
            let environment_id = interpreter.get_root_environment();

            let result = execute_function(
                &mut interpreter,
                environment_id,
                function_id,
                arguments
            );

            assert!(result.is_err())
        }
    }
}
