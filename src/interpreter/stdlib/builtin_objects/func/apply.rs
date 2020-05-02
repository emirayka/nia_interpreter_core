use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;
use crate::interpreter::value::Function;

pub fn apply(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `func:apply' takes two argument exactly."
        ).into();
    }

    let mut values = values;

    let function = library::read_as_function(
        interpreter,
        values.remove(0)
    )?.clone();

    let evaluated_arguments = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    let result = match function {
        Function::Builtin(builtin_function) => {
            interpreter.evaluate_builtin_function_invocation(
                &builtin_function,
                environment_id,
                evaluated_arguments
            )?
        },
        Function::Interpreted(interpreted_function) => {
            interpreter.evaluate_interpreted_function_invocation(
                &interpreted_function,
                evaluated_arguments
            )?
        },
        _ => return Error::invalid_argument_error(
            "Built-in function `func:apply' can invoke only built-in or interpreted functions."
        ).into()
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn calls_a_function_with_provided_arguments() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(func:apply #(+ 1  2  3)  '())",      "6"),
            ("(func:apply #(+ %1 2  3)  '(1))",     "6"),
            ("(func:apply #(+ %1 %2 3)  '(1 2))",   "6"),
            ("(func:apply #(+ %1 %2 %3) '(1 2 3))", "6"),

            ("(func:apply (fn (#opt (a 1) (b 2) (c 3)) (list a b c)) '())", "'(1 2 3)"),
            ("(func:apply (fn (#opt (a 1) (b 2) (c 3)) (list a b c)) '(1))", "'(1 2 3)"),
            ("(func:apply (fn (#opt (a 1) (b 2) (c 3)) (list a b c)) '(1 2))", "'(1 2 3)"),
            ("(func:apply (fn (#opt (a 1) (b 2) (c 3)) (list a b c)) '(1 2 3))", "'(1 2 3)"),

            ("(func:apply (fn (#rest a) a) '())", "'()"),
            ("(func:apply (fn (#rest a) a) '(1))", "'(1)"),
            ("(func:apply (fn (#rest a) a) '(1 2))", "'(1 2)"),
            ("(func:apply (fn (#rest a) a) '(1 2 3))", "'(1 2 3)"),

            ("(func:apply (fn (#keys (a 1) (b 2) (c 3)) (list a b c)) '())", "'(1 2 3)"),
            ("(func:apply (fn (#keys (a 1) (b 2) (c 3)) (list a b c)) '(:a 1))", "'(1 2 3)"),
            ("(func:apply (fn (#keys (a 1) (b 2) (c 3)) (list a b c)) '(:a 1 :b 2))", "'(1 2 3)"),
            ("(func:apply (fn (#keys (a 1) (b 2) (c 3)) (list a b c)) '(:a 1 :b 2 :c 3))", "'(1 2 3)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_macro_or_special_form_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(func:apply (function (macro () 1)) '())",
            "(func:apply (flookup 'cond) '())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(func:apply 1 '())",
            "(func:apply 1.1 '())",
            "(func:apply #t '())",
            "(func:apply #f '())",
            "(func:apply \"string\" '())",
            "(func:apply 'symbol '())",
            "(func:apply :keyword '())",
            "(func:apply '(1 2 3) '())",
            "(func:apply {} '())",

            "(func:apply #(+ %1 %1) 1)",
            "(func:apply #(+ %1 %1) 1.1)",
            "(func:apply #(+ %1 %1) #t)",
            "(func:apply #(+ %1 %1) #f)",
            "(func:apply #(+ %1 %1) \"string\")",
            "(func:apply #(+ %1 %1) 'symbol)",
            "(func:apply #(+ %1 %1) :keyword)",
            "(func:apply #(+ %1 %1) {})",
            "(func:apply #(+ %1 %1) #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(func:apply)",
            "(func:apply #(+ %1 %2) '(2) 3)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
