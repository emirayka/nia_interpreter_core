use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn is_frozen_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:is-frozen?' takes one argument exactly.",
        )
        .into();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(values.remove(0))?;

    let object = interpreter.get_object(object_id)?;
    let result = object.is_frozen().into();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_is_frozen_value() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            ("(let ((obj {})) (object:is-frozen? obj))", "#f"),
            (
                "(let ((obj {})) (object:freeze! obj) (object:is-frozen? obj))",
                "#t",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(object:is-frozen?)", "(object:is-frozen? {} 'val)"];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_when_not_an_object_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:is-frozen? 1)",
            "(object:is-frozen? 1.1)",
            "(object:is-frozen? #t)",
            "(object:is-frozen? #f)",
            "(object:is-frozen? \"string\")",
            "(object:is-frozen? :keyword)",
            "(object:is-frozen? 'symbol)",
            "(object:is-frozen? '(list))",
            "(object:is-frozen? #())",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }
}
