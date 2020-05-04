use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn t(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 0 {
        return Error::invalid_argument_count_error(
            "Built-in function `func:t' takes no arguments.",
        )
        .into();
    }

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_true() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![("(func:t)", "#t")];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(func:t 1)", "(func:t 1 2)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
