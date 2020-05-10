use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn set_configurable_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:set-configurable?' takes three arguments exactly.",
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

    object.set_property_configurable(property_symbol_id, flag_value)?;

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
    fn sets_configurable_flag() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            (
                "(let ((obj {:prop 1})) (object:is-configurable? obj :prop))",
                "#t",
            ),
            (
                "(let ((obj {:prop 1})) (object:set-configurable! obj :prop #f) (object:is-configurable? obj :prop))",
                "#f",
            ),
            (
                "(try (let ((obj {:prop 1})) (object:set-configurable! obj :prop #f) (object:set-configurable! obj :prop #t) #f) (catch 'generic-execution-error #t))",
                "#t",
            ), // todo: probably change error symbol here
        ];

        utils::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:set-configurable!)",
            "(object:set-configurable! {})",
            "(object:set-configurable! {} :a)",
            "(object:set-configurable! {} :a #t 2)",
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
            "(object:set-configurable! 1 :prop #t)",
            "(object:set-configurable! 1.1 :prop #t)",
            "(object:set-configurable! #t :prop #t)",
            "(object:set-configurable! #f :prop #t)",
            "(object:set-configurable! \"string\" :prop #t)",
            "(object:set-configurable! :keyword :prop #t)",
            "(object:set-configurable! 'symbol :prop #t)",
            "(object:set-configurable! '(list) :prop #t)",
            "(object:set-configurable! #() :prop #t)",
            "(object:set-configurable! {} 1 #t)",
            "(object:set-configurable! {} 1.1 #t)",
            "(object:set-configurable! {} #t #t)",
            "(object:set-configurable! {} #f #t)",
            "(object:set-configurable! {} '(list) #t)",
            "(object:set-configurable! {} {} #t)",
            "(object:set-configurable! {} #() #t)",
            "(object:set-configurable! {} 'a 1)",
            "(object:set-configurable! {} 'a 1.1)",
            "(object:set-configurable! {} 'a \"string\")",
            "(object:set-configurable! {} 'a :keyword)",
            "(object:set-configurable! {} 'a 'symbol)",
            "(object:set-configurable! {} 'a '(list))",
            "(object:set-configurable! {} 'a #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
