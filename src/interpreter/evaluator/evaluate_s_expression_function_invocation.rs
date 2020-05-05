use crate::interpreter::evaluator::evaluate_builtin_function_invocation::evaluate_builtin_function_invocation;
use crate::interpreter::evaluator::evaluate_interpreted_function_invocation::evaluate_interpreted_function_invocation;
use crate::interpreter::evaluator::evaluate_macro_invocation::evaluate_macro_invocation;
use crate::interpreter::evaluator::evaluate_special_form_invocation::evaluate_special_form_invocation;
use crate::interpreter::evaluator::evaluate_value::evaluate_value;
use crate::interpreter::evaluator::extract_arguments::extract_arguments;
use crate::ConsId;
use crate::EnvironmentId;
use crate::Error;
use crate::Function;
use crate::FunctionId;
use crate::Interpreter;
use crate::Value;

pub fn evaluate_s_expression_function_invocation(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    function: FunctionId,
    cons_id: ConsId,
) -> Result<Value, Error> {
    let function = interpreter
        .get_function(function)
        .map(|function| function.clone())?;

    match function {
        Function::Builtin(builtin_function) => {
            // 2) evaluate arguments
            let arguments = extract_arguments(interpreter, cons_id)?;

            let evaluated_arguments = crate::library::evaluate_forms(
                interpreter,
                environment_id,
                arguments,
            )?;

            // 3) apply function from step 1 to arguments from step 2
            evaluate_builtin_function_invocation(
                interpreter,
                &builtin_function,
                environment_id,
                evaluated_arguments,
            )
        },
        Function::Interpreted(interpreted_function) => {
            // 2) evaluate arguments
            let arguments = extract_arguments(interpreter, cons_id)?;

            let evaluated_arguments = crate::library::evaluate_forms(
                interpreter,
                environment_id,
                arguments,
            )?;

            // 3) apply function from step 1 to arguments from step 2
            evaluate_interpreted_function_invocation(
                interpreter,
                &interpreted_function,
                evaluated_arguments,
            )
        },
        Function::SpecialForm(special_form) => {
            let arguments = extract_arguments(interpreter, cons_id)?;

            evaluate_special_form_invocation(
                interpreter,
                environment_id,
                &special_form,
                arguments,
            )
        },
        Function::Macro(macro_function) => {
            let arguments = extract_arguments(interpreter, cons_id)?;
            let evaluation_result = evaluate_macro_invocation(
                interpreter,
                &macro_function,
                arguments,
            )?;

            evaluate_value(interpreter, environment_id, evaluation_result)
        },
    }
}
