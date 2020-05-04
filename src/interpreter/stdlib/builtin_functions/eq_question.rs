use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn eq_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `eq?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let value1 = values.remove(0);
    let value2 = values.remove(0);

    let result = value1 == value2;

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
    fn returns_correct_comparison_results_for_equal_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(eq? 1 1)", "#t"),
            ("(eq? 1.1 1.1)", "#t"),
            ("(eq? #t #t)", "#t"),
            ("(eq? #f #f)", "#t"),
            ("(eq? \"string\" \"string\")", "#t"),
            ("(eq? 'symbol 'symbol)", "#t"),
            ("(eq? :keyword :keyword)", "#t"),
            ("(eq? {:a 1} {:a 1})", "#f"),
            ("(eq? '(1 2) '(1 2))", "#f"),
            ("(eq? #(+ %1 %2) #(+ %1 %2))", "#f"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_correct_comparison_results_for_not_equal_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(eq? 1 2)", "#f"),
            ("(eq? 1.1 1.2)", "#f"),
            ("(eq? #t #f)", "#f"),
            ("(eq? #f #t)", "#f"),
            ("(eq? \"string-1\" \"string-2\")", "#f"),
            ("(eq? 'symbol-1 'symbol-2)", "#f"),
            ("(eq? :keyword-1 :keyword-2)", "#f"),
            ("(eq? {:a 1} {:a 2})", "#f"),
            ("(eq? '(1 2) '(1 3))", "#f"),
            ("(eq? #(+ %1 %2) #(+ %1 %3))", "#f"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_false_for_values_of_different_types() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(eq? 1 2.2)", "#f"),
            ("(eq? 1.1 1)", "#f"),
            ("(eq? #t \"string\")", "#f"),
            ("(eq? #f 'symbol)", "#f"),
            ("(eq? \"string-1\" :keyword)", "#f"),
            ("(eq? 'symbol-1 2)", "#f"),
            ("(eq? :keyword-1 1)", "#f"),
            ("(eq? {:a 1} 4)", "#f"),
            ("(eq? '(1 2) 5)", "#f"),
            ("(eq? #(+ %1 %2) 5)", "#f"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(eq?)", "(eq? 1)", "(eq? 1 2 3)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
