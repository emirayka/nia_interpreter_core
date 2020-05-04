use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::ObjectId;
use crate::SymbolId;
use crate::Value;

use crate::OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE;
use crate::OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE;
use crate::OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE;
use crate::OBJECT_VALUE_WRAPPER_FLAG_WRITABLE;

use crate::library;

pub fn check_property_is_not_defined(
    interpreter: &mut Interpreter,
    object_id: ObjectId,
    property_symbol_id: SymbolId,
) -> Result<(), Error> {
    if interpreter.object_has_property(object_id, property_symbol_id)? {
        return Error::generic_execution_error("Property already defined.").into();
    }

    Ok(())
}

pub fn read_property_symbol_id_from_property_descriptor(
    interpreter: &mut Interpreter,
    property_descriptor_object_id: ObjectId,
) -> Result<SymbolId, Error> {
    let name_symbol_id = interpreter.intern("name");

    let property_symbol_id = if let Some(value) =
        interpreter.get_object_property(property_descriptor_object_id, name_symbol_id)?
    {
        library::read_string_keyword_or_symbol_as_symbol_id(interpreter, value)?
    } else {
        return Error::invalid_argument_error(
            "Built-in function `object:define-property!' takes property descriptor with a name.",
        )
        .into();
    };

    Ok(property_symbol_id)
}

pub fn read_property_value_from_property_descriptor(
    interpreter: &mut Interpreter,
    property_descriptor_object_id: ObjectId,
) -> Result<Value, Error> {
    let value_symbol_id = interpreter.intern("value");

    let value = if let Some(value) =
        interpreter.get_object_property(property_descriptor_object_id, value_symbol_id)?
    {
        value
    } else {
        interpreter.intern_nil_symbol_value()
    };

    Ok(value)
}

pub fn read_internable_flag_from_property_descriptor(
    interpreter: &mut Interpreter,
    property_descriptor_object_id: ObjectId,
) -> Result<bool, Error> {
    let internable_symbol_id = interpreter.intern("internable");

    let internable = if let Some(value) =
        interpreter.get_object_property(property_descriptor_object_id, internable_symbol_id)?
    {
        library::read_as_bool(value)?
    } else {
        true
    };

    Ok(internable)
}

pub fn read_writable_flag_from_property_descriptor(
    interpreter: &mut Interpreter,
    property_descriptor_object_id: ObjectId,
) -> Result<bool, Error> {
    let writable_symbol_id = interpreter.intern("writable");

    let writable = if let Some(value) =
        interpreter.get_object_property(property_descriptor_object_id, writable_symbol_id)?
    {
        library::read_as_bool(value)?
    } else {
        true
    };

    Ok(writable)
}

pub fn read_enumerable_flag_from_property_descriptor(
    interpreter: &mut Interpreter,
    property_descriptor_object_id: ObjectId,
) -> Result<bool, Error> {
    let enumerable_symbol_id = interpreter.intern("enumerable");

    let enumerable = if let Some(value) =
        interpreter.get_object_property(property_descriptor_object_id, enumerable_symbol_id)?
    {
        library::read_as_bool(value)?
    } else {
        true
    };

    Ok(enumerable)
}

pub fn read_configurable_flag_from_property_descriptor(
    interpreter: &mut Interpreter,
    property_descriptor_object_id: ObjectId,
) -> Result<bool, Error> {
    let configurable_symbol_id = interpreter.intern("configurable");

    let configurable = if let Some(value) =
        interpreter.get_object_property(property_descriptor_object_id, configurable_symbol_id)?
    {
        library::read_as_bool(value)?
    } else {
        true
    };

    Ok(configurable)
}

pub fn read_flags_from_property_descriptor(
    interpreter: &mut Interpreter,
    property_descriptor_object_id: ObjectId,
) -> Result<u8, Error> {
    let flag_internable =
        read_internable_flag_from_property_descriptor(interpreter, property_descriptor_object_id)?;

    let flag_writable =
        read_writable_flag_from_property_descriptor(interpreter, property_descriptor_object_id)?;

    let flag_enumerable =
        read_enumerable_flag_from_property_descriptor(interpreter, property_descriptor_object_id)?;

    let flag_configurable = read_configurable_flag_from_property_descriptor(
        interpreter,
        property_descriptor_object_id,
    )?;

    let mut flags = 0x0u8;

    if flag_internable {
        flags |= OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE;
    }

    if flag_writable {
        flags |= OBJECT_VALUE_WRAPPER_FLAG_WRITABLE;
    }

    if flag_enumerable {
        flags |= OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE;
    }

    if flag_configurable {
        flags |= OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE;
    }

    Ok(flags)
}

pub fn define_property_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:define-property!' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let object_id = library::read_as_object_id(values.remove(0))?;

    let property_descriptor_object_id = library::read_as_object_id(values.remove(0))?;

    let property_symbol_id = read_property_symbol_id_from_property_descriptor(
        interpreter,
        property_descriptor_object_id,
    )?;

    check_property_is_not_defined(interpreter, object_id, property_symbol_id)?;

    let property_value =
        read_property_value_from_property_descriptor(interpreter, property_descriptor_object_id)?;

    let flags = read_flags_from_property_descriptor(interpreter, property_descriptor_object_id)?;

    let object = interpreter.get_object_mut(object_id)?;

    object.set_property(property_symbol_id, property_value)?;
    object.set_property_flags(property_symbol_id, flags)?;

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
    fn defines_new_property() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "(let ((obj {})) (object:define-property! obj {:name \"prop\"}) obj)",
                "{:prop nil}",
            ),
            (
                "(let ((obj {})) (object:define-property! obj {:name 'prop}) obj)",
                "{:prop nil}",
            ),
            (
                "(let ((obj {})) (object:define-property! obj {:name :prop}) obj)",
                "{:prop nil}",
            ),
            (
                "(let ((obj {})) (object:define-property! obj {:name \"prop\" :value 1}) obj)",
                "{:prop 1}",
            ),
            (
                "(let ((obj {})) (object:define-property! obj {:name 'prop :value 1}) obj)",
                "{:prop 1}",
            ),
            (
                "(let ((obj {})) (object:define-property! obj {:name :prop :value 1}) obj)",
                "{:prop 1}",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn defines_new_property_with_flags() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\"}) (object:is-internable? obj :prop))", "#t"),
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\" :internable #f}) (object:is-internable? obj :prop))", "#f"),
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\" :internable #t}) (object:is-internable? obj :prop))", "#t"),

            ("(let ((obj {})) (object:define-property! obj {:name \"prop\"}) (object:is-writable? obj :prop))", "#t"),
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\" :writable #f}) (object:is-writable? obj :prop))", "#f"),
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\" :writable #t}) (object:is-writable? obj :prop))", "#t"),

            ("(let ((obj {})) (object:define-property! obj {:name \"prop\"}) (object:is-enumerable? obj :prop))", "#t"),
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\" :enumerable #f}) (object:is-enumerable? obj :prop))", "#f"),
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\" :enumerable #t}) (object:is-enumerable? obj :prop))", "#t"),

            ("(let ((obj {})) (object:define-property! obj {:name \"prop\"}) (object:is-configurable? obj :prop))", "#t"),
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\" :configurable #f}) (object:is-configurable? obj :prop))", "#f"),
            ("(let ((obj {})) (object:define-property! obj {:name \"prop\" :configurable #t}) (object:is-configurable? obj :prop))", "#t"),
        );

        assertion::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_generic_execution_error_when_property_already_defined() {
        let mut interpreter = Interpreter::new();

        let specs =
            vec!["(let ((obj {:a 1})) (object:define-property! obj {:name 'a :value 10}) obj)"];

        assertion::assert_results_are_generic_execution_errors(&mut interpreter, specs);
    }

    #[test]
    fn returns_invalid_argument_error_when_name_was_not_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(let ((obj {})) (object:define-property! obj {:value 1}) obj)",
            "(let ((obj {})) (object:define-property! obj {:value 1}) obj)",
            "(let ((obj {})) (object:define-property! obj {:value 1}) obj)",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, specs);
    }

    #[test]
    fn returns_invalid_argument_when_invalid_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:define-property! 1 {})",
            "(object:define-property! 1.1 {})",
            "(object:define-property! #t {})",
            "(object:define-property! #f {})",
            "(object:define-property! \"string\" {})",
            "(object:define-property! :keyword {})",
            "(object:define-property! 'symbol {})",
            "(object:define-property! '() {})",
            "(object:define-property! #() {})",
            "(object:define-property! {} 1)",
            "(object:define-property! {} 1.1)",
            "(object:define-property! {} #t)",
            "(object:define-property! {} #f)",
            "(object:define-property! {} \"string\")",
            "(object:define-property! {} :keyword)",
            "(object:define-property! {} 'symbol)",
            "(object:define-property! {} '())",
            "(object:define-property! {} #())",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:define-property!)",
            "(object:define-property! {})",
            "(object:define-property! {} 'item 'sym2)",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
