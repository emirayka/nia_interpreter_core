use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::function::special_form_function::SpecialFormFunction;

fn define_function(
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

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let name = interpreter.intern_symbol("define-function");

    let result = interpreter.define_function(
        interpreter.get_root_environment(),
        &name,
        Value::Function(Function::SpecialForm(SpecialFormFunction::new(define_function)))
    );

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::*;

    macro_rules! assert_argument_error {
        ($e:expr) => {
            assert!(
                match $e.err().unwrap().get_total_cause().get_error_kind() {
                    ErrorKind::Argument(_) => true,
                    _ => false
                }
            );
        }
    }

    macro_rules! assert_invalid_argument_error {
        ($e:expr) => {
            assert!(
                match $e.err().unwrap().get_total_cause().get_error_kind() {
                    ErrorKind::Argument(ArgumentErrorKind::InvalidArgument) => true,
                    _ => false
                }
            );

            assert_eq!(SYMBOL_NAME_INVALID_ARGUMENT, $e.err().unwrap().get_symbol().unwrap().get_name());
        }
    }

    macro_rules! assert_invalid_argument_count_error {
        ($e:expr) => {
            assert!(
                match $e.err().unwrap().get_total_cause().get_error_kind() {
                    ErrorKind::Argument(ArgumentErrorKind::InvalidArgumentCount) => true,
                    _ => false
                }
            );

            assert_eq!(SYMBOL_NAME_INVALID_ARGUMENT_COUNT, $e.err().unwrap().get_symbol().unwrap().get_name());
        }
    }

    #[test]
    fn test_defines_function_with_evaluation_result_of_the_second_form_when_two_forms_were_provided() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        interpreter.execute("(define-function test 2)").unwrap();
        let name = interpreter.intern_symbol("test");

        assert_eq!(
            &Value::Integer(2),
            interpreter.lookup_function(
                interpreter.get_root_environment(),
                &name
            ).unwrap());
    }

    #[test]
    fn test_defines_function_with_nil_when_one_form_were_provided() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        interpreter.execute("(define-function test)").unwrap();
        let name = interpreter.intern_symbol("test");

        assert_eq!(
            &interpreter.intern_nil(),
            interpreter.lookup_function(
                interpreter.get_root_environment(),
                &name
            ).unwrap());
    }

    #[test]
    fn test_returns_err_when_incorrect_count_of_forms_were_provided() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        // todo: fix bug that will be in future
        // it should get the final cause of an error, instead of the top-most
        let result = interpreter.execute("(define-function)");
        assert_argument_error!(result.as_ref());
        assert_invalid_argument_count_error!(result.as_ref());

        let result = interpreter.execute("(define-function test 2 kek)");
        assert_argument_error!(result.as_ref());
        assert_invalid_argument_count_error!(result.as_ref());

        let result = interpreter.execute("(define-function 3 2)");
        assert_argument_error!(result.as_ref());
        assert_invalid_argument_error!(result.as_ref());
    }
}
