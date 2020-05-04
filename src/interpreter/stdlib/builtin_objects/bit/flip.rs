use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn flip(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `bit:flip' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let index = library::read_as_i64(values.remove(0))?;

    if index < 0 || index > 63 {
        return Error::invalid_argument_error(
            "Built-in function `bit:flip' takes value between [0; 64) as bit index.",
        )
        .into();
    }

    let value = library::read_as_i64(values.remove(0))?;

    let result = (0x1 << index) ^ value;

    Ok(Value::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn flips_bit() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(bit:flip 0 0)", "1"),
            ("(bit:flip 0 1)", "0"),
            ("(bit:flip 1 0)", "2"),
            ("(bit:flip 1 1)", "3"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(bit:flip 1 1.1)",
            "(bit:flip 1 #t)",
            "(bit:flip 1 #f)",
            "(bit:flip 1 'symbol)",
            "(bit:flip 1 \"string\")",
            "(bit:flip 1 :keyword)",
            "(bit:flip 1 '(s-expression))",
            "(bit:flip 1 {})",
            "(bit:flip 1 #())",
            "(bit:flip 1.1 1)",
            "(bit:flip #t 1)",
            "(bit:flip #f 1)",
            "(bit:flip 'symbol 1)",
            "(bit:flip \"string\" 1)",
            "(bit:flip :keyword 1)",
            "(bit:flip '(s-expression) 1)",
            "(bit:flip {} 1)",
            "(bit:flip #() 1)",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(bit:flip)", "(bit:flip 1)", "(bit:flip 1 2 3)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
