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
    let part_value = match part.get_cdr() {
        Value::Cons(cons) => cons.get_car(),
        _ => return Err(Error::empty())
    };

    let predicate_result = interpreter.execute_value(environment, part_predicate);

    match predicate_result {
        Ok(value) => match value {
            Value::Boolean(true) => {
                let value_result = interpreter.execute_value(environment, part_value);

                match value_result {
                    Ok(result) => Ok(Some(result)),
                    Err(error) => Err(error)
                }
            },
            Value::Boolean(false) => Ok(None),
            _ => Err(Error::empty())
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
                    result = Err(error);
                    break;
                },
                _ => ()
            }
        } else {
            result = Err(Error::empty());
            break;
        }
    }

    result
}

pub fn infect_cond(interpreter: &mut Interpreter) -> Result<(), Error> {
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

    #[test]
    fn test_works_correctly() {
        let mut interpreter = Interpreter::new();
        infect_cond(&mut interpreter);

        assert_eq!(Value::Integer(1), interpreter.execute("(cond (#t 1) (#t 2) (#t 3))").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(cond (#f 1) (#t 2) (#t 3))").unwrap());
        assert_eq!(Value::Integer(3), interpreter.execute("(cond (#f 1) (#f 2) (#t 3))").unwrap());
        assert_eq!(interpreter.intern_nil(), interpreter.execute("(cond (#f 1) (#f 2) (#f 3))").unwrap());
        assert_eq!(interpreter.intern_nil(), interpreter.execute("(cond)").unwrap());
    }
}