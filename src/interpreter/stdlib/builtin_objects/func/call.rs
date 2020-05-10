use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::interpreter::value::Function;

pub fn call(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `func:call' takes one argument at least.",
        )
        .into();
    }

    let mut values = values;

    let function =
        library::read_as_function(interpreter, values.remove(0))?.clone();

    let evaluated_arguments = values;

    let result =
        match function {
            Function::Builtin(builtin_function) => interpreter.execute_builtin_function(
                &builtin_function,
                environment_id,
                evaluated_arguments,
            )?,
            Function::Interpreted(interpreted_function) => interpreter
                .execute_interpreted_function(&interpreted_function, evaluated_arguments)?,
            _ => return Error::invalid_argument_error(
                "Built-in function `func:call' can invoke only built-in or interpreted functions.",
            )
            .into(),
        };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn calls_a_function_with_provided_arguments() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(func:call #(+ 1  2  3)  )", "6"),
            ("(func:call #(+ %1 2  3)  1)", "6"),
            ("(func:call #(+ %1 %2 3)  1 2)", "6"),
            ("(func:call #(+ %1 %2 %3) 1 2 3)", "6"),
            (
                "(func:call (fn (#opt (a 1) (b 2) (c 3)) (list a b c)))",
                "'(1 2 3)",
            ),
            (
                "(func:call (fn (#opt (a 1) (b 2) (c 3)) (list a b c)) 1)",
                "'(1 2 3)",
            ),
            (
                "(func:call (fn (#opt (a 1) (b 2) (c 3)) (list a b c)) 1 2)",
                "'(1 2 3)",
            ),
            (
                "(func:call (fn (#opt (a 1) (b 2) (c 3)) (list a b c)) 1 2 3)",
                "'(1 2 3)",
            ),
            ("(func:call (fn (#rest a) a))", "'()"),
            ("(func:call (fn (#rest a) a) 1)", "'(1)"),
            ("(func:call (fn (#rest a) a) 1 2)", "'(1 2)"),
            ("(func:call (fn (#rest a) a) 1 2 3)", "'(1 2 3)"),
            (
                "(func:call (fn (#keys (a 1) (b 2) (c 3)) (list a b c)))",
                "'(1 2 3)",
            ),
            (
                "(func:call (fn (#keys (a 1) (b 2) (c 3)) (list a b c)) :a 1)",
                "'(1 2 3)",
            ),
            (
                "(func:call (fn (#keys (a 1) (b 2) (c 3)) (list a b c)) :a 1 :b 2)",
                "'(1 2 3)",
            ),
            (
                "(func:call (fn (#keys (a 1) (b 2) (c 3)) (list a b c)) :a 1 :b 2 :c 3)",
                "'(1 2 3)",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_macro_or_special_form_was_provided()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(func:call (function (macro () 1)))",
            "(func:call (flookup 'cond))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(func:call 1)",
            "(func:call 1.1)",
            "(func:call #t)",
            "(func:call #f)",
            "(func:call \"string\")",
            "(func:call 'symbol)",
            "(func:call :keyword)",
            "(func:call '(1 2 3))",
            "(func:call {})",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(func:call)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
