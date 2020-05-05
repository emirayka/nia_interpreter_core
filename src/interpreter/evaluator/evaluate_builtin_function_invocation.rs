use crate::BuiltinFunction;
use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn evaluate_builtin_function_invocation(
    interpreter: &mut Interpreter,
    builtin_function: &BuiltinFunction,
    execution_environment: EnvironmentId,
    evaluated_arguments: Vec<Value>,
) -> Result<Value, Error> {
    (builtin_function.get_func())(
        interpreter,
        execution_environment,
        evaluated_arguments,
    )
}
