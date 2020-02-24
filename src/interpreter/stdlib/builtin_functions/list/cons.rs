use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn cons(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `cons' must take exactly two arguments."
        ).into_result();
    }

    let mut values = values;

    Ok(interpreter.make_cons_value(
        values.remove(0),
        values.remove(0)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::lib::testing_helpers::{
        for_meta_value_pairs_evaluated_ifbsyko
    };

    // todo: ensure this test is fine
    #[test]
    fn returns_a_cons_cell() {
        for_meta_value_pairs_evaluated_ifbsyko(
            |interpreter, str1, val1, str2, val2| {
                let code = &format!("(cons {} {})", str1, str2);
                let result = interpreter.execute(code).unwrap();

                let expected = interpreter.make_cons_value(
                    val1,
                    val2
                );

                assertion::assert_deep_equal(
                    interpreter,
                    expected,
                    result
                );
            }
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(cons)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(cons 1)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(cons 1 2 3)");
        assertion::assert_invalid_argument_count_error(&result);
    }
}
