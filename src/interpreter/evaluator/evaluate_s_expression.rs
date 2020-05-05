use crate::interpreter::evaluator::evaluate_s_expression_function_invocation::evaluate_s_expression_function_invocation;
use crate::interpreter::evaluator::evaluate_s_expression_keyword_invocation::evaluate_s_expression_keyword;
use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::Value;
use crate::{CallStackItem, ConsId};

pub fn evaluate_s_expression(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    s_expression: ConsId,
) -> Result<Value, Error> {
    if interpreter.is_overflow() {
        return Error::stack_overflow_error().into();
    }

    // 1) evaluate first symbol
    let car = interpreter.get_car(s_expression)?;

    match car {
        Value::Symbol(function_symbol_id) => {
            let function_value = match interpreter.lookup_function(
                environment_id,
                function_symbol_id,
            )? {
                Some(function_value) => function_value,
                None => {
                    let function_name = interpreter.get_symbol_name(function_symbol_id)?;

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
                function_symbol_id,
                function_id,
                s_expression,
            )
        }
        Value::Function(function_id) => {
            let nil_symbol_id = interpreter.intern_nil_symbol_id();

            evaluate_s_expression_function_invocation(
                interpreter,
                environment_id,
                nil_symbol_id,
                function_id,
                s_expression,
            )
        },
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

            let nil_symbol_id = interpreter.intern_nil_symbol_id();

            evaluate_s_expression_function_invocation(
                interpreter,
                environment_id,
                nil_symbol_id,
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
