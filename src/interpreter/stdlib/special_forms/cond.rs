use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::cons::Cons;
use crate::interpreter::function::Function;
use crate::interpreter::function::special_form_function::SpecialFormFunction;

fn execute_part(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    part: &Cons
) -> Result<Option<Value>, Error> {
    let part_predicate = part.get_car();
    let part_action = match part.get_cdr() {
        Value::Cons(cons) => cons.get_car(),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "Invalid action part."
        ))
    };

    let predicate_result = interpreter.execute_value(environment, part_predicate);

    match predicate_result {
        Ok(value) => match value {
            Value::Boolean(true) => {
                let action_result = interpreter.execute_value(environment, part_action);

                match action_result {
                    Ok(result) => Ok(Some(result)),
                    Err(error) => Err(Error::generic_execution_error_caused(
                        interpreter,
                        "Cannot evaluate the action part.",
                        error
                    ))
                }
            },
            Value::Boolean(false) => Ok(None),
            _ => Err(Error::invalid_argument(
                interpreter,
                "Predicate must evaluate to boolean value."
            ))
        },
        Err(error) => Err(error)
    }
}

fn cond(interpreter: &mut Interpreter, environment: EnvironmentId, values: Vec<Value>) -> Result<Value, Error> {
    let mut result = Ok(interpreter.intern_nil());

    for value in values {
        if let Value::Cons(part) = &value {
            match execute_part(interpreter, environment, part) {
                Ok(Some(value)) => {
                    result = Ok(value);
                    break;
                },
                Err(error) => {
                    result = Err(Error::generic_execution_error_caused(
                        interpreter,
                        "Cannot execute special form `cond' clause.",
                        error
                    ));
                    break;
                },
                _ => ()
            }
        } else {
            result = Err(Error::invalid_argument(
                interpreter,
                "Invalid usage of special form `cond'."
            ));
            break;
        }
    }

    result
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let name = interpreter.intern_symbol("cond");

    let result = interpreter.define_function(
        interpreter.get_root_environment(),
        &name,
        Value::Function(Function::SpecialForm(SpecialFormFunction::new(cond)))
    );

    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;

    #[test]
    fn test_cond_works_correctly() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Integer(1), interpreter.execute("(cond (#t 1) (#t 2) (#t 3))").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(cond (#f 1) (#t 2) (#t 3))").unwrap());
        assert_eq!(Value::Integer(3), interpreter.execute("(cond (#f 1) (#f 2) (#t 3))").unwrap());
        assert_eq!(interpreter.intern_nil(), interpreter.execute("(cond (#f 1) (#f 2) (#f 3))").unwrap());
        assert_eq!(interpreter.intern_nil(), interpreter.execute("(cond)").unwrap());
    }

    #[test]
    fn test_returns_err_when_invalid_clause_was_provided_to_cond() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        let invalid_forms = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "symbol",
            "\"string\"",
            ":keyword",
        );

        for invalid_form in invalid_forms {
            let result = interpreter.execute(&format!("(cond {})", invalid_form));

            assertion::assert_argument_error(&result);
            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn test_returns_err_when_invalid_clause_was_provided_to_cond_2() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        let invalid_forms = vec!(
            "(1)",
            "(1.1)",
            "(#t)",
            "(#f)",
            "(symbol)",
            "(\"string\")",
            "(:keyword)",
        );

        for invalid_form in invalid_forms {
            let result = interpreter.execute(&format!("(cond {})", invalid_form));

            assertion::assert_argument_error(&result);
            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn test_returns_err_when_invalid_predicate_was_provided_to_cond() {
        let mut interpreter = Interpreter::raw();
        infect(&mut interpreter).unwrap();

        let name = interpreter.intern_symbol("test");
        interpreter.define_variable(
            interpreter.get_root_environment(),
            &name,
            Value::Integer(1)
        );


        let invalid_forms = vec!(
            "(cond (1 1))",
            "(cond (1.1 1))",
            "(cond (test1))",
            "(cond (\"string\" 1))",
            "(cond (:keyword 1))",
            "(cond ((cond (#t 1)) 1))",
        );

        for invalid_form in invalid_forms {
            let result = interpreter.execute(invalid_form);

            assertion::assert_argument_error(&result);
            assertion::assert_invalid_argument_error(&result);
        }
    }
}