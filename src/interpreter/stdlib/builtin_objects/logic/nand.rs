use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn nand(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `logic:nand' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let v1 = library::read_as_bool(values.remove(0))?;

    let v2 = library::read_as_bool(values.remove(0))?;

    match (v1, v2) {
        (true, true) => Ok(Value::Boolean(false)),
        _ => Ok(Value::Boolean(true)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_nand_result() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(logic:nand #f #f)", "#t"),
            ("(logic:nand #f #t)", "#t"),
            ("(logic:nand #t #f)", "#t"),
            ("(logic:nand #t #t)", "#f"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_invalid_argument_count_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(logic:nand)", "(logic:nand #t)", "(logic:nand #t #t #t)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(logic:nand 1 #t)",
            "(logic:nand 1.1 #t)",
            "(logic:nand 'symbol #t)",
            "(logic:nand \"string\" #t)",
            "(logic:nand :keyword #t)",
            "(logic:nand '(s-expression) #t)",
            "(logic:nand {} #t)",
            "(logic:nand (function (lambda () 1)) #t)",
            "(logic:nand #t 1)",
            "(logic:nand #t 1.1)",
            "(logic:nand #t 'symbol)",
            "(logic:nand #t \"string\")",
            "(logic:nand #t :keyword)",
            "(logic:nand #t '(s-expression))",
            "(logic:nand #t {})",
            "(logic:nand #t (function (lambda () 1)))",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
