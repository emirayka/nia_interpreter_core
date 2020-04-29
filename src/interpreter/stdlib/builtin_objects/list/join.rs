use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn join(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:join' takes two arguments exactly."
        ).into();
    }

    let mut result = Vec::new();

    for value in values {
        let vector = library::read_as_vector(
            interpreter,
            value
        )?;

        result.extend(vector);
    }

    Ok(interpreter.vec_to_list(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_concatenated_lists() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            ("(list:join '() '(1 2 3 4))", "'(1 2 3 4)"),
            ("(list:join '(1) '(2 3 4))", "'(1 2 3 4)"),
            ("(list:join '(1 2) '(3 4))", "'(1 2 3 4)"),
            ("(list:join '(1 2 3) '(4))", "'(1 2 3 4)"),
            ("(list:join '(1 2 3 4) '())", "'(1 2 3 4)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:join 1 '())",
            "(list:join 1.1 '())",
            "(list:join #t '())",
            "(list:join #f '())",
            "(list:join \"string\" '())",
            "(list:join 'symbol '())",
            "(list:join :keyword '())",
            "(list:join {} '())",
            "(list:join #() '())",

            "(list:join '() 1)",
            "(list:join '() 1.1)",
            "(list:join '() #t)",
            "(list:join '() #f)",
            "(list:join '() \"string\")",
            "(list:join '() 'symbol)",
            "(list:join '() :keyword)",
            "(list:join '() {})",
            "(list:join '() #())",
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
            "(list:join)",
            "(list:join 1)",
            "(list:join 1 2 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
