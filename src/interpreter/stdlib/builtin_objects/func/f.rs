use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn f(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 0 {
        return Error::invalid_argument_count_error(
            "Built-in function `func:t' takes no arguments."
        ).into();
    }

    Ok(Value::Boolean(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_false() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(func:f)", "#f"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(func:f 1)",
            "(func:f 1 2)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
