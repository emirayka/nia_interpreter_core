use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn set_internable_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:set-internable!' takes three arguments exactly.",
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

    object.set_property_internable(property_symbol_id, flag_value)?;

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
    fn sets_internable_flag() {
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
            (
                "(try (let ((obj {:prop 1})) (object:set-internable! obj :prop #f) obj:prop #f) (catch 'generic-execution-error #t))",
                "#t",
            ), // todo: probably change error symbol here
            (
                "(try (let ((obj {:prop 1})) (object:set-internable! obj :prop #f) (object:get obj :prop) #f) (catch 'generic-execution-error #t))",
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
            "(object:set-internable!)",
            "(object:set-internable! {})",
            "(object:set-internable! {} :a)",
            "(object:set-internable! {} :a #t 2)",
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
            "(object:set-internable! 1 :prop #t)",
            "(object:set-internable! 1.1 :prop #t)",
            "(object:set-internable! #t :prop #t)",
            "(object:set-internable! #f :prop #t)",
            "(object:set-internable! \"string\" :prop #t)",
            "(object:set-internable! :keyword :prop #t)",
            "(object:set-internable! 'symbol :prop #t)",
            "(object:set-internable! '(list:new) :prop #t)",
            "(object:set-internable! #() :prop #t)",
            "(object:set-internable! {} 1 #t)",
            "(object:set-internable! {} 1.1 #t)",
            "(object:set-internable! {} #t #t)",
            "(object:set-internable! {} #f #t)",
            "(object:set-internable! {} '(list:new) #t)",
            "(object:set-internable! {} {} #t)",
            "(object:set-internable! {} #() #t)",
            "(object:set-internable! {} 'a 1)",
            "(object:set-internable! {} 'a 1.1)",
            "(object:set-internable! {} 'a \"string\")",
            "(object:set-internable! {} 'a :keyword)",
            "(object:set-internable! {} 'a 'symbol)",
            "(object:set-internable! {} 'a '(list:new))",
            "(object:set-internable! {} 'a #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
