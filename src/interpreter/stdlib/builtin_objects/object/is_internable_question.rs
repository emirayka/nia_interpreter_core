use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn is_internable_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:is-internable?' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(values.remove(0))?;

    let property_symbol_id =
        library::read_string_keyword_or_symbol_as_symbol_id(
            interpreter,
            values.remove(0),
        )?;

    let object = interpreter.get_object(object_id)?;

    let result = object.is_property_internable(property_symbol_id)?.into();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_internable_flag_value() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            (
                "(let ((obj {:prop 1})) (object:is-internable? obj :prop))",
                "#t",
            ),
            (
                "(let ((obj {:prop 1})) (object:set-internable! obj :prop #f) (object:is-internable? obj :prop))",
                "#f",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:is-internable?)",
            "(object:is-internable? {})",
            "(object:is-internable? {} :a 2)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_when_not_an_object_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:is-internable? 1 :prop)",
            "(object:is-internable? 1.1 :prop)",
            "(object:is-internable? #t :prop)",
            "(object:is-internable? #f :prop)",
            "(object:is-internable? \"string\" :prop)",
            "(object:is-internable? :keyword :prop)",
            "(object:is-internable? 'symbol :prop)",
            "(object:is-internable? '(list:new) :prop)",
            "(object:is-internable? #() :prop)",
            "(object:is-internable? {} 1)",
            "(object:is-internable? {} 1.1)",
            "(object:is-internable? {} #t)",
            "(object:is-internable? {} #f)",
            "(object:is-internable? {} '(list:new))",
            "(object:is-internable? {} {})",
            "(object:is-internable? {} #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
