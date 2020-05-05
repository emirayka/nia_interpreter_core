use crate::interpreter::evaluator::define_environment_functions::define_environment_functions;
use crate::interpreter::evaluator::define_environment_variables::define_environment_variables;
use crate::interpreter::evaluator::evaluate_values::evaluate_values;
use crate::Error;
use crate::InterpretedFunction;
use crate::Interpreter;
use crate::Value;

pub fn evaluate_interpreted_function_invocation(
    interpreter: &mut Interpreter,
    func: &InterpretedFunction,
    evaluated_arguments: Vec<Value>,
) -> Result<Value, Error> {
    if func.get_arguments().required_len() > evaluated_arguments.len() {
        return Error::generic_execution_error(
            "Not enough arguments to call a function.",
        )
        .into();
    }

    // 1) make new environment
    let execution_environment_id =
        interpreter.make_environment(func.get_environment())?;

    // 2) setup environment variables and functions
    define_environment_variables(
        interpreter,
        execution_environment_id,
        func.get_arguments(),
        &evaluated_arguments,
    )?;

    define_environment_functions(
        interpreter,
        execution_environment_id,
        func.get_arguments(),
        &evaluated_arguments,
    )?;

    // 3) execute code
    let execution_result = evaluate_values(
        interpreter,
        execution_environment_id,
        func.get_code(),
    )?;

    // 4) return result
    let result = execution_result
        .unwrap_or_else(|| interpreter.intern_nil_symbol_value());

    Ok(result)
}
