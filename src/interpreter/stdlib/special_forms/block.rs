use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::stdlib::special_forms::_lib::infect_special_form;
use crate::interpreter::cons::Cons;

fn make_cons(interpreter: &mut Interpreter, values: Vec<Value>) -> Value {
    if values.len() == 0 {
        return interpreter.intern_nil();
    }

    let mut values = values;

    let mut base_cons = Value::Cons(Cons::new(
        values.remove(0),
        interpreter.intern_nil()
    ));

    let mut cons = &mut base_cons;

    for value in values {
        let new_cons = Value::Cons(Cons::new(
            value,
            interpreter.intern_nil()
        ));

        cons = if let Value::Cons(cons) = cons{
            cons.set_cdr(new_cons);
            cons.get_cdr_mut()
        } else {
            unreachable!();
        };
    }

    base_cons
}

fn block(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;
    let mut results = Vec::new();

    for value in values {
        let result = interpreter.execute_value(environment, &value)?;

        results.push(result);
    }

    Ok(make_cons(interpreter, results))
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_special_form(interpreter, "block", block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::cons::Cons;
    use crate::interpreter::error::assertion;

    #[test]
    fn returns_list_of_execution_results() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter);

        assert_eq!(interpreter.intern_nil(), interpreter.execute("(block)").unwrap());
        assert_eq!(
            Value::Cons(Cons::new(
                Value::Integer(1),
                interpreter.intern_nil()
            )),
            interpreter.execute("(block 1)").unwrap()
        );
        assert_eq!(
            Value::Cons(Cons::new(
                Value::Integer(1),
                Value::Cons(Cons::new(
                    Value::Integer(2),
                    interpreter.intern_nil()
                )),
            )),
            interpreter.execute("(block 1 2)").unwrap()
        );
        assert_eq!(
            Value::Cons(Cons::new(
                Value::Integer(1),
                Value::Cons(Cons::new(
                    Value::Integer(2),
                    Value::Cons(Cons::new(
                        Value::Integer(3),
                        interpreter.intern_nil()
                    )),
                )),
            )),
            interpreter.execute("(block 1 2 3)").unwrap()
        );
    }
}
