use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::ConsId;
use crate::interpreter::value::Value;

pub fn iterate_through_list(
    interpreter: &mut Interpreter,
    cons_id: ConsId,
    mut closure: impl FnMut(&mut Interpreter, Value) -> Result<bool, Error>,
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
                if interpreter.symbol_is_nil(symbol_id)? {
                    break;
                } else {
                    return Error::generic_execution_error("Invalid list")
                        .into();
                }
            },
            Value::Cons(cons_id) => {
                car = interpreter.get_car(cons_id)?;
                cdr = interpreter.get_cdr(cons_id)?;
            },
            _ => return Error::generic_execution_error("Invalid list").into(),
        };
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::interpreter::library;

    #[test]
    fn executes_once_for_each_item_in_list() {
        let mut result = 0;

        let closure = |_interpreter: &mut Interpreter, value: Value| {
            if let Value::Integer(int) = value {
                result += int;

                Ok(true)
            } else {
                unreachable!();
            }
        };

        let mut interpreter = Interpreter::new();
        let value = interpreter
            .execute_in_main_environment("(list 1 2 3 4)")
            .unwrap();
        let list = library::read_as_cons_id(value).unwrap();

        iterate_through_list(&mut interpreter, list, closure).unwrap();

        nia_assert_equal(10, result);
    }

    #[test]
    fn able_to_determine_if_need_to_continue() {
        let mut result = 0;

        let closure = |_interpreter: &mut Interpreter, value: Value| {
            if let Value::Integer(int) = value {
                result += int;

                Ok(int % 2 != 0)
            } else {
                unreachable!();
            }
        };

        let mut interpreter = Interpreter::new();
        let value = interpreter
            .execute_in_main_environment("(list 1 2 3 4)")
            .unwrap();
        let list = library::read_as_cons_id(value).unwrap();

        iterate_through_list(&mut interpreter, list, closure).unwrap();

        nia_assert_equal(3, result);
    }
}
