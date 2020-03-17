use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn aperture(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:aperture' takes two arguments."
        ).into_result();
    }

    let mut values = values;

    let size = library::read_as_positive_i64(
        interpreter,
        values.remove(0)
    )? as usize;

    let mut values = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    let mut result = Vec::new();

    if size <= values.len() {
        let diff = 1 + values.len() - size;

        for i in 0..diff {
            result.push(interpreter.vec_to_list((&values[i..i + size]).to_vec())); // todo: possible optimisation
        }
    }

    Ok(interpreter.vec_to_list(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn returns_correct_apertures() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:aperture 1 '(1 2 3 4 5))", "'((1) (2) (3) (4) (5))"),
            ("(list:aperture 2 '(1 2 3 4 5))", "'((1 2) (2 3) (3 4) (4 5))"),
            ("(list:aperture 3 '(1 2 3 4 5))", "'((1 2 3) (2 3 4) (3 4 5))"),
            ("(list:aperture 4 '(1 2 3 4 5))", "'((1 2 3 4) (2 3 4 5))"),
            ("(list:aperture 5 '(1 2 3 4 5))", "'((1 2 3 4 5))"),
            ("(list:aperture 6 '(1 2 3 4 5))", "'()"),
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
            "(list:aperture 0 '())",
            "(list:aperture -1 '())",
            "(list:aperture 1.1 '())",
            "(list:aperture #t '())",
            "(list:aperture #f '())",
            "(list:aperture \"string\" '())",
            "(list:aperture 'symbol '())",
            "(list:aperture :keyword '())",
            "(list:aperture '(1 2 3) '())",
            "(list:aperture {} '())",
            "(list:aperture #() '())",

            "(list:aperture #() 1)",
            "(list:aperture #() 1.1)",
            "(list:aperture #() #t)",
            "(list:aperture #() #f)",
            "(list:aperture #() \"string\")",
            "(list:aperture #() 'symbol)",
            "(list:aperture #() :keyword)",
            "(list:aperture #() {})",
            "(list:aperture #() #())",
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
            "(list:aperture)",
            "(list:aperture 1)",
            "(list:aperture 1 2 3)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
