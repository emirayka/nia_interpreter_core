use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn id(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `func:id' takes one argument.",
        )
        .into();
    }

    let mut values = values;

    Ok(values.remove(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_the_same_value() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(func:id 1)", "1"),
            ("(func:id 1.1)", "1.1"),
            ("(func:id #t)", "#t"),
            ("(func:id #f)", "#f"),
            ("(func:id \"string\")", "\"string\""),
            ("(func:id 'symbol)", "'symbol"),
            ("(func:id :keyword)", ":keyword"),
            ("(func:id '(1 2))", "'(1 2)"),
            ("(func:id {})", "{}"),
            ("(func:id #())", "#()"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(func:id)", "(func:id 1 2)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
