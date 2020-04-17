use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

pub fn neq_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `neq?' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let value1 = values.remove(0);
    let value2 = values.remove(0);

    let result = value1 == value2;

    Ok(Value::Boolean(!result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_false_for_equal_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(neq? 1 1)", "#f"),
            ("(neq? 1.1 1.1)", "#f"),
            ("(neq? #t #t)", "#f"),
            ("(neq? #f #f)", "#f"),
            ("(neq? \"string\" \"string\")", "#f"),
            ("(neq? 'symbol 'symbol)", "#f"),
            ("(neq? :keyword :keyword)", "#f"),

            ("(neq? {:a 1} {:a 1})", "#t"),
            ("(neq? '(1 2) '(1 2))", "#t"),
            ("(neq? #(+ %1 %2) #(+ %1 %2))", "#t")
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_true_for_not_equal_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(neq? 1 2)", "#t"),
            ("(neq? 1.1 1.2)", "#t"),
            ("(neq? #t #f)", "#t"),
            ("(neq? #f #t)", "#t"),
            ("(neq? \"string-1\" \"string-2\")", "#t"),
            ("(neq? 'symbol-1 'symbol-2)", "#t"),
            ("(neq? :keyword-1 :keyword-2)", "#t"),

            ("(neq? {:a 1} {:a 2})", "#t"),
            ("(neq? '(1 2) '(1 3))", "#t"),
            ("(neq? #(+ %1 %2) #(+ %1 %3))", "#t")
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_true_for_values_of_different_types() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(neq? 1 2.2)", "#t"),
            ("(neq? 1.1 1)", "#t"),
            ("(neq? #t \"string\")", "#t"),
            ("(neq? #f 'symbol)", "#t"),
            ("(neq? \"string-1\" :keyword)", "#t"),
            ("(neq? 'symbol-1 2)", "#t"),
            ("(neq? :keyword-1 1)", "#t"),

            ("(neq? {:a 1} 4)", "#t"),
            ("(neq? '(1 2) 5)", "#t"),
            ("(neq? #(+ %1 %2) 5)", "#t")
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
            "(neq?)",
            "(neq? 1)",
            "(neq? 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
