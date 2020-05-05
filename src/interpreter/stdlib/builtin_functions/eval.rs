use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn eval(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `eval' takes one argument exactly .",
        )
        .into();
    }

    let mut values = values;
    let value = values.remove(0);

    interpreter.execute_value(environment_id, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn evaluates_provided_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(eval 1)", "1"),
            ("(eval 1.1)", "1.1"),
            ("(eval #f)", "#f"),
            ("(eval #t)", "#t"),
            ("(eval \"string\")", "\"string\""),
            ("(defv a 1) (eval 'a)", "1"),
            ("(eval '(+ 2 2))", "4"),
            ("(eval :keyword)", ":keyword"),
            ("(eval {})", "{}"),
            ("(eval #())", "#()"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn respects_environment() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(defv a 1) (eval 'a)", "1"),
            ("(let (a) (eval 'a))", "nil"),
            ("(defv b 1) (let ((b 2)) (eval 'b))", "2"),
            ("(let ((b 2)) (let ((b 3)) (eval 'b)))", "3"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_count_when_incorrect_count_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(eval)", "(eval 1 2)", "(eval 1 2 3)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
