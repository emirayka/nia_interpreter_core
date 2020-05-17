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
use crate::SymbolId;
use crate::Value;

pub fn evaluate_s_expression_function_invocation(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    function_symbol_id: SymbolId,
    function_id: FunctionId,
    cons_id: ConsId,
) -> Result<Value, Error> {
    let function = interpreter
        .get_function(function_id)
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

            if interpreter.symbol_is_not_nil(function_symbol_id)? {
                interpreter.push_named_call(
                    function_id,
                    function_symbol_id,
                    evaluated_arguments.clone(),
                );
            } else {
                interpreter.push_anonymous_call(
                    function_id,
                    evaluated_arguments.clone(),
                );
            }

            // 3) apply function from step 1 to arguments from step 2
            let result = evaluate_builtin_function_invocation(
                interpreter,
                &builtin_function,
                environment_id,
                evaluated_arguments,
            );

            interpreter.pop_call();
            result
        }
        Function::Interpreted(interpreted_function) => {
            // 2) evaluate arguments
            let arguments = extract_arguments(interpreter, cons_id)?;

            let evaluated_arguments = crate::library::evaluate_forms(
                interpreter,
                environment_id,
                arguments,
            )?;

            if interpreter.symbol_is_not_nil(function_symbol_id)? {
                interpreter.push_named_call(
                    function_id,
                    function_symbol_id,
                    evaluated_arguments.clone(),
                );
            } else {
                interpreter.push_anonymous_call(
                    function_id,
                    evaluated_arguments.clone(),
                );
            }

            // 3) apply function from step 1 to arguments from step 2
            let result = evaluate_interpreted_function_invocation(
                interpreter,
                &interpreted_function,
                evaluated_arguments,
            );

            interpreter.pop_call();
            result
        }
        Function::SpecialForm(special_form) => {
            let arguments = extract_arguments(interpreter, cons_id)?;

            if interpreter.symbol_is_not_nil(function_symbol_id)? {
                interpreter.push_named_call(
                    function_id,
                    function_symbol_id,
                    arguments.clone(),
                );
            } else {
                interpreter.push_anonymous_call(function_id, arguments.clone());
            }

            let result = evaluate_special_form_invocation(
                interpreter,
                environment_id,
                &special_form,
                arguments,
            );

            interpreter.pop_call();
            result
        }
        Function::Macro(macro_function) => {
            let arguments = extract_arguments(interpreter, cons_id)?;

            if interpreter.symbol_is_not_nil(function_symbol_id)? {
                interpreter.push_named_call(
                    function_id,
                    function_symbol_id,
                    arguments.clone(),
                );
            } else {
                interpreter.push_anonymous_call(function_id, arguments.clone());
            }

            let evaluation_result = evaluate_macro_invocation(
                interpreter,
                &macro_function,
                arguments,
            )?;

            let result =
                evaluate_value(interpreter, environment_id, evaluation_result);

            interpreter.pop_call();
            result
        }
    }
}
