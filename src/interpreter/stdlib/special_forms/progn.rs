use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::library;

pub fn progn(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    library::execute_forms(
        interpreter,
        environment,
        &values
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("nil", "(progn)"),
            ("3", "(progn 3)"),
            ("2", "(progn 3 2)"),
            ("1", "(progn 3 2 1)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            specs
        );
    }
}
