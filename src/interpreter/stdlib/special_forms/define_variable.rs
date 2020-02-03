use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::stdlib::special_forms::_lib::infect_special_form;

fn define_variable(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() < 1 || values.len() > 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Special form `define-variable' must be used with one or two forms."
        ));
    }

    let first_argument = values.remove(0);
    let second_argument = if values.len() > 0 {
        Some(values.remove(0))
    } else {
        None
    };

    let variable_name = match first_argument {
        Value::Symbol(symbol) => symbol,
        _ => return Err(Error::invalid_argument(
            interpreter,
            "First form of `define-variable' must be a symbol."
        ))
    };

    let evaluated_value = match second_argument {
        Some(value) => interpreter.evaluate_value(environment, &value),
        None => Ok(interpreter.intern_nil())
    };

    let result = match evaluated_value {
        Ok(value) => value,
        Err(error) => return Err(Error::generic_execution_error_caused(
            interpreter,
            "Cannot evaluate the second form of define-variable.",
            error
        ))
    };

    match interpreter.define_variable(
        interpreter.get_root_environment(),
        &variable_name,
        result
    ) {
        Ok(()) => Ok(Value::Boolean(true)),
        Err(error) => Err(Error::generic_execution_error_caused(
            interpreter,
            &format!("Cannot define variable: {}.", variable_name.get_name()),
            error
        ))
    }

}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_special_form(interpreter, "define-variable", define_variable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;

    #[test]
    fn test_defines_variable_with_evaluation_result_of_the_second_form_when_two_forms_were_provided() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        interpreter.execute("(define-variable test 2)").unwrap();
        let name = interpreter.intern_symbol("test");

        assert!(interpreter.has_variable(
            interpreter.get_root_environment(),
            &name));
        assert_eq!(
            &Value::Integer(2),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                &name
            ).unwrap());
    }

    #[test]
    fn test_defines_variable_with_nil_when_one_form_were_provided() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        interpreter.execute("(define-variable test)").unwrap();
        let name = interpreter.intern_symbol("test");

        assert!(interpreter.has_variable(
            interpreter.get_root_environment(),
            &name));
        assert_eq!(
            &interpreter.intern_nil(),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                &name
            ).unwrap());
    }

    #[test]
    fn test_returns_err_when_incorrect_count_of_forms_were_provided() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(define-variable)");
        assertion::assert_argument_error(&result);
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(define-variable test 2 kek)");
        assertion::assert_argument_error(&result);
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn test_returns_err_when_an_incorrect_form_were_provided() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(define-variable 3 2)");
        assertion::assert_argument_error(&result);
        assertion::assert_invalid_argument_error(&result);
    }
}
