use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Function;
use crate::interpreter::value::FunctionId;
use crate::interpreter::value::Value;

pub fn execute_function(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    function_id: FunctionId,
    arguments: Vec<Value>,
) -> Result<Value, Error> {
    let function = interpreter.get_function(function_id)?.clone();

    match function {
        Function::Interpreted(interpreted_function) => interpreter
            .execute_interpreted_function(&interpreted_function, arguments),
        Function::Builtin(builtin_function) => interpreter
            .execute_builtin_function(
                &builtin_function,
                environment_id,
                arguments,
            ),
        _ => Error::invalid_argument_error("").into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use std::convert::TryInto;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    fn execute_forms(
        interpreter: &mut Interpreter,
        values: Vec<&str>,
    ) -> Vec<Value> {
        let mut results = Vec::new();

        for value in values {
            let result =
                interpreter.execute_in_main_environment(value).unwrap();

            results.push(result);
        }

        results
    }

    #[test]
    fn returns_result_for_functions() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("(flookup '+)", vec!["1", "2", "3"], "6"),
            (
                "(function (lambda (a b c) (+ a b c)))",
                vec!["1", "2", "3"],
                "6",
            ),
        ];

        for spec in specs {
            let function_id = interpreter
                .execute_in_main_environment(spec.0)
                .unwrap()
                .try_into()
                .unwrap();

            let arguments = execute_forms(&mut interpreter, spec.1);
            let expected =
                interpreter.execute_in_main_environment(spec.2).unwrap();
            let environment_id = interpreter.get_root_environment_id();

            let result = execute_function(
                &mut interpreter,
                environment_id,
                function_id,
                arguments,
            )
            .unwrap();

            assertion::assert_deep_equal(&mut interpreter, expected, result);
        }
    }

    #[test]
    fn returns_error_for_macros_and_special_forms() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("(flookup 'quote)", vec!["1"]),
            ("(function (macro (a b c) 2))", vec!["1", "2", "3"]),
        ];

        for spec in specs {
            let function_id = interpreter
                .execute_in_main_environment(spec.0)
                .unwrap()
                .try_into()
                .unwrap();

            let arguments = execute_forms(&mut interpreter, spec.1);
            let environment_id = interpreter.get_root_environment_id();

            let result = execute_function(
                &mut interpreter,
                environment_id,
                function_id,
                arguments,
            );

            nia_assert(result.is_err())
        }
    }
}
