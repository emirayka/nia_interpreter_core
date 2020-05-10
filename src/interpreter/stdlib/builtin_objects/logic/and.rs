use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn and(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `logic:and' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let v1 = library::read_as_bool(values.remove(0))?;

    let v2 = library::read_as_bool(values.remove(0))?;

    match (v1, v2) {
        (true, true) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_correct_and_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(logic:and #f #f)", "#f"),
            ("(logic:and #f #t)", "#f"),
            ("(logic:and #t #f)", "#f"),
            ("(logic:and #t #t)", "#t"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_invalid_argument_count_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(logic:and)", "(logic:and #t)", "(logic:and #t #t #t)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(logic:and 1 #t)",
            "(logic:and 1.1 #t)",
            "(logic:and 'symbol #t)",
            "(logic:and \"string\" #t)",
            "(logic:and :keyword #t)",
            "(logic:and '(s-expression) #t)",
            "(logic:and {} #t)",
            "(logic:and (function (lambda () 1)) #t)",
            "(logic:and #t 1)",
            "(logic:and #t 1.1)",
            "(logic:and #t 'symbol)",
            "(logic:and #t \"string\")",
            "(logic:and #t :keyword)",
            "(logic:and #t '(s-expression))",
            "(logic:and #t {})",
            "(logic:and #t (function (lambda () 1)))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
