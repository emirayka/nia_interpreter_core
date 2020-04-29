use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn take(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:take' takes two arguments."
        ).into();
    }

    let mut values = values;

    let count = library::read_as_i64(
        interpreter,
        values.remove(0)
    )? as usize;

    let mut values = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    if values.len() < count {
        return Error::invalid_argument_count_error(
            "Built-in function `list:take' takes a list that has length greater than count."
        ).into();
    }

    values.drain(count..);

    Ok(interpreter.vec_to_list(values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_heads() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:take 0 '(1 2 3 4 5))", "'()"),
            ("(list:take 1 '(1 2 3 4 5))", "'(1)"),
            ("(list:take 2 '(1 2 3 4 5))", "'(1 2)"),
            ("(list:take 3 '(1 2 3 4 5))", "'(1 2 3)"),
            ("(list:take 4 '(1 2 3 4 5))", "'(1 2 3 4)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:take 1.1 '())",
            "(list:take #t '())",
            "(list:take #f '())",
            "(list:take \"string\" '())",
            "(list:take 'symbol '())",
            "(list:take :keyword '())",
            "(list:take '(1 2 3) '())",
            "(list:take {} '())",
            "(list:take #() '())",

            "(list:take #() 1)",
            "(list:take #() 1.1)",
            "(list:take #() #t)",
            "(list:take #() #f)",
            "(list:take #() \"string\")",
            "(list:take #() 'symbol)",
            "(list:take #() :keyword)",
            "(list:take #() {})",
            "(list:take #() #())",
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
            "(list:take)",
            "(list:take 1)",
            "(list:take 1 2 3)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
