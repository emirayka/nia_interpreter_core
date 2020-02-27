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

    #[test]
    fn returns_true_when_an_atom_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:atom? 1)", Value::Boolean(true)),
            ("(is:atom? 1.1)", Value::Boolean(true)),
            ("(is:atom? #t)", Value::Boolean(true)),
            ("(is:atom? #f)", Value::Boolean(true)),
            ("(is:atom? \"string\")", Value::Boolean(true)),
            ("(is:atom? 'symbol)", Value::Boolean(true)),
            ("(is:atom? :keyword)", Value::Boolean(true)),
            ("(is:atom? {})", Value::Boolean(true)),
            ("(is:atom? #())", Value::Boolean(true)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_false_when_not_an_atom_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:atom? (cons 1 nil))", Value::Boolean(false)),
            ("(is:atom? (cons 1 2))", Value::Boolean(false)),
            ("(is:atom? (cons 1 (cons 2 nil)))", Value::Boolean(false)),
            ("(is:atom? (cons 1 (cons 2 3)))", Value::Boolean(false)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(is:atom?)",
            "(is:atom? 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}