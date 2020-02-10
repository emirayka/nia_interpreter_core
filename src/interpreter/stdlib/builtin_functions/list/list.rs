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

    #[test]
    fn returns_nil_when_was_called_with_zero_arguments() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(list)").unwrap();

        assertion::assert_is_nil(result);
    }

    #[test]
    fn returns_a_list_of_one_value_when_was_called_with_one_argument() {
        let mut interpreter = Interpreter::new();

        let values = vec!(
            ("1", Value::Integer(1)),
            ("1.1", Value::Float(1.1)),
            ("#t", Value::Boolean(true)),
            ("#f", Value::Boolean(false)),
        );

        for (str, value) in values {
            let expected = Value::Cons(Cons::new(
                value,
                interpreter.intern_nil()
            ));
            let result = interpreter.execute(&format!("(list {})", str)).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn returns_a_list_of_two_values_when_was_called_with_two_arguments() {
        let mut interpreter = Interpreter::new();

        let values = vec!(
            ("1", Value::Integer(1)),
            ("1.1", Value::Float(1.1)),
            ("#t", Value::Boolean(true)),
            ("#f", Value::Boolean(false)),
        );

        for (str1, value1) in &values {
            for (str2, value2) in &values {
                let code = &format!("(list {} {})", str1, str2);
                let result = interpreter.execute(code).unwrap();
                let expected = Value::Cons(Cons::new(
                    value1.clone(),
                    Value::Cons(Cons::new(
                        value2.clone(),
                        interpreter.intern_nil()
                    ))
                ));

                assert_eq!(expected, result);
            }
        }
    }
}
