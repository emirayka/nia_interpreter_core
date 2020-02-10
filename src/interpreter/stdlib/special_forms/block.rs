use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

// todo: check and delete
//fn make_cons(interpreter: &mut Interpreter, values: Vec<Value>) -> Value {
//    if values.len() == 0 {
//        return interpreter.intern_nil();
//    }
//
//    let nil = interpreter.intern_nil();
//    let mut values = values;
//
//    let mut base_cons = interpreter.make_cons_value(
//        values.remove(0),
//        nil.clone()
//    );
//
//    let mut cons = &mut base_cons;
//
//    for value in values {
//        let new_cons = interpreter.make_cons_value(
//            value,
//            nil.clone()
//        );
//
//        cons = if let Value::Cons(cons_id) = cons {
//            cons_id.set_cdr(new_cons);
//            cons_id.get_cdr_mut()
//        } else {
//            unreachable!();
//        };
//    }
//
//    base_cons
//}

pub fn block(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let values = values;
    let mut results = Vec::new();

    for value in values {
        let result = interpreter.execute_value(environment, &value)?;

        results.push(result);
    }

    Ok(interpreter.cons_from_vec(results))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_list_of_execution_results() {
        let mut interpreter = Interpreter::new();
        let nil = interpreter.intern_nil();

        assert_eq!(interpreter.intern_nil(), interpreter.execute("(block)").unwrap());

        let expected = interpreter.make_cons_value(
            Value::Integer(1),
            nil.clone()
        );
        let result = interpreter.execute("(block 1)").unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            &expected,
            &result
        );

        let cdr = interpreter.make_cons_value(
            Value::Integer(2),
            nil.clone()
        );

        let expected = interpreter.make_cons_value(
            Value::Integer(1),
            cdr,
        );

        let result = interpreter.execute("(block 1 2)").unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            &expected,
            &result
        );

        let cdr = interpreter.make_cons_value(
            Value::Integer(3),
            nil.clone()
        );

        let cdr = interpreter.make_cons_value(
            Value::Integer(2),
            cdr
        );

        let expected = interpreter.make_cons_value(
            Value::Integer(1),
            cdr
        );

        let result = interpreter.execute("(block 1 2 3)").unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            &expected,
            &result
        );
    }
}
