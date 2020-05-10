use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::ObjectId;
use crate::SymbolId;
use crate::Value;

use crate::library;

pub fn delete_property_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:set!' takes three arguments exactly.",
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

    let object = interpreter.get_object_mut(object_id)?;

    if !object.has_property(property_symbol_id) {
        return Error::generic_execution_error("Object has not property.")
            .into();
    }

    object.delete_property(property_symbol_id)?;

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
    fn deletes_property() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "(let ((obj {:prop 1})) (object:delete-property! obj \"prop\") obj)",
                "{}",
            ),
            (
                "(let ((obj {:prop 1})) (object:delete-property! obj :prop) obj)",
                "{}",
            ),
            (
                "(let ((obj {:prop 1})) (object:delete-property! obj 'prop) obj)",
                "{}",
            ),
            (
                "(let ((obj {:prop 1 :prop-2 2})) (object:delete-property! obj \"prop\") obj)",
                "{:prop-2 2}",
            ),
            (
                "(let ((obj {:prop 1 :prop-2 2})) (object:delete-property! obj :prop) obj)",
                "{:prop-2 2}",
            ),
            (
                "(let ((obj {:prop 1 :prop-2 2})) (object:delete-property! obj 'prop) obj)",
                "{:prop-2 2}",
            ),
            (
                "(let ((obj {:prop 1 :prop-2 2})) (object:delete-property! obj \"prop-2\") obj)",
                "{:prop 1}",
            ),
            (
                "(let ((obj {:prop 1 :prop-2 2})) (object:delete-property! obj :prop-2) obj)",
                "{:prop 1}",
            ),
            (
                "(let ((obj {:prop 1 :prop-2 2})) (object:delete-property! obj 'prop-2) obj)",
                "{:prop 1}",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_generic_execution_error_when_property_does_not_exist_in_object()
    {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(let ((obj {})) (object:delete-property! obj \"prop-2\") obj)",
            "(let ((obj {})) (object:delete-property! obj :prop-2) obj)",
            "(let ((obj {})) (object:delete-property! obj 'prop-2) obj)",
            "(let ((obj {:prop 1})) (object:delete-property! obj \"prop-2\") obj)",
            "(let ((obj {:prop 1})) (object:delete-property! obj :prop-2) obj)",
            "(let ((obj {:prop 1})) (object:delete-property! obj 'prop-2) obj)",
        ];

        utils::assert_results_are_generic_execution_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_invalid_argument_when_invalid_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:delete-property! 1 {})",
            "(object:delete-property! 1.1 {})",
            "(object:delete-property! #t {})",
            "(object:delete-property! #f {})",
            "(object:delete-property! \"string\" {})",
            "(object:delete-property! :keyword {})",
            "(object:delete-property! 'symbol {})",
            "(object:delete-property! '(list) {})",
            "(object:delete-property! #() {})",
            "(object:delete-property! {} 1)",
            "(object:delete-property! {} 1.1)",
            "(object:delete-property! {} #t)",
            "(object:delete-property! {} #f)",
            "(object:delete-property! {} '(list))",
            "(object:delete-property! {} {})",
            "(object:delete-property! {} #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:delete-property!)",
            "(object:delete-property! {:item 2})",
            "(object:delete-property! {:item 2} 'item 'sym2)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
