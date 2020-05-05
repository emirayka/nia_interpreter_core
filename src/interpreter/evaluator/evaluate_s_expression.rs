use crate::interpreter::evaluator::evaluate_s_expression_function_invocation::evaluate_s_expression_function_invocation;
use crate::interpreter::evaluator::evaluate_s_expression_keyword_invocation::evaluate_s_expression_keyword;
use crate::ConsId;
use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn evaluate_s_expression(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    s_expression: ConsId,
) -> Result<Value, Error> {
    // 1) evaluate first symbol
    let car = interpreter.get_car(s_expression)?;

    match car {
        Value::Symbol(func_symbol_id) => {
            let function_value = match interpreter.lookup_function(
                environment_id,
                func_symbol_id,
            )? {
                Some(function_value) => function_value,
                None => {
                    let function_name = interpreter.get_symbol_name(func_symbol_id)?;

                    return Error::generic_execution_error(
                        &format!("Cannot find function `{}'.", function_name)
                    ).into();
                }
            };

            let function_id = match function_value {
                Value::Function(function_id) => function_id,
                _ => return Error::generic_execution_error(
                    "The result of evaluation of first item of an s-expression must be a function or keyword."
                ).into(),
            };

            evaluate_s_expression_function_invocation(
                interpreter,
                environment_id,
                function_id,
                s_expression,
            )
        }
        Value::Function(function_id) => evaluate_s_expression_function_invocation(
            interpreter,
            environment_id,
            function_id,
            s_expression,
        ),
        Value::Cons(cons_id) => {
            let evaluation_result = evaluate_s_expression(
                interpreter,
                environment_id,
                cons_id,
            )?;

            let function_id = match evaluation_result {
                Value::Function(function_id) => function_id,
                _ => return Error::generic_execution_error(
                    "."
                ).into(),
            };

            evaluate_s_expression_function_invocation(
                interpreter,
                environment_id,
                function_id,
                s_expression,
            )
        }
        Value::Keyword(keyword_id) => evaluate_s_expression_keyword(
            interpreter,
            environment_id,
            keyword_id,
            s_expression,
        ),
        _ => return Error::generic_execution_error(
            "The result of evaluation of first item of an s-expression must be a function or keyword."
        ).into(),
    }
}
