use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn is_enumerable_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:is-enumerable?' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(values.remove(0))?;

    let property_symbol_id =
        library::read_string_keyword_or_symbol_as_symbol_id(interpreter, values.remove(0))?;

    let object = interpreter.get_object(object_id)?;

    let result = object.is_property_enumerable(property_symbol_id)?.into();

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
    fn returns_enumerable_flag_value() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            ("(let ((obj {:prop 1})) (object:is-enumerable? obj :prop))", "#t"),
            ("(let ((obj {:prop 1})) (object:set-enumerable! obj :prop #f) (object:is-enumerable? obj :prop))", "#f"),
        );

        assertion::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:is-enumerable?)",
            "(object:is-enumerable? {})",
            "(object:is-enumerable? {} :a 2)",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_when_not_an_object_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:is-enumerable? 1 :prop)",
            "(object:is-enumerable? 1.1 :prop)",
            "(object:is-enumerable? #t :prop)",
            "(object:is-enumerable? #f :prop)",
            "(object:is-enumerable? \"string\" :prop)",
            "(object:is-enumerable? :keyword :prop)",
            "(object:is-enumerable? 'symbol :prop)",
            "(object:is-enumerable? '(list) :prop)",
            "(object:is-enumerable? #() :prop)",
            "(object:is-enumerable? {} 1)",
            "(object:is-enumerable? {} 1.1)",
            "(object:is-enumerable? {} #t)",
            "(object:is-enumerable? {} #f)",
            "(object:is-enumerable? {} '(list))",
            "(object:is-enumerable? {} {})",
            "(object:is-enumerable? {} #())",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }
}
