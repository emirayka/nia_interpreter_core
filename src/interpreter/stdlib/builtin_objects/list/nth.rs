use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn nth(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:nth' takes two arguments."
        ).into_result();
    }

    let mut values = values;

    let index = library::read_as_i64(
        interpreter,
        values.remove(0)
    )? as usize;

    let vector = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    if let Some(value) = vector.get(index) {
        Ok(*value)
    } else {
        return interpreter.make_invalid_argument_error(
            "Built-in function `list:nth' takes a list that has enough items."
        ).into_result();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    
    #[test]
    fn returns_element_with_the_index() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:nth 0 '(1 2 3 4))", "1"),
            ("(list:nth 1 '(1 2 3 4))", "2"),
            ("(list:nth 2 '(1 2 3 4))", "3"),
            ("(list:nth 3 '(1 2 3 4))", "4")
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_list_has_zero_length() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:nth 1 '())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:nth 1.1 '(1 2 3))",
            "(list:nth #t '(1 2 3))",
            "(list:nth #f '(1 2 3))",
            "(list:nth \"string\" '(1 2 3))",
            "(list:nth 'symbol '(1 2 3))",
            "(list:nth :keyword '(1 2 3))",
            "(list:nth '(1 2 3) '(1 2 3))",
            "(list:nth {} '(1 2 3))",
            "(list:nth #() '(1 2 3))",

            "(list:nth 0 1)",
            "(list:nth 0 1.1)",
            "(list:nth 0 #t)",
            "(list:nth 0 #f)",
            "(list:nth 0 \"string\")",
            "(list:nth 0 'symbol)",
            "(list:nth 0 :keyword)",
            "(list:nth 0 {})",
            "(list:nth 0 #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:nth)",
            "(list:nth 1)",
            "(list:nth 1 2 3)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
