use crate::interpreter::evaluator::evaluate_value::evaluate_value;
use crate::interpreter::evaluator::extract_arguments::extract_arguments;
use crate::ConsId;
use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::KeywordId;
use crate::Value;

pub fn evaluate_s_expression_keyword(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    keyword_id: KeywordId,
    cons_id: ConsId,
) -> Result<Value, Error> {
    let keyword_name = interpreter
        .get_keyword(keyword_id)
        .map(|keyword| keyword.get_name().clone())?;

    let symbol_id = interpreter.intern_symbol_id(&keyword_name);

    let mut arguments = extract_arguments(interpreter, cons_id)?;

    if arguments.len() != 1 {
        return Error::generic_execution_error(
            "Invalid argument count in keyword s-expression.",
        )
        .into();
    }

    let argument = arguments.remove(0);

    let evaluated_argument =
        evaluate_value(interpreter, environment_id, argument)?;

    match evaluated_argument {
        Value::Object(object_id) => interpreter
            .get_object_property(object_id, symbol_id)?
            .ok_or_else(|| {
                Error::generic_execution_error(
                    "Object have not an item to yield.",
                )
            }),
        _ => {
            return Error::generic_execution_error(
                "Cannot get an item of not an object.",
            )
            .into();
        },
    }
}
