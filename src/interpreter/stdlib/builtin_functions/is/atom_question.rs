use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn atom_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `is:atom?' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Cons(_) => false,
        _ => true
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::testing_helpers::for_value_pairs_evaluated_ifbsykcou;
    use crate::interpreter::lib::assertion;

    // todo: ensure this test is fine
    #[test]
    fn returns_true_when_an_atom_was_passed() {
        for_value_pairs_evaluated_ifbsykcou(
            |interpreter, code, value| {
                if let Value::Cons(_) = value {
                    return;
                }

                let code = format!("(is:atom? {})", code);
                let result = interpreter.execute(&code).unwrap();
                let expected = Value::Boolean(true);

                assert_eq!(expected, result);
            }
        )
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_false_when_not_an_atom_was_passed() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Boolean(false);
        let result = interpreter.execute("(is:atom? (cons 1 2))").unwrap();

        assert_eq!(expected, result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(is:atom?)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(is:atom? 1 2)");
        assertion::assert_invalid_argument_count_error(&result);
    }
}
