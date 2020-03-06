use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::lib;

pub fn join(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut result = Vec::new();

    for value in values {
        let vector = lib::read_as_vector(
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
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_concatenated_lists() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            ("(list:join)", "'()"),
            ("(list:join '(1 2))", "'(1 2)"),
            ("(list:join '(1 2) '(3 4))", "'(1 2 3 4)"),
            ("(list:join '(1 2) '(3 4) '(5 6))", "'(1 2 3 4 5 6)"),
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
            "(list:join 1)",
            "(list:join 1.1)",
            "(list:join #t)",
            "(list:join #f)",
            "(list:join \"string\")",
            "(list:join 'symbol)",
            "(list:join :keyword)",
            "(list:join {})",
            "(list:join #())",

            "(list:join '() 1)",
            "(list:join '() 1.1)",
            "(list:join '() #t)",
            "(list:join '() #f)",
            "(list:join '() \"string\")",
            "(list:join '() 'symbol)",
            "(list:join '() :keyword)",
            "(list:join '() {})",
            "(list:join '() #())",

            "(list:join '() '() 1)",
            "(list:join '() '() 1.1)",
            "(list:join '() '() #t)",
            "(list:join '() '() #f)",
            "(list:join '() '() \"string\")",
            "(list:join '() '() 'symbol)",
            "(list:join '() '() :keyword)",
            "(list:join '() '() {})",
            "(list:join '() '() #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}
