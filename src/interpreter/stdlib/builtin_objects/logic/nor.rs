use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn nor(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `logic:nor' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let v1 = library::read_as_bool(values.remove(0))?;

    let v2 = library::read_as_bool(values.remove(0))?;

    match (v1, v2) {
        (false, false) => Ok(Value::Boolean(true)),
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
    fn returns_correct_nor_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(logic:nor #f #f)", "#t"),
            ("(logic:nor #f #t)", "#f"),
            ("(logic:nor #t #f)", "#f"),
            ("(logic:nor #t #t)", "#f"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_invalid_argument_count_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(logic:nor)", "(logic:nor #t)", "(logic:nor #t #t #t)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(logic:nor 1 #t)",
            "(logic:nor 1.1 #t)",
            "(logic:nor 'symbol #t)",
            "(logic:nor \"string\" #t)",
            "(logic:nor :keyword #t)",
            "(logic:nor '(s-expression) #t)",
            "(logic:nor {} #t)",
            "(logic:nor (function (lambda () 1)) #t)",
            "(logic:nor #t 1)",
            "(logic:nor #t 1.1)",
            "(logic:nor #t 'symbol)",
            "(logic:nor #t \"string\")",
            "(logic:nor #t :keyword)",
            "(logic:nor #t '(s-expression))",
            "(logic:nor #t {})",
            "(logic:nor #t (function (lambda () 1)))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
