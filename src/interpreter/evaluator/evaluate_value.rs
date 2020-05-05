use crate::interpreter::evaluator::evaluate_s_expression::evaluate_s_expression;
use crate::interpreter::evaluator::evaluate_symbol::evaluate_symbol;
use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn evaluate_value(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    value: Value,
) -> Result<Value, Error> {
    match value {
        Value::Symbol(symbol_name) => {
            evaluate_symbol(interpreter, environment, symbol_name)
        },
        Value::Cons(cons) => {
            evaluate_s_expression(interpreter, environment, cons)
        },
        _ => Ok(value),
    }
}
