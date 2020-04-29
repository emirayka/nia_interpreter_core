use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::ConsId;

fn execute_part(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    part_cons_id: ConsId
) -> Result<Option<Value>, Error> {
    let part_action = interpreter.get_cadr(part_cons_id)
        .map_err(|_| Error::invalid_argument_error("Invalid action part."))?;

    let part_predicate = interpreter.get_car(part_cons_id)
        .map_err(|err| Error::generic_execution_error_caused(
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
                .map_err(|err| Error::generic_execution_error_caused(
                    "Cannot evaluate the action part.",
                    err
                ))?;

            Ok(Some(action_result))
        },
        Value::Boolean(false) => Ok(None),
        _ => Error::invalid_argument_error(
            "Predicate must evaluate to boolean value."
        ).into()
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
                    result = Error::generic_execution_error_caused(
                        "Cannot execute special form `cond' clause.",
                        error
                    ).into();
                    break;
                },
                _ => ()
            }
        } else {
            result = Error::invalid_argument_error(
                "Invalid usage of special form `cond'."
            ).into();
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn cond_works_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("1", "(cond (#t 1) (#t 2) (#t 3))"),
            ("2", "(cond (#f 1) (#t 2) (#t 3))"),
            ("3", "(cond (#f 1) (#f 2) (#t 3))"),
            ("nil", "(cond (#f 1) (#f 2) (#f 3))"),
            ("nil", "(cond)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_err_when_invalid_clause_was_provided_to_cond() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(cond 1)",
            "(cond 1.1)",
            "(cond #t)",
            "(cond #f)",
            "(cond symbol)",
            "(cond \"string\")",
            "(cond :keyword)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs
        )
    }

    #[test]
    fn returns_err_when_invalid_clause_was_provided_to_cond_2() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(cond 1)",
            "(cond 1.1)",
            "(cond #t)",
            "(cond #f)",
            "(cond symbol)",
            "(cond \"string\")",
            "(cond :keyword)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_predicate_was_provided_to_cond() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(cond (1 1))",
            "(cond (1.1 1))",
            "(cond (test1))",
            "(cond (\"string\" 1))",
            "(cond (:keyword 1))",
            "(cond ((cond (#t 1)) 1))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs
        );
    }
}