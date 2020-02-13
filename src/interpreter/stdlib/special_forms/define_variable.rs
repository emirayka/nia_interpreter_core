use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn define_variable(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() < 1 || values.len() > 2 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `define-variable' must be used with one or two forms."
        );
    }

    let first_argument = values.remove(0);
    let second_argument = if values.len() > 0 {
        Some(values.remove(0))
    } else {
        None
    };

    let variable_name = match first_argument {
        Value::Symbol(symbol) => symbol,
        _ => return interpreter.make_invalid_argument_error(
            "First form of `define-variable' must be a symbol."
        )
    };

    let evaluated_value = match second_argument {
        Some(value) => interpreter.evaluate_value(environment, &value),
        None => Ok(interpreter.intern_nil())
    };

    let result = match evaluated_value {
        Ok(value) => value,
        Err(error) => return interpreter.make_generic_execution_error_caused(
            "Cannot evaluate the second form of define-variable.",
            error
        )
    };

    match interpreter.define_variable(
        interpreter.get_root_environment(),
        &variable_name,
        result
    ) {
        Ok(()) => Ok(Value::Boolean(true)),
        Err(error) => interpreter.make_generic_execution_error_caused(
            &format!("Cannot define variable: {}.", variable_name.get_name()),
            error
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn defines_variable_with_evaluation_result_of_the_second_form_when_two_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable test 2)").unwrap();
        let name = interpreter.intern_symbol("test");

        assert!(interpreter.has_variable(
            interpreter.get_root_environment(),
            &name));
        assert_eq!(
            Value::Integer(2),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                &name
            ).unwrap());
    }

    #[test]
    fn defines_variable_with_nil_when_one_form_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-variable test)").unwrap();
        let name = interpreter.intern_symbol("test");

        assert!(interpreter.has_variable(
            interpreter.get_root_environment(),
            &name));
        assert_eq!(
            interpreter.intern_nil(),
            interpreter.lookup_variable(
                interpreter.get_root_environment(),
                &name
            ).unwrap());
    }

    #[test]
    fn returns_err_when_incorrect_count_of_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(define-variable)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(define-variable test 2 kek)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_err_when_an_incorrect_form_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(define-variable 3 2)");
        assertion::assert_invalid_argument_error(&result);
    }
}
