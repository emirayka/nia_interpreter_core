use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn block(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let values = values;
    let mut results = Vec::new();

    for value in values {
        let result = interpreter.execute_value(environment, value)?;

        results.push(result);
    }

    Ok(interpreter.vec_to_list(results))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_list_of_execution_results() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("(block)", "'()"),
            ("(block 1)", "'(1)"),
            ("(block 1 2)", "'(1 2)"),
            ("(block 1 2 3)", "'(1 2 3)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            specs
        );
    }
}
