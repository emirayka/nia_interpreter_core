use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::cons::Cons;

pub fn list(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    Ok(Cons::from_vec(interpreter, values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::lib::testing_helpers::{for_value_pairs_evaluated_ifbsyk, for_meta_value_pairs_evaluated_ifbsyk};

    #[test]
    fn returns_nil_when_was_called_with_zero_arguments() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(list)").unwrap();

        assertion::assert_is_nil(result);
    }

    #[test]
    fn returns_a_list_of_one_value_when_was_called_with_one_argument() {
        for_value_pairs_evaluated_ifbsyk(
            |interpreter, string, value| {
                let expected = Value::Cons(Cons::new(
                    value,
                    interpreter.intern_nil()
                ));
                let result = interpreter.execute(&format!("(list {})", string)).unwrap();

                assert_eq!(expected, result);
            }
        );
    }

    #[test]
    fn returns_a_list_of_two_values_when_was_called_with_two_arguments() {
        for_meta_value_pairs_evaluated_ifbsyk(
            |interpreter, str1, val1, str2, val2| {
                let code = &format!("(list {} {})", str1, str2);
                let result = interpreter.execute(code).unwrap();

                let expected = Value::Cons(Cons::new(
                    val1,
                    Value::Cons(Cons::new(
                        val2,
                        interpreter.intern_nil()
                    ))
                ));

                assert_eq!(expected, result);
            }
        );
    }
}
