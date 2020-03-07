use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

use crate::interpreter::library;

pub fn equal_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `eq?' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let value1 = values.remove(0);
    let value2 = values.remove(0);

    let result = library::deep_equal(
        interpreter,
        value1,
        value2
    )?;

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_comparison_results_for_equal_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(equal? 1 1)", "#t"),
            ("(equal? 1.1 1.1)", "#t"),
            ("(equal? #t #t)", "#t"),
            ("(equal? #f #f)", "#t"),
            ("(equal? \"string\" \"string\")", "#t"),
            ("(equal? 'symbol 'symbol)", "#t"),
            ("(equal? :keyword :keyword)", "#t"),

            ("(equal? {:a 1} {:a 1})", "#t"),
            ("(equal? '(1 2) '(1 2))", "#t"),
            ("(equal? #(+ %1 %2) #(+ %1 %2))", "#t")
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_correct_comparison_results_for_not_equal_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(equal? 1 2)", "#f"),
            ("(equal? 1.1 1.2)", "#f"),
            ("(equal? #t #f)", "#f"),
            ("(equal? #f #t)", "#f"),
            ("(equal? \"string-1\" \"string-2\")", "#f"),
            ("(equal? 'symbol-1 'symbol-2)", "#f"),
            ("(equal? :keyword-1 :keyword-2)", "#f"),

            ("(equal? {:a 1} {:a 2})", "#f"),
            ("(equal? '(1 2) '(1 3))", "#f"),
            ("(equal? #(+ %1 %2) #(+ %1 %3))", "#f")
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_false_for_values_of_different_types() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(equal? 1 2.2)", "#f"),
            ("(equal? 1.1 1)", "#f"),
            ("(equal? #t \"string\")", "#f"),
            ("(equal? #f 'symbol)", "#f"),
            ("(equal? \"string-1\" :keyword)", "#f"),
            ("(equal? 'symbol-1 2)", "#f"),
            ("(equal? :keyword-1 1)", "#f"),

            ("(equal? {:a 1} 4)", "#f"),
            ("(equal? '(1 2) 5)", "#f"),
            ("(equal? #(+ %1 %2) 5)", "#f")
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_count_when_not_enough_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(equal?)",
            "(equal? 1)",
            "(equal? 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
