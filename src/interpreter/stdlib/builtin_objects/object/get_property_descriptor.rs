use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::{ObjectId, SymbolId};

// todo: refactor this somehow to
fn extract_property_descriptor_items(
    interpreter: &mut Interpreter,
    object_id: ObjectId,
    property_symbol_id: SymbolId,
) -> Result<(Value, Value, Value, Value, Value, Value), Error> {
    let name = property_symbol_id.into();
    let object = interpreter.get_object_mut(object_id)?;

    if !object.has_property(property_symbol_id) {
        return Error::generic_execution_error(
            "Object has not property to yield.",
        )
        .into();
    }

    let value =
        object
            .get_property_value(property_symbol_id)?
            .ok_or_else(|| {
                Error::generic_execution_error(
                    "Object has not property to yield.",
                )
            })?;

    let internable = object.is_property_internable(property_symbol_id)?.into();

    let writable = object.is_property_writable(property_symbol_id)?.into();

    let enumerable = object.is_property_enumerable(property_symbol_id)?.into();

    let configurable =
        object.is_property_configurable(property_symbol_id)?.into();

    Ok((name, value, internable, writable, enumerable, configurable))
}

fn construct_property_descriptor(
    interpreter: &mut Interpreter,
    object_id: ObjectId,
    property_symbol_id: SymbolId,
) -> Result<Value, Error> {
    let name_symbol_id = interpreter.intern_symbol_id("name");
    let value_symbol_id = interpreter.intern_symbol_id("value");
    let internable_symbol_id = interpreter.intern_symbol_id("internable");
    let writable_symbol_id = interpreter.intern_symbol_id("writable");
    let enumerable_symbol_id = interpreter.intern_symbol_id("enumerable");
    let configurable_symbol_id = interpreter.intern_symbol_id("configurable");

    let (name, value, internable, writable, enumerable, configurable) =
        extract_property_descriptor_items(
            interpreter,
            object_id,
            property_symbol_id,
        )?;

    let property_descriptor_object_id = interpreter.make_object();
    let object = interpreter.get_object_mut(property_descriptor_object_id)?;

    object.set_property(name_symbol_id, name)?;
    object.set_property(value_symbol_id, value)?;
    object.set_property(internable_symbol_id, internable)?;
    object.set_property(writable_symbol_id, writable)?;
    object.set_property(enumerable_symbol_id, enumerable)?;
    object.set_property(configurable_symbol_id, configurable)?;

    Ok(Value::Object(property_descriptor_object_id))
}

pub fn get_property_descriptor(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:get-property-descriptor' takes two arguments exactly.",
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

    let property_descriptor_object_id = construct_property_descriptor(
        interpreter,
        object_id,
        property_symbol_id,
    )?;

    Ok(property_descriptor_object_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_property_descriptor() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            (
                "(let ((obj {:a 1})) (object:get-property-descriptor obj :a))",
                "{:name 'a :value 1 :internable #t :writable #t :enumerable #t :configurable #t}",
            ),
            (
                "(let ((obj {:b 2})) (object:set-internable! obj :b #t) (object:get-property-descriptor obj :b))",
                "{:name 'b :value 2 :internable #t :writable #t :enumerable #t :configurable #t}",
            ),
            (
                "(let ((obj {:c 3})) (object:set-writable! obj :c #t) (object:get-property-descriptor obj :c))",
                "{:name 'c :value 3 :internable #t :writable #t :enumerable #t :configurable #t}",
            ),
            (
                "(let ((obj {:d 4})) (object:set-enumerable! obj :d #t) (object:get-property-descriptor obj :d))",
                "{:name 'd :value 4 :internable #t :writable #t :enumerable #t :configurable #t}",
            ),
            (
                "(let ((obj {:e 5})) (object:set-configurable! obj :e #t) (object:get-property-descriptor obj :e))",
                "{:name 'e :value 5 :internable #t :writable #t :enumerable #t :configurable #t}",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct()
    {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(object:get-property-descriptor)",
            "(object:get-property-descriptor {:prop 1})",
            "(object:get-property-descriptor {:prop 1} :prop 2)",
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
            "(object:get-property-descriptor 1 :prop)",
            "(object:get-property-descriptor 1.1 :prop)",
            "(object:get-property-descriptor #t :prop)",
            "(object:get-property-descriptor #f :prop)",
            "(object:get-property-descriptor \"string\" :prop)",
            "(object:get-property-descriptor :keyword :prop)",
            "(object:get-property-descriptor 'symbol :prop)",
            "(object:get-property-descriptor '(list) :prop)",
            "(object:get-property-descriptor #() :prop)",
            "(object:get-property-descriptor {} 1)",
            "(object:get-property-descriptor {} 1.1)",
            "(object:get-property-descriptor {} #t)",
            "(object:get-property-descriptor {} #f)",
            "(object:get-property-descriptor {} '(list))",
            "(object:get-property-descriptor {} {})",
            "(object:get-property-descriptor {} #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
