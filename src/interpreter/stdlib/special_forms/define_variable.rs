use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::lib::_lib;

pub fn define_variable(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() < 1 || values.len() > 2 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `define-variable' must be used with one or two forms."
        ).into_result();
    }

    let first_argument = values.remove(0);
    let second_argument = if values.len() > 0 {
        Some(values.remove(0))
    } else {
        None
    };

    let variable_symbol_id = match first_argument {
        Value::Symbol(symbol) => symbol,
        _ => return interpreter.make_invalid_argument_error(
            "First form of `define-variable' must be a symbol."
        ).into_result()
    };

    _lib::check_if_symbol_assignable(interpreter, variable_symbol_id)?;

    let evaluated_value = match second_argument {
        Some(value) => {
            interpreter.evaluate_value(environment, value)
                .map_err(|err| interpreter.make_generic_execution_error_caused(
                    "Cannot evaluate the second form of define-variable.",
                    err
                ))?
        },
        None => interpreter.intern_nil_symbol_value()
    };

    interpreter.define_variable(
        interpreter.get_root_environment(),
        variable_symbol_id,
        evaluated_value
    ).map_err(|err| {
        let symbol_name = match interpreter.get_symbol_name(variable_symbol_id) {
            Ok(symbol_name) => symbol_name,
            _ => return interpreter.make_generic_execution_error("")
        };

        let variable_name = &format!(
            "Cannot define variable: {}.",
            symbol_name
        );

        interpreter.make_generic_execution_error_caused(
            variable_name,
            err
        )
    })?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::lib::testing_helpers::{for_special_symbols, for_constants};

    // todo: ensure this test is fine
    #[test]
    fn defines_variable_with_evaluation_result_of_the_second_form_when_two_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable test 2)").unwrap();
        let name = interpreter.intern("test");

        assert!(interpreter.has_variable(
            interpreter.get_root_environment(),
            name
        ).unwrap());
        assert_eq!(
            Value::Integer(2),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                name
            ).unwrap()
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn defines_variable_with_nil_when_one_form_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable test)").unwrap();
        let name = interpreter.intern("test");

        assert!(interpreter.has_variable(
            interpreter.get_root_environment(),
            name
        ).unwrap());
        assert_eq!(
            interpreter.intern_nil_symbol_value(),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                name
            ).unwrap());
    }

    // todo: ensure this test is fine
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


    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_incorrect_count_of_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(define-variable)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(define-variable test 2 kek)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_an_incorrect_form_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(define-variable 3 2)");
        assertion::assert_invalid_argument_error(&result);
    }
}
