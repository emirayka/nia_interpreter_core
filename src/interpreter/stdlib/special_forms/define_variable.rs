use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::library;

pub fn define_variable(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() < 1 || values.len() > 3 {
        return Error::invalid_argument_count_error(
            "Special form `define-variable' must be used with one or two forms."
        ).into();
    }

    let first_argument = values.remove(0);
    let second_argument = if values.len() > 0 {
        Some(values.remove(0))
    } else {
        None
    };

    let need_to_be_const = if values.len() > 0 {
        let result = library::read_as_keyword(
            interpreter,
            values.remove(0),
        )?.is_const();

        if !result {
            return Error::invalid_argument_error(
                "Third argument of special form `define-variable' must be a keyword `:const'."
            ).into();
        }

        result
    } else {
        false
    };

    let variable_symbol_id = match first_argument {
        Value::Symbol(symbol) => symbol,
        _ => return Error::invalid_argument_error(
            "First form of `define-variable' must be a symbol."
        ).into()
    };

    library::check_if_symbol_assignable(interpreter, variable_symbol_id)?;

    let evaluated_value = match second_argument {
        Some(value) => {
            interpreter.evaluate_value(environment, value)
                .map_err(|err| Error::generic_execution_error_caused(
                    "Cannot evaluate the second form of define-variable.",
                    err,
                ))?
        }
        None => interpreter.intern_nil_symbol_value()
    };

    let result = if need_to_be_const {
        interpreter.define_const_variable(
            interpreter.get_root_environment(),
            variable_symbol_id,
            evaluated_value,
        )
    } else {
        interpreter.define_variable(
            interpreter.get_root_environment(),
            variable_symbol_id,
            evaluated_value,
        )
    };

    result.map_err(|err| {
        let symbol_name = match interpreter.get_symbol_name(variable_symbol_id) {
            Ok(symbol_name) => symbol_name,
            _ => return Error::generic_execution_error("")
        };

        let variable_name = &format!(
            "Cannot define variable: {}.",
            symbol_name
        );

        Error::generic_execution_error_caused(
            variable_name,
            err,
        )
    })?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::{for_special_symbols, for_constants};

    #[test]
    fn defines_variable_with_evaluation_result_of_the_second_form_when_two_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable test 2)").unwrap();
        let name = interpreter.intern("test");

        nia_assert(interpreter.has_variable(
            interpreter.get_root_environment(),
            name,
        ).unwrap());

        nia_assert_equal(
            Some(Value::Integer(2)),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                name,
            ).unwrap()
        );
    }

    #[test]
    fn defines_variable_with_nil_when_one_form_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable test)").unwrap();
        let name = interpreter.intern("test");

        nia_assert(interpreter.has_variable(
            interpreter.get_root_environment(),
            name,
        ).unwrap());

        nia_assert_equal(
            Ok(Some(interpreter.intern_nil_symbol_value())),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                name,
            ));
    }


    #[test]
    fn able_to_define_const_variable() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable test 3 :const)").unwrap();

        let result = interpreter.execute("test");
        nia_assert_equal(Value::Integer(3), result.unwrap());

        let result = interpreter.execute("(set! test 2)");
        nia_assert_is_err(&result)
    }

    #[test]
    fn returns_error_when_attempts_to_define_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(define-variable {} 2)", constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(define-variable {} 2)", special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_err_when_incorrect_count_of_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(define-variable)",
            "(define-variable test 2 :const kek)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_an_incorrect_form_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(define-variable 3 2)"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }
}
