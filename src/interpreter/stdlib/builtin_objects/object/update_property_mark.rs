use std::convert::TryInto;

use crate::SymbolId;
use crate::ObjectId;
use crate::Value;
use crate::Error;
use crate::EnvironmentId;
use crate::Interpreter;

use crate::OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE;
use crate::OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE;
use crate::OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE;
use crate::OBJECT_VALUE_WRAPPER_FLAG_WRITABLE;

use crate::library;

fn read_create_flag_from_property_descriptor(
    interpreter: &mut Interpreter,
    property_descriptor_object_id: ObjectId,
) -> Result<bool, Error> {
    let create_symbol_id = interpreter.intern("create");

    let result = if let Some(value) = interpreter.get_object_property(
        property_descriptor_object_id,
        create_symbol_id
    )? {
        library::read_as_bool(interpreter, value)?
    } else {
        true
    };

    Ok(result)
}

pub fn update_property_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:update-property!' takes two arguments exactly."
        ).into();
    }

    let mut values = values;

    let object_id = library::read_as_object_id(
        interpreter,
        values.remove(0)
    )?;

    let property_descriptor_object_id = library::read_as_object_id(
        interpreter,
        values.remove(0)
    )?;

    let property_symbol_id = super::define_property_mark::read_property_symbol_id_from_property_descriptor(
        interpreter,
        property_descriptor_object_id
    )?;

    let create_flag = read_create_flag_from_property_descriptor(
        interpreter,
        property_descriptor_object_id
    )?;

    if !create_flag && !interpreter.object_has_property(object_id, property_symbol_id)? {
        return Error::generic_execution_error(
            "Cannot update property because it is not defined and create flag is false."
        ).into();
    }

    let property_value = super::define_property_mark::read_property_value_from_property_descriptor(
        interpreter,
        property_descriptor_object_id
    )?;

    let flags = super::define_property_mark::read_flags_from_property_descriptor(
        interpreter,
        property_descriptor_object_id
    )?;

    let object = interpreter.get_object_mut(object_id)?;

    object.set_property(property_symbol_id, property_value)?;
    object.set_property_flags(property_symbol_id, flags)?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::for_special_symbols;

    #[test]
    fn defines_new_property() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\"}) obj)", "{:prop nil}"),
            ("(let ((obj {})) (object:update-property! obj {:name 'prop}) obj)", "{:prop nil}"),
            ("(let ((obj {})) (object:update-property! obj {:name :prop}) obj)", "{:prop nil}"),

            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :value 1}) obj)", "{:prop 1}"),
            ("(let ((obj {})) (object:update-property! obj {:name 'prop :value 1}) obj)", "{:prop 1}"),
            ("(let ((obj {})) (object:update-property! obj {:name :prop :value 1}) obj)", "{:prop 1}"),

            ("(let ((obj {})) (object:update-property! obj {:create #t :name \"prop\"}) obj)", "{:prop nil}"),
            ("(let ((obj {})) (object:update-property! obj {:create #t :name 'prop}) obj)", "{:prop nil}"),
            ("(let ((obj {})) (object:update-property! obj {:create #t :name :prop}) obj)", "{:prop nil}"),

            ("(let ((obj {})) (object:update-property! obj {:create #t :name \"prop\" :value 1}) obj)", "{:prop 1}"),
            ("(let ((obj {})) (object:update-property! obj {:create #t :name 'prop :value 1}) obj)", "{:prop 1}"),
            ("(let ((obj {})) (object:update-property! obj {:create #t :name :prop :value 1}) obj)", "{:prop 1}"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_error_when_provided_create_flag_false_and_property_does_not_exist() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(let ((obj {})) (object:update-property! obj {:create #f :name \"prop\"}) obj)",
            "(let ((obj {})) (object:update-property! obj {:create #f :name 'prop}) obj)",
            "(let ((obj {})) (object:update-property! obj {:create #f :name :prop}) obj)",

            "(let ((obj {})) (object:update-property! obj {:create #f :name \"prop\" :value 1}) obj)",
            "(let ((obj {})) (object:update-property! obj {:create #f :name 'prop :value 1}) obj)",
            "(let ((obj {})) (object:update-property! obj {:create #f :name :prop :value 1}) obj)"
        );

        assertion::assert_results_are_generic_execution_errors(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn defines_new_property_with_flags() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\"}) (object:is-internable? obj :prop))", "#t"),
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :internable #f}) (object:is-internable? obj :prop))", "#f"),
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :internable #t}) (object:is-internable? obj :prop))", "#t"),

            ("(let ((obj {})) (object:update-property! obj {:name \"prop\"}) (object:is-writable? obj :prop))", "#t"),
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :writable #f}) (object:is-writable? obj :prop))", "#f"),
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :writable #t}) (object:is-writable? obj :prop))", "#t"),

            ("(let ((obj {})) (object:update-property! obj {:name \"prop\"}) (object:is-enumerable? obj :prop))", "#t"),
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :enumerable #f}) (object:is-enumerable? obj :prop))", "#f"),
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :enumerable #t}) (object:is-enumerable? obj :prop))", "#t"),

            ("(let ((obj {})) (object:update-property! obj {:name \"prop\"}) (object:is-configurable? obj :prop))", "#t"),
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :configurable #f}) (object:is-configurable? obj :prop))", "#f"),
            ("(let ((obj {})) (object:update-property! obj {:name \"prop\" :configurable #t}) (object:is-configurable? obj :prop))", "#t"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn sets_flags_of_existing_properties() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\"}) (object:is-internable? obj :prop))", "#t"),
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\" :internable #f}) (object:is-internable? obj :prop))", "#f"),
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\" :internable #t}) (object:is-internable? obj :prop))", "#t"),

            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\"}) (object:is-writable? obj :prop))", "#t"),
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\" :writable #f}) (object:is-writable? obj :prop))", "#f"),
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\" :writable #t}) (object:is-writable? obj :prop))", "#t"),

            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\"}) (object:is-enumerable? obj :prop))", "#t"),
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\" :enumerable #f}) (object:is-enumerable? obj :prop))", "#f"),
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\" :enumerable #t}) (object:is-enumerable? obj :prop))", "#t"),

            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\"}) (object:is-configurable? obj :prop))", "#t"),
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\" :configurable #f}) (object:is-configurable? obj :prop))", "#f"),
            ("(let ((obj {:prop nil})) (object:update-property! obj {:name \"prop\" :configurable #t}) (object:is-configurable? obj :prop))", "#t"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_name_was_not_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(let ((obj {})) (object:update-property! obj {:value 1}) obj)",
            "(let ((obj {})) (object:update-property! obj {:value 1}) obj)",
            "(let ((obj {})) (object:update-property! obj {:value 1}) obj)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_invalid_argument_when_invalid_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(object:update-property! 1 {})",
            "(object:update-property! 1.1 {})",
            "(object:update-property! #t {})",
            "(object:update-property! #f {})",
            "(object:update-property! \"string\" {})",
            "(object:update-property! :keyword {})",
            "(object:update-property! 'symbol {})",
            "(object:update-property! '() {})",
            "(object:update-property! #() {})",

            "(object:update-property! {} 1)",
            "(object:update-property! {} 1.1)",
            "(object:update-property! {} #t)",
            "(object:update-property! {} #f)",
            "(object:update-property! {} \"string\")",
            "(object:update-property! {} :keyword)",
            "(object:update-property! {} 'symbol)",
            "(object:update-property! {} '())",
            "(object:update-property! {} #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(object:update-property!)",
            "(object:update-property! {})",
            "(object:update-property! {} 'item 'sym2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
