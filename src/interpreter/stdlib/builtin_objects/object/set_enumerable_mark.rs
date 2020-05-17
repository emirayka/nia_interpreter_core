use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn set_enumerable_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:set-enumerable?' takes three arguments exactly.",
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

    let flag_value = library::read_as_bool(values.remove(0))?;

    let object = interpreter.get_object_mut(object_id)?;

    object.set_property_enumerable(property_symbol_id, flag_value)?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn sets_enumerable_flag() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            (
                "(let ((obj {:prop 1})) (object:is-enumerable? obj :prop))",
                "#t",
            ),
            (
                "(let ((obj {:prop 1})) (object:set-enumerable! obj :prop #f) (object:is-enumerable? obj :prop))",
                "#f",
            ),
            // todo: when iterators would be implemented, add test here
        ];

        utils::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:set-enumerable!)",
            "(object:set-enumerable! {})",
            "(object:set-enumerable! {} :a)",
            "(object:set-enumerable! {} :a #t 2)",
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
            "(object:set-enumerable! 1 :prop #t)",
            "(object:set-enumerable! 1.1 :prop #t)",
            "(object:set-enumerable! #t :prop #t)",
            "(object:set-enumerable! #f :prop #t)",
            "(object:set-enumerable! \"string\" :prop #t)",
            "(object:set-enumerable! :keyword :prop #t)",
            "(object:set-enumerable! 'symbol :prop #t)",
            "(object:set-enumerable! '(list:new) :prop #t)",
            "(object:set-enumerable! #() :prop #t)",
            "(object:set-enumerable! {} 1 #t)",
            "(object:set-enumerable! {} 1.1 #t)",
            "(object:set-enumerable! {} #t #t)",
            "(object:set-enumerable! {} #f #t)",
            "(object:set-enumerable! {} '(list:new) #t)",
            "(object:set-enumerable! {} {} #t)",
            "(object:set-enumerable! {} #() #t)",
            "(object:set-enumerable! {} 'a 1)",
            "(object:set-enumerable! {} 'a 1.1)",
            "(object:set-enumerable! {} 'a \"string\")",
            "(object:set-enumerable! {} 'a :keyword)",
            "(object:set-enumerable! {} 'a 'symbol)",
            "(object:set-enumerable! {} 'a '(list:new))",
            "(object:set-enumerable! {} 'a #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
