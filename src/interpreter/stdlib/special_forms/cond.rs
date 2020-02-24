use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::cons::cons_arena::ConsId;

fn execute_part(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    part_cons_id: ConsId
) -> Result<Option<Value>, Error> {
    let part_action = interpreter.get_cadr(part_cons_id)
        .map_err(|_| interpreter.make_invalid_argument_error("Invalid action part."))?;

    let part_predicate = interpreter.get_car(part_cons_id)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    let predicate_result = interpreter.execute_value(
        environment,
        part_predicate
    )?;

    match predicate_result {
        Value::Boolean(true) => {
            let action_result = interpreter.execute_value(environment, part_action)
                .map_err(|err| interpreter.make_generic_execution_error_caused(
                    "Cannot evaluate the action part.",
                    err
                ))?;

            Ok(Some(action_result))
        },
        Value::Boolean(false) => Ok(None),
        _ => interpreter.make_invalid_argument_error(
            "Predicate must evaluate to boolean value."
        ).into_result()
    }
}

pub fn cond(interpreter: &mut Interpreter, environment: EnvironmentId, values: Vec<Value>) -> Result<Value, Error> {
    let mut result = Ok(interpreter.intern_nil_symbol_value());

    for value in values {
        if let Value::Cons(part) = value {
            match execute_part(interpreter, environment, part) {
                Ok(Some(value)) => {
                    result = Ok(value);
                    break;
                },
                Err(error) => {
                    result = interpreter.make_generic_execution_error_caused(
                        "Cannot execute special form `cond' clause.",
                        error
                    ).into_result();
                    break;
                },
                _ => ()
            }
        } else {
            result = interpreter.make_invalid_argument_error(
                "Invalid usage of special form `cond'."
            ).into_result();
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    // todo: ensure this test is fine
    #[test]
    fn cond_works_correctly() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(1), interpreter.execute("(cond (#t 1) (#t 2) (#t 3))").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(cond (#f 1) (#t 2) (#t 3))").unwrap());
        assert_eq!(Value::Integer(3), interpreter.execute("(cond (#f 1) (#f 2) (#t 3))").unwrap());
        assert_eq!(interpreter.intern_nil_symbol_value(), interpreter.execute("(cond (#f 1) (#f 2) (#f 3))").unwrap());
        assert_eq!(interpreter.intern_nil_symbol_value(), interpreter.execute("(cond)").unwrap());
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_invalid_clause_was_provided_to_cond() {
        let mut interpreter = Interpreter::new();

        let invalid_forms = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "symbol",
            "\"string\"",
            ":keyword",
        );

        for invalid_form in invalid_forms {
            let result = interpreter.execute(&format!("(cond {})", invalid_form));

            assertion::assert_invalid_argument_error(&result);
        }
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_invalid_clause_was_provided_to_cond_2() {
        let mut interpreter = Interpreter::new();

        let invalid_forms = vec!(
            "(1)",
            "(1.1)",
            "(#t)",
            "(#f)",
            "(symbol)",
            "(\"string\")",
            "(:keyword)",
        );

        for invalid_form in invalid_forms {
            let result = interpreter.execute(&format!("(cond {})", invalid_form));

            assertion::assert_invalid_argument_error(&result);
        }
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_invalid_argument_error_when_invalid_predicate_was_provided_to_cond() {
        let mut interpreter = Interpreter::new();

        let name = interpreter.intern("test");
        interpreter.define_variable(
            interpreter.get_root_environment(),
            name,
            Value::Integer(1)
        ).unwrap();

        let invalid_forms = vec!(
            "(cond (1 1))",
            "(cond (1.1 1))",
            "(cond (test1))",
            "(cond (\"string\" 1))",
            "(cond (:keyword 1))",
            "(cond ((cond (#t 1)) 1))",
        );

        for invalid_form in invalid_forms {
            let result = interpreter.execute(invalid_form);

            assertion::assert_invalid_argument_error(&result);
        }
    }
}