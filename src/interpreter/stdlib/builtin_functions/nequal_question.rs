use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;

pub fn nequal_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `nequal?' must take exactly one argument."
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

    Ok(Value::Boolean(!result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_comparison_results_for_equal_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(nequal? 1 1)", "#f"),
            ("(nequal? 1.1 1.1)", "#f"),
            ("(nequal? #t #t)", "#f"),
            ("(nequal? #f #f)", "#f"),
            ("(nequal? \"string\" \"string\")", "#f"),
            ("(nequal? 'symbol 'symbol)", "#f"),
            ("(nequal? :keyword :keyword)", "#f"),

            ("(nequal? {:a 1} {:a 1})", "#f"),
            ("(nequal? '(1 2) '(1 2))", "#f"),
            ("(nequal? #(+ %1 %2) #(+ %1 %2))", "#f")
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
            ("(nequal? 1 2)", "#t"),
            ("(nequal? 1.1 1.2)", "#t"),
            ("(nequal? #t #f)", "#t"),
            ("(nequal? #f #t)", "#t"),
            ("(nequal? \"string-1\" \"string-2\")", "#t"),
            ("(nequal? 'symbol-1 'symbol-2)", "#t"),
            ("(nequal? :keyword-1 :keyword-2)", "#t"),

            ("(nequal? {:a 1} {:a 2})", "#t"),
            ("(nequal? '(1 2) '(1 3))", "#t"),
            ("(nequal? #(+ %1 %2) #(+ %1 %3))", "#t")
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
            ("(nequal? 1 2.2)", "#t"),
            ("(nequal? 1.1 1)", "#t"),
            ("(nequal? #t \"string\")", "#t"),
            ("(nequal? #f 'symbol)", "#t"),
            ("(nequal? \"string-1\" :keyword)", "#t"),
            ("(nequal? 'symbol-1 2)", "#t"),
            ("(nequal? :keyword-1 1)", "#t"),

            ("(nequal? {:a 1} 4)", "#t"),
            ("(nequal? '(1 2) 5)", "#t"),
            ("(nequal? #(+ %1 %2) 5)", "#t")
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
            "(nequal?)",
            "(nequal? 1)",
            "(nequal? 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
