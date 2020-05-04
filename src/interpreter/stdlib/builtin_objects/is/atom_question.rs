use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn atom_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `is:atom?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Cons(_) => false,
        _ => true,
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_true_when_an_atom_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:atom? 1)", Value::Boolean(true)),
            ("(is:atom? 1.1)", Value::Boolean(true)),
            ("(is:atom? #t)", Value::Boolean(true)),
            ("(is:atom? #f)", Value::Boolean(true)),
            ("(is:atom? \"string\")", Value::Boolean(true)),
            ("(is:atom? 'symbol)", Value::Boolean(true)),
            ("(is:atom? :keyword)", Value::Boolean(true)),
            ("(is:atom? {})", Value::Boolean(true)),
            ("(is:atom? #())", Value::Boolean(true)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_false_when_not_an_atom_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:atom? (cons 1 nil))", Value::Boolean(false)),
            ("(is:atom? (cons 1 2))", Value::Boolean(false)),
            ("(is:atom? (cons 1 (cons 2 nil)))", Value::Boolean(false)),
            ("(is:atom? (cons 1 (cons 2 3)))", Value::Boolean(false)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:atom?)", "(is:atom? 1 2)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
