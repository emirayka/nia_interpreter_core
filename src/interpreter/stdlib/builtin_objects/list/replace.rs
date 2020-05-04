use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn replace(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:replace' takes exactly three arguments.",
        )
        .into();
    }

    let mut values = values;

    let index = library::read_as_i64(values.remove(0))? as usize;

    let value = values.remove(0);

    let mut values = library::read_as_vector(interpreter, values.remove(0))?;

    if let Some(value_ref) = values.get_mut(index) {
        *value_ref = value;
    } else {
        return Error::invalid_argument_error(
            "Built-in function `list:replace' takes a list that has enough items.",
        )
        .into();
    }

    // todo: probably change it because it's not optimal
    Ok(interpreter.vec_to_list(values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn replaces_element_in_list() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(list:replace 0 0 '(1 2 3 4))", "'(0 2 3 4)"),
            ("(list:replace 1 0 '(1 2 3 4))", "'(1 0 3 4)"),
            ("(list:replace 2 0 '(1 2 3 4))", "'(1 2 0 4)"),
            ("(list:replace 3 0 '(1 2 3 4))", "'(1 2 3 0)"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_list_has_zero_length() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(list:replace 1 1 '())"];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:replace 1.1 1 '(1 2 3))",
            "(list:replace #t 1 '(1 2 3))",
            "(list:replace #f 1 '(1 2 3))",
            "(list:replace \"string\" 1 '(1 2 3))",
            "(list:replace 'symbol 1 '(1 2 3))",
            "(list:replace :keyword 1 '(1 2 3))",
            "(list:replace '(1 2 3) 1 '(1 2 3))",
            "(list:replace {} 1 '(1 2 3))",
            "(list:replace #() 1 '(1 2 3))",
            "(list:replace 0 1 1)",
            "(list:replace 0 1 1.1)",
            "(list:replace 0 1 #t)",
            "(list:replace 0 1 #f)",
            "(list:replace 0 1 \"string\")",
            "(list:replace 0 1 'symbol)",
            "(list:replace 0 1 :keyword)",
            "(list:replace 0 1 {})",
            "(list:replace 0 1 #())",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:replace)",
            "(list:replace 1)",
            "(list:replace 1 2)",
            "(list:replace 1 2 3 4)",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
