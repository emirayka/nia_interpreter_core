use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn define_function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() < 1 || values.len() > 2 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Special form `define-function' must be used with one or two forms."
        ));
    }

    let first_argument = values.remove(0);
    let second_argument = if values.len() > 0 {
        Some(values.remove(0))
    } else {
        None
    };

    let function_name = match first_argument {
        Value::Symbol(symbol) => symbol,
        _ => return Err(Error::invalid_argument(
            interpreter,
            "First form of `define-function' must be a symbol."
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
            "Cannot evaluate the second form of define-function.",
            error
        ))
    };

    match interpreter.define_function(
        interpreter.get_root_environment(),
        &function_name,
        result
    ) {
        Ok(()) => Ok(Value::Boolean(true)),
        Err(error) => Err(Error::generic_execution_error_caused(
            interpreter,
            &format!("Cannot define function: {}.", function_name.get_name()),
            error
        ))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn defines_function_with_evaluation_result_of_the_second_form_when_two_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test 2)").unwrap();
        let name = interpreter.intern_symbol("test");

        assert!(interpreter.has_function(
            interpreter.get_root_environment(),
            &name));
        assert_eq!(
            &Value::Integer(2),
            interpreter.lookup_function(
                interpreter.get_root_environment(),
                &name
            ).unwrap());
    }

    #[test]
    fn defines_function_with_nil_when_one_form_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test)").unwrap();
        let name = interpreter.intern_symbol("test");

        assert!(interpreter.has_function(
            interpreter.get_root_environment(),
            &name));
        assert_eq!(
            &interpreter.intern_nil(),
            interpreter.lookup_function(
                interpreter.get_root_environment(),
                &name
            ).unwrap());
    }

    // todo: move to higher module test
    #[test]
    fn defines_function_that_can_be_executed() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test (function (lambda (a b) b)))").unwrap();
        let result = interpreter.execute("(test 2 3)");

        assert_eq!(Value::Integer(3), result.unwrap());
    }

    // todo: move to higher module test
    #[test]
    fn possible_to_make_a_closure() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test (function (lambda (a) (function (lambda () a)))))").unwrap();
        interpreter.execute("(define-function test2 (test 2))").unwrap();

        assert_eq!(Value::Integer(2), interpreter.execute("(test2)").unwrap());
    }

    #[test]
    fn returns_err_when_incorrect_count_of_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(define-function)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(define-function test 2 kek)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_err_when_an_incorrect_form_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(define-function 3 2)");
        assertion::assert_invalid_argument_error(&result);
    }
}
