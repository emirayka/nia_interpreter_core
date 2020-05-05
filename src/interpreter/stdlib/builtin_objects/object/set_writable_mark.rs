use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn set_writable_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:set-writable?' takes three arguments exactly.",
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

    object.set_property_writable(property_symbol_id, flag_value)?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn sets_writable_flag() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            (
                "(let ((obj {:prop 1})) (object:is-writable? obj :prop))",
                "#t",
            ),
            (
                "(let ((obj {:prop 1})) (object:set-writable! obj :prop #f) (object:is-writable? obj :prop))",
                "#f",
            ),
            (
                "(try (let ((obj {:prop 1})) (object:set-writable! obj :prop #f) (object:set! obj :prop 2) #f) (catch 'generic-execution-error #t))",
                "#t",
            ), // todo: probably change error symbol here
        ];

        assertion::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:set-writable!)",
            "(object:set-writable! {})",
            "(object:set-writable! {} :a)",
            "(object:set-writable! {} :a #t 2)",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_when_not_an_object_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:set-writable! 1 :prop #t)",
            "(object:set-writable! 1.1 :prop #t)",
            "(object:set-writable! #t :prop #t)",
            "(object:set-writable! #f :prop #t)",
            "(object:set-writable! \"string\" :prop #t)",
            "(object:set-writable! :keyword :prop #t)",
            "(object:set-writable! 'symbol :prop #t)",
            "(object:set-writable! '(list) :prop #t)",
            "(object:set-writable! #() :prop #t)",
            "(object:set-writable! {} 1 #t)",
            "(object:set-writable! {} 1.1 #t)",
            "(object:set-writable! {} #t #t)",
            "(object:set-writable! {} #f #t)",
            "(object:set-writable! {} '(list) #t)",
            "(object:set-writable! {} {} #t)",
            "(object:set-writable! {} #() #t)",
            "(object:set-writable! {} 'a 1)",
            "(object:set-writable! {} 'a 1.1)",
            "(object:set-writable! {} 'a \"string\")",
            "(object:set-writable! {} 'a :keyword)",
            "(object:set-writable! {} 'a 'symbol)",
            "(object:set-writable! {} 'a '(list))",
            "(object:set-writable! {} 'a #())",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
