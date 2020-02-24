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
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `car' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let car = match values.remove(0) {
        Value::Cons(cons_id) => interpreter.get_car(cons_id)
            .map_err(|err| interpreter.make_generic_execution_error_caused(
                "",
                err
            ))?,
        _ => return interpreter.make_invalid_argument_error(
            ""
        ).into_result()
    };

    Ok(car)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::lib::testing_helpers::{
        for_value_pairs_evaluated_ifbsyko
    };

    #[test]
    fn returns_car_of_cons() {
        for_value_pairs_evaluated_ifbsyko(
            |interpreter, string, value| {
                let code = &format!("(car (cons {} 1))", string);

                let expected = value;
                let result = interpreter.execute(code).unwrap();

                assertion::assert_deep_equal(interpreter, expected, result);
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
        for_value_pairs_evaluated_ifbsyko(
            |interpreter, string,_value| {
                let code = &format!("(car {})", string);
                let result = interpreter.execute(code);

                assertion::assert_invalid_argument_error(&result);
            }
        )
    }
}
