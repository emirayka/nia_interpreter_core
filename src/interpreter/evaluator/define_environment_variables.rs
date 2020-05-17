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
            interpreter.define_variable(
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
            let variable_symbol_id =
                interpreter.intern_symbol_id(optional_argument.get_name());

            match values.next() {
                Some(value) => {
                    interpreter.define_variable(
                        execution_environment_id,
                        variable_symbol_id,
                        *value,
                    )?;

                    if let Some(provided_name) =
                        optional_argument.get_provided()
                    {
                        let variable_symbol_id =
                            interpreter.intern_symbol_id(provided_name);

                        interpreter.define_variable(
                            execution_environment_id,
                            variable_symbol_id,
                            Value::Boolean(true),
                        )?;
                    }
                }
                None => {
                    let value = match optional_argument.get_default() {
                        Some(default_value) => evaluate_value(
                            interpreter,
                            execution_environment_id,
                            default_value,
                        )?,
                        None => interpreter.intern_nil_symbol_value(),
                    };
                    interpreter.define_variable(
                        execution_environment_id,
                        variable_symbol_id,
                        value,
                    )?;

                    if let Some(provided_name) =
                        optional_argument.get_provided()
                    {
                        let variable_symbol_id =
                            interpreter.intern_symbol_id(provided_name);

                        interpreter.define_variable(
                            execution_environment_id,
                            variable_symbol_id,
                            Value::Boolean(false),
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}

mod define_rest_argument {
    use super::*;

    pub fn define_rest_argument<'a, I: Iterator<Item = &'a Value>>(
        interpreter: &mut Interpreter,
        execution_environment_id: EnvironmentId,
        arguments: &FunctionArguments,
        values: &mut I,
    ) -> Result<(), Error> {
        if let Some(rest_argument_name) = arguments.get_rest_argument() {
            let variable_symbol_id =
                interpreter.intern_symbol_id(rest_argument_name);

            let rest_values = values.map(|item| *item).collect();
            let rest_values_cons = interpreter.vec_to_list(rest_values);

            interpreter.define_variable(
                execution_environment_id,
                variable_symbol_id,
                rest_values_cons,
            )?;

            return Ok(());
        }

        Ok(())
    }
}

mod define_keyword_arguments {
    use super::*;

    pub fn define_keyword_arguments<'a, I: Iterator<Item = &'a Value>>(
        interpreter: &mut Interpreter,
        execution_environment_id: EnvironmentId,
        arguments: &FunctionArguments,
        values: &mut I,
    ) -> Result<(), Error> {
        if arguments.get_key_arguments().len() == 0 {
            return Ok(());
        }

        // initial set to placeholder
        for key_argument in arguments.get_key_arguments() {
            let variable_symbol_id =
                interpreter.intern_symbol_id(key_argument.get_name());
            let value = interpreter.get_exclusive_nil_value();

            interpreter.define_variable(
                execution_environment_id,
                variable_symbol_id,
                value,
            )?;
        }

        // try set with provided arguments
        while let Some(keyword_value) = values.next() {
            let keyword_id =
                crate::library::read_as_keyword_id(*keyword_value)?;

            let keyword_name =
                interpreter.get_keyword(keyword_id)?.get_name().clone();

            let variable_symbol_id =
                interpreter.intern_symbol_id(&keyword_name);

            let value = values.next().ok_or_else(|| {
                Error::generic_execution_error(
                    "Expected keyword argument value.",
                )
            })?;

            interpreter.set_environment_variable(
                execution_environment_id,
                variable_symbol_id,
                *value,
            )?;
        }

        // fill arguments that have placeholder value
        for key_argument in arguments.get_key_arguments() {
            let variable_symbol_id =
                interpreter.intern_symbol_id(key_argument.get_name());

            let looked_up_variable = interpreter
                .lookup_variable(execution_environment_id, variable_symbol_id)?
                .ok_or_else(|| {
                    Error::generic_execution_error("Cannot find variable.")
                })?;

            // check if placeholder
            let were_set =
                looked_up_variable != interpreter.get_exclusive_nil_value();

            if !were_set {
                let value =
                    if let Some(default_value) = key_argument.get_default() {
                        evaluate_value(
                            interpreter,
                            execution_environment_id,
                            default_value,
                        )?
                    } else {
                        interpreter.intern_nil_symbol_value()
                    };

                interpreter.set_environment_variable(
                    execution_environment_id,
                    variable_symbol_id,
                    value,
                )?;
            }

            if let Some(provided_name) = key_argument.get_provided() {
                let variable_symbol_id =
                    interpreter.intern_symbol_id(provided_name);
                let value = Value::Boolean(were_set);

                interpreter.define_variable(
                    execution_environment_id,
                    variable_symbol_id,
                    value,
                )?;
            }
        }

        Ok(())
    }
}

pub fn define_environment_variables(
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
    )?;

    define_optional_arguments::define_optional_arguments(
        interpreter,
        execution_environment_id,
        arguments,
        &mut iterator,
    )?;

    define_rest_argument::define_rest_argument(
        interpreter,
        execution_environment_id,
        arguments,
        &mut iterator,
    )?;

    define_keyword_arguments::define_keyword_arguments(
        interpreter,
        execution_environment_id,
        arguments,
        &mut iterator,
    )?;

    if iterator.peek().is_some() {
        return Error::generic_execution_error(
            "Function was called with too many arguments.",
        )
        .into();
    }

    Ok(())
}
