use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn car(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Built-in function `car' must take exactly one argument."
        ));
    }

    let mut values = values;

    match values.remove(0) {
        Value::Cons(cons) => Ok(cons.get_car().clone()),
        _ => return Err(Error::invalid_argument(
            interpreter,
            ""
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::lib::testing_helpers::{
        for_value_pairs_evaluated_ifbsyk
    };

    #[test]
    fn returns_car_of_cons() {
        for_value_pairs_evaluated_ifbsyk(
            |interpreter, string, value| {
                let code = &format!("(car (cons {} 1))", string);
                let result = interpreter.execute(code).unwrap();

                assert_eq!(value, result)
            }
        )
    }

    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(car)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(car (cons 1 2) 3)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    #[test]
    fn returns_invalid_argument_error_when_called_with_a_value_that_is_not_cons() {
        for_value_pairs_evaluated_ifbsyk(
            |interpreter, string,_value| {
                let code = &format!("(car {})", string);
                let result = interpreter.execute(code);

                assertion::assert_invalid_argument_error(&result);
            }
        )
    }
}
