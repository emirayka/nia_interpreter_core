use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn block(
    interpreter: &mut Interpreter,
    execution_environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let results = crate::library::evaluate_forms(
        interpreter,
        execution_environment,
        values,
    )?;

    Ok(interpreter.vec_to_list(results))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_list_of_execution_results() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("(block)", "'()"),
            ("(block 1)", "'(1)"),
            ("(block 1 2)", "'(1 2)"),
            ("(block 1 2 3)", "'(1 2 3)"),
        ];

        utils::assert_results_are_equal(&mut interpreter, specs);
    }
}
