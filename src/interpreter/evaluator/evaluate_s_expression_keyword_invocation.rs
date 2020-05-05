use crate::interpreter::evaluator::evaluate_value::evaluate_value;
use crate::interpreter::evaluator::extract_arguments::extract_arguments;
use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::KeywordId;
use crate::Value;
use crate::{ConsId, SymbolId};

fn evaluate_s_expression_keyword_get(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    property_symbol_id: SymbolId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let mut values = values;
    let object_value = values.remove(0);

    let evaluated_argument =
        evaluate_value(interpreter, environment_id, object_value)?;

    let value = match evaluated_argument {
        Value::Object(object_id) => {
            interpreter.get_object_property(object_id, property_symbol_id)?
        },
        _ => {
            return Error::generic_execution_error(
                "Cannot get an item of not an object.",
            )
            .into();
        },
    };

    match value {
        Some(value) => Ok(value),
        None => Ok(interpreter.intern_nil_symbol_value()),
    }
}

fn evaluate_s_expression_keyword_set(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    property_symbol_id: SymbolId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let mut values = values;

    let object_value = values.remove(0);
    let property_new_value = values.remove(0);

    let evaluated_object_value =
        evaluate_value(interpreter, environment_id, object_value)?;

    let value = match evaluated_object_value {
        Value::Object(object_id) => interpreter.set_object_property(
            object_id,
            property_symbol_id,
            property_new_value,
        )?,
        _ => {
            return Error::generic_execution_error(
                "Cannot get an item of not an object.",
            )
            .into();
        },
    };

    Ok(property_new_value)
}

pub fn evaluate_s_expression_keyword(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    keyword_id: KeywordId,
    cons_id: ConsId,
) -> Result<Value, Error> {
    let keyword_name = interpreter
        .get_keyword(keyword_id)
        .map(|keyword| keyword.get_name().clone())?;

    let property_symbol_id = interpreter.intern_symbol_id(&keyword_name);
    let mut arguments = extract_arguments(interpreter, cons_id)?;

    match arguments.len() {
        1 => evaluate_s_expression_keyword_get(
            interpreter,
            environment_id,
            property_symbol_id,
            arguments,
        ),
        2 => evaluate_s_expression_keyword_set(
            interpreter,
            environment_id,
            property_symbol_id,
            arguments,
        ),
        _ => {
            return Error::generic_execution_error(
                "Invalid argument count in keyword s-expression.",
            )
            .into();
        },
    }
}
