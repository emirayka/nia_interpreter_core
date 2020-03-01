use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::cons::cons_arena::ConsId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::lib;

pub fn iterate_through_list(
    interpreter: &mut Interpreter,
    cons_id: ConsId,
    mut closure: impl FnMut(&mut Interpreter, Value) -> Result<bool, Error>
) -> Result<(), Error> {
    let mut car = interpreter.get_car(cons_id)?;
    let mut cdr = interpreter.get_cdr(cons_id)?;

    loop {
        let should_continue = closure(interpreter, car)?;

        if !should_continue {
            break;
        }

        match cdr {
            Value::Symbol(symbol_id) => {
                let symbol = interpreter.get_symbol(symbol_id)?;

                if symbol.is_nil() {
                    break;
                } else {
                    return interpreter.make_generic_execution_error(
                        "Invalid list"
                    ).into_result()
                }
            },
            Value::Cons(cons_id) => {
                car = interpreter.get_car(cons_id)?;
                cdr = interpreter.get_cdr(cons_id)?;
            },
            _ => return interpreter.make_generic_execution_error(
                "Invalid list"
            ).into_result()
        };
    }

    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn executes_once_for_each_item_in_list() {
        let mut result = 0;

        let closure = |interpreter: &mut Interpreter, value: Value| {
            if let Value::Integer(int) = value {
                result += int;

                Ok(true)
            } else {
                unreachable!();
            }
        };

        let mut interpreter = Interpreter::new();
        let value = interpreter.execute("(list 1 2 3 4)").unwrap();
        let list = lib::read_as_cons(
            &interpreter,
            value
        ).unwrap();

        iterate_through_list(&mut interpreter, list, closure).unwrap();

        assert_eq!(10, result);
    }

    #[test]
    fn able_to_determine_if_need_to_continue() {
        let mut result = 0;

        let closure = |interpreter: &mut Interpreter, value: Value| {
            if let Value::Integer(int) = value {
                result += int;

                Ok(int % 2 != 0)
            } else {
                unreachable!();
            }
        };

        let mut interpreter = Interpreter::new();
        let value = interpreter.execute("(list 1 2 3 4)").unwrap();
        let list = lib::read_as_cons(
            &interpreter,
            value
        ).unwrap();

        iterate_through_list(&mut interpreter, list, closure).unwrap();

        assert_eq!(3, result);
    }
}
