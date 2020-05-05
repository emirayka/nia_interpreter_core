use crate::interpreter::evaluator::evaluate_value::evaluate_value;
use crate::EnvironmentId;
use crate::Error;
use crate::FunctionArguments;
use crate::Interpreter;
use crate::Value;

mod define_ordinary_arguments {
    use super::*;

    pub fn define_ordinary_arguments<'a, I: Iterator<Item = &'a Value>>(
        interpreter: &mut Interpreter,
        execution_environment_id: EnvironmentId,
        arguments: &FunctionArguments,
        values: &mut I,
    ) -> Result<(), Error> {
        for ordinary_argument in arguments.get_ordinary_arguments() {
            let argument_value = values.next().ok_or_else(|| {
                Error::generic_execution_error(
                    "Not enough arguments were passed to a function.",
                )
            })?;

            let symbol_id = interpreter.intern_symbol_id(ordinary_argument);
            interpreter.define_function(
                execution_environment_id,
                symbol_id,
                *argument_value,
            )?;
        }

        Ok(())
    }
}

mod define_optional_arguments {
    use super::*;

    pub fn define_optional_arguments<'a, I: Iterator<Item = &'a Value>>(
        interpreter: &mut Interpreter,
        execution_environment_id: EnvironmentId,
        arguments: &FunctionArguments,
        values: &mut I,
    ) -> Result<(), Error> {
        for optional_argument in arguments.get_optional_arguments() {
            let function_symbol_id =
                interpreter.intern_symbol_id(optional_argument.get_name());

            match values.next() {
                Some(value) => {
                    interpreter.define_function(
                        execution_environment_id,
                        function_symbol_id,
                        *value,
                    )?;

                    if let Some(provided_name) =
                        optional_argument.get_provided()
                    {
                        let function_symbol_id =
                            interpreter.intern_symbol_id(provided_name);

                        interpreter.define_function(
                            execution_environment_id,
                            function_symbol_id,
                            Value::Boolean(true),
                        )?;
                    }
                },
                None => {
                    let value = match optional_argument.get_default() {
                        Some(default_value) => evaluate_value(
                            interpreter,
                            execution_environment_id,
                            default_value,
                        )?,
                        None => interpreter.intern_nil_symbol_value(),
                    };
                    interpreter.define_function(
                        execution_environment_id,
                        function_symbol_id,
                        value,
                    )?;

                    if let Some(provided_name) =
                        optional_argument.get_provided()
                    {
                        let function_symbol_id =
                            interpreter.intern_symbol_id(provided_name);

                        interpreter.define_function(
                            execution_environment_id,
                            function_symbol_id,
                            Value::Boolean(false),
                        )?;
                    }
                },
            }
        }

        Ok(())
    }
}

pub fn define_environment_functions(
    interpreter: &mut Interpreter,
    execution_environment_id: EnvironmentId,
    arguments: &FunctionArguments,
    values: &Vec<Value>,
) -> Result<(), Error> {
    let mut iterator = values.iter().peekable();

    define_ordinary_arguments::define_ordinary_arguments(
        interpreter,
        execution_environment_id,
        arguments,
        &mut iterator,
    );

    define_optional_arguments::define_optional_arguments(
        interpreter,
        execution_environment_id,
        arguments,
        &mut iterator,
    );

    Ok(())
}
