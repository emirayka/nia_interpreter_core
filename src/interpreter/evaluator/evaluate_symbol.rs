use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::SymbolId;
use crate::Value;

pub fn evaluate_symbol(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    symbol_id: SymbolId,
) -> Result<Value, Error> {
    if interpreter.check_if_symbol_special(symbol_id)? {
        return Error::generic_execution_error(
            "Cannot evaluate special symbols.",
        )
        .into();
    }

    let evaluation_result = match interpreter
        .lookup_variable(environment_id, symbol_id)?
    {
        Some(result) => result,
        None => match interpreter.get_special_variable(symbol_id) {
            Some(func) => return func(interpreter),
            None => {
                let variable_name = interpreter.get_symbol_name(symbol_id)?;

                return Error::generic_execution_error(&format!(
                    "Cannot find variable `{}'.",
                    variable_name
                ))
                .into();
            },
        },
    };

    Ok(evaluation_result)
}
