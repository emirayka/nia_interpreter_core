use crate::interpreter::evaluator::evaluate_value::evaluate_value;
use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn evaluate_values(
    interpreter: &mut Interpreter,
    execution_environment: EnvironmentId,
    code: &Vec<Value>,
) -> Result<Option<Value>, Error> {
    let mut last_result = None;

    for value in code {
        last_result =
            evaluate_value(interpreter, execution_environment, *value)
                .map(|v| Some(v))?;
    }

    Ok(last_result)
}
