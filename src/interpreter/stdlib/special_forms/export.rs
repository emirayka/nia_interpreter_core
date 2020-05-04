use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::Value;
use crate::{ObjectId, SymbolId};

use crate::library;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportType {
    Export(SymbolId),
    ExportAsNamed(SymbolId, SymbolId),
    ExportAsDefault(SymbolId),

    ExportObject(ObjectId),
    ExportObjectAsNamed(ObjectId, SymbolId),
    ExportObjectAsDefault(ObjectId),

    Reexport(SymbolId, String),
    ReexportDefault(String),
    ReexportAsNamed(SymbolId, SymbolId, String),
    ReexportAsDefault(SymbolId, String),

    ReexportAll(String),
    ReexportAllAsNamed(SymbolId, String),
    ReexportAllAsDefault(String),

    ReexportObject(ObjectId, String),
    ReexportObjectAsNamed(ObjectId, SymbolId, String),
    ReexportObjectAsDefault(ObjectId, String),
}

fn deep_equal_export_type(
    interpreter: &mut Interpreter,
    v1: ExportType,
    v2: ExportType,
) -> Result<bool, Error> {
    use ExportType::*;

    let result = match (v1, v2) {
        (ExportObject(o1), ExportObject(o2))
        | (ExportObjectAsDefault(o1), ExportObjectAsDefault(o2)) => {
            library::deep_equal(interpreter, o1.into(), o2.into())?
        }
        (ExportObjectAsNamed(o1, s1), ExportObjectAsNamed(o2, s2)) => {
            library::deep_equal(interpreter, o1.into(), o2.into())?
                && library::deep_equal(interpreter, s1.into(), s2.into())?
        }
        (ReexportObject(o1, string1), ReexportObject(o2, string2))
        | (ReexportObjectAsDefault(o1, string1), ReexportObjectAsDefault(o2, string2)) => {
            library::deep_equal(interpreter, o1.into(), o2.into())? && string1 == string2
        }
        (ReexportObjectAsNamed(o1, s1, string1), ReexportObjectAsNamed(o2, s2, string2)) => {
            library::deep_equal(interpreter, o1.into(), o2.into())?
                && library::deep_equal(interpreter, s1.into(), s2.into())?
                && string1 == string2
        }
        (v1, v2) => v1 == v2,
    };

    Ok(result)
}

fn evaluate_values(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Vec<Value>, Error> {
    let values = values;
    let mut evaluated_values = Vec::new();

    for value in values {
        let evaluated_value = match value {
            // todo: make sure that there is no evaluation of s-expression
            // it should be parsing of `object:make' invocation
            Value::Cons(_) => {
                let evaluated_value = interpreter.execute_value(environment_id, value)?;

                if let Value::Object(object_did) = evaluated_value {
                    evaluated_value
                } else {
                    return Error::invalid_argument_error(
                        "Special form `export' takes as arguments only s-expressions that evaluates to objects, symbols and strings."
                    ).into();
                }
            }
            Value::Symbol(_) => value,
            Value::String(_) => value,
            _ => return Error::invalid_argument_error(
                "Special form `export' takes as arguments only s-expressions that evaluates to objects, symbols and strings."
            ).into()
        };

        evaluated_values.push(evaluated_value);
    }

    Ok(evaluated_values)
}

mod read_export_type {
    use super::*;

    fn check_interned_symbol_is_as(
        interpreter: &mut Interpreter,
        symbol_id: SymbolId,
    ) -> Result<(), Error> {
        library::check_interned_symbol_is_expected(interpreter, symbol_id, "as")
    }

    fn check_interned_symbol_is_from(
        interpreter: &mut Interpreter,
        symbol_id: SymbolId,
    ) -> Result<(), Error> {
        library::check_interned_symbol_is_expected(interpreter, symbol_id, "from")
    }

    fn check_interned_symbol_is_default(
        interpreter: &mut Interpreter,
        symbol_id: SymbolId,
    ) -> Result<(), Error> {
        library::check_interned_symbol_is_expected(interpreter, symbol_id, "default")
    }

    fn check_interned_symbol_is_asterisk(
        interpreter: &mut Interpreter,
        symbol_id: SymbolId,
    ) -> Result<(), Error> {
        library::check_interned_symbol_is_expected(interpreter, symbol_id, "*")
    }

    fn check_interned_symbol_is_not_asterisk(
        interpreter: &mut Interpreter,
        symbol_id: SymbolId,
    ) -> Result<(), Error> {
        let symbol = interpreter.get_symbol(symbol_id)?;

        if symbol.get_gensym_id() != 0 || symbol.get_name() == "*" {
            return Error::invalid_argument_error("Expected interned symbol that is not `*'.")
                .into();
        }

        return Ok(());
    }

    fn interned_symbol_is_default(
        interpreter: &mut Interpreter,
        symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let result = interpreter.get_symbol_name(symbol_id)? == "default";

        Ok(result)
    }

    fn try_read_export(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 1 {
            return Error::generic_execution_error("").into();
        }

        let export_value_symbol_id = library::read_as_symbol_id(values[0])?;

        Ok(ExportType::Export(export_value_symbol_id))
    }

    fn try_read_export_as_named_or_export_as_default(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let module_value_symbol_id = library::read_as_symbol_id(values[0])?;
        let as_symbol_id = library::read_as_symbol_id(values[1])?;
        let export_value_symbol_id = library::read_as_symbol_id(values[2])?;

        check_interned_symbol_is_as(interpreter, as_symbol_id)?;

        if interned_symbol_is_default(interpreter, export_value_symbol_id)? {
            Ok(ExportType::ExportAsDefault(module_value_symbol_id))
        } else {
            Ok(ExportType::ExportAsNamed(
                module_value_symbol_id,
                export_value_symbol_id,
            ))
        }
    }

    fn try_read_export_object(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 1 {
            return Error::generic_execution_error("").into();
        }

        let module_value_object_id = library::read_as_object_id(values[0])?;

        Ok(ExportType::ExportObject(module_value_object_id))
    }

    fn try_read_export_object_as_named_or_export_object_as_default(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let module_value_object_id = library::read_as_object_id(values[0])?;
        let as_symbol_id = library::read_as_symbol_id(values[1])?;
        let export_value_symbol_id = library::read_as_symbol_id(values[2])?;

        check_interned_symbol_is_as(interpreter, as_symbol_id)?;

        if interned_symbol_is_default(interpreter, export_value_symbol_id)? {
            Ok(ExportType::ExportObjectAsDefault(module_value_object_id))
        } else {
            Ok(ExportType::ExportObjectAsNamed(
                module_value_object_id,
                export_value_symbol_id,
            ))
        }
    }

    fn try_read_reexport_or_reexport_default(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let module_value_symbol_id = library::read_as_symbol_id(values[0])?;
        let from_symbol_id = library::read_as_symbol_id(values[1])?;
        let module_path_string_id = library::read_as_string_id(values[2])?;

        check_interned_symbol_is_not_asterisk(interpreter, module_value_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        let module_path_string = interpreter
            .get_string(module_path_string_id)?
            .get_string()
            .clone();

        let module_path_string = interpreter.resolve_with_current_module_path(
            module_path_string
        )?;

        if interned_symbol_is_default(interpreter, module_value_symbol_id)? {
            Ok(ExportType::ReexportDefault(module_path_string))
        } else {
            Ok(ExportType::Reexport(
                module_value_symbol_id,
                module_path_string,
            ))
        }
    }

    fn try_read_reexport_default(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let default_symbol_id = library::read_as_symbol_id(values[0])?;
        let from_symbol_id = library::read_as_symbol_id(values[1])?;
        let module_path_string_id = library::read_as_string_id(values[2])?;

        check_interned_symbol_is_default(interpreter, default_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        let module_path_string = interpreter
            .get_string(module_path_string_id)?
            .get_string()
            .clone();

        let module_path_string = interpreter.resolve_with_current_module_path(
            module_path_string
        )?;

        Ok(ExportType::ReexportDefault(module_path_string))
    }

    fn try_read_reexport_as_named_or_reexport_as_default(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 5 {
            return Error::generic_execution_error("").into();
        }

        let module_value_symbol_id = library::read_as_symbol_id(values[0])?;
        let as_symbol_id = library::read_as_symbol_id(values[1])?;
        let export_value_symbol_id = library::read_as_symbol_id(values[2])?;
        let from_symbol_id = library::read_as_symbol_id(values[3])?;
        let module_path_string_id = library::read_as_string_id(values[4])?;

        check_interned_symbol_is_not_asterisk(interpreter, module_value_symbol_id)?;
        check_interned_symbol_is_as(interpreter, as_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        let module_path_string = interpreter
            .get_string(module_path_string_id)?
            .get_string()
            .clone();

        if interned_symbol_is_default(interpreter, export_value_symbol_id)? {
            Ok(ExportType::ReexportAsDefault(
                module_value_symbol_id,
                module_path_string,
            ))
        } else {
            Ok(ExportType::ReexportAsNamed(
                module_value_symbol_id,
                export_value_symbol_id,
                module_path_string,
            ))
        }
    }

    fn try_read_reexport_all(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let asterisk_symbol_id = library::read_as_symbol_id(values[0])?;
        let from_symbol_id = library::read_as_symbol_id(values[1])?;
        let module_path_string_id = library::read_as_string_id(values[2])?;

        check_interned_symbol_is_asterisk(interpreter, asterisk_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        let module_path_string = interpreter
            .get_string(module_path_string_id)?
            .get_string()
            .clone();

        let module_path_string = interpreter.resolve_with_current_module_path(
            module_path_string
        )?;

        Ok(ExportType::ReexportAll(module_path_string))
    }

    fn try_read_reexport_all_as_named_or_reexport_all_as_default(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 5 {
            return Error::generic_execution_error("").into();
        }

        let asterisk_symbol_id = library::read_as_symbol_id(values[0])?;
        let as_symbol_id = library::read_as_symbol_id(values[1])?;
        let export_value_symbol_id = library::read_as_symbol_id(values[2])?;
        let from_symbol_id = library::read_as_symbol_id(values[3])?;
        let module_path_string_id = library::read_as_string_id(values[4])?;

        check_interned_symbol_is_asterisk(interpreter, asterisk_symbol_id)?;
        check_interned_symbol_is_as(interpreter, as_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        let module_path_string = interpreter
            .get_string(module_path_string_id)?
            .get_string()
            .clone();

        let module_path_string = interpreter.resolve_with_current_module_path(
            module_path_string
        )?;

        if interned_symbol_is_default(interpreter, export_value_symbol_id)? {
            Ok(ExportType::ReexportAllAsDefault(module_path_string))
        } else {
            Ok(ExportType::ReexportAllAsNamed(
                export_value_symbol_id,
                module_path_string,
            ))
        }
    }

    fn try_read_reexport_object(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let module_value_object_id = library::read_as_object_id(values[0])?;
        let from_symbol_id = library::read_as_symbol_id(values[1])?;
        let module_path_string_id = library::read_as_string_id(values[2])?;

        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        let module_path_string = interpreter
            .get_string(module_path_string_id)?
            .get_string()
            .clone();

        let module_path_string = interpreter.resolve_with_current_module_path(
            module_path_string
        )?;

        Ok(ExportType::ReexportObject(
            module_value_object_id,
            module_path_string,
        ))
    }

    fn try_read_reexport_object_as_named_or_reexport_object_as_default(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ExportType, Error> {
        if values.len() != 5 {
            return Error::generic_execution_error("").into();
        }

        let module_value_object_id = library::read_as_object_id(values[0])?;
        let as_symbol_id = library::read_as_symbol_id(values[1])?;
        let export_value_symbol_id = library::read_as_symbol_id(values[2])?;
        let from_symbol_id = library::read_as_symbol_id(values[3])?;
        let module_path_string_id = library::read_as_string_id(values[4])?;

        check_interned_symbol_is_as(interpreter, as_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        let module_path_string = interpreter
            .get_string(module_path_string_id)?
            .get_string()
            .clone();

        let module_path_string = interpreter.resolve_with_current_module_path(
            module_path_string
        )?;

        if interned_symbol_is_default(interpreter, export_value_symbol_id)? {
            Ok(ExportType::ReexportObjectAsDefault(
                module_value_object_id,
                module_path_string,
            ))
        } else {
            Ok(ExportType::ReexportObjectAsNamed(
                module_value_object_id,
                export_value_symbol_id,
                module_path_string,
            ))
        }
    }

    fn read_export_type_runner(
        funcs: Vec<fn(&mut Interpreter, &Vec<Value>) -> Result<ExportType, Error>>,
        interpreter: &mut Interpreter,
        values: Vec<Value>,
    ) -> Result<ExportType, Error> {
        for func in funcs {
            match func(interpreter, &values) {
                Ok(export_type) => return Ok(export_type),
                Err(error) => {
                    if error.is_failure() {
                        return Err(error);
                    }
                }
            }
        }

        Error::generic_execution_error("Cannot parse export.").into()
    }

    pub fn read_export_type(
        interpreter: &mut Interpreter,
        values: Vec<Value>,
    ) -> Result<ExportType, Error> {
        read_export_type_runner(
            vec![
                try_read_export,
                try_read_export_as_named_or_export_as_default,
                try_read_export_object,
                try_read_export_object_as_named_or_export_object_as_default,
                try_read_reexport_or_reexport_default,
                try_read_reexport_as_named_or_reexport_as_default,
                try_read_reexport_all,
                try_read_reexport_all_as_named_or_reexport_all_as_default,
                try_read_reexport_object,
                try_read_reexport_object_as_named_or_reexport_object_as_default,
            ],
            interpreter,
            values,
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ConsId;
        use nia_basic_assertions::{nia_assert, nia_assert_equal};
        use std::convert::TryInto;

        #[test]
        fn reads_export_types_correctly() {
            let mut interpreter = Interpreter::new();

            let name_symbol_id = interpreter.intern("name");
            let exported_name_symbol_id = interpreter.intern("exported-name");

            let name_1_symbol_id = interpreter.intern("name-1");
            let exported_name_1_symbol_id = interpreter.intern("exported-name-1");

            let name_2_symbol_id = interpreter.intern("name-2");
            let exported_name_2_symbol_id = interpreter.intern("exported-name-2");

            let module_path_string = String::from("./module");

            let export_object_1_object_id = interpreter
                .execute_in_main_environment("#{:name-1 :name-2}")
                .unwrap()
                .try_into()
                .unwrap();

            let export_object_2_object_id = interpreter
                .execute_in_main_environment("{:name-1 'exported-name-1 :name-2 'exported-name-2}")
                .unwrap()
                .try_into()
                .unwrap();

            let specs = vec!(
                (ExportType::Export(name_symbol_id), "(list 'name)"),
                (ExportType::ExportAsNamed(name_symbol_id, exported_name_symbol_id), "(list 'name 'as 'exported-name)"),
                (ExportType::ExportAsDefault(name_symbol_id), "(list 'name 'as 'default)"),
                (ExportType::ExportObject(export_object_1_object_id), "(list #{:name-1 :name-2})"),
                (ExportType::ExportObject(export_object_2_object_id), "(list {:name-1 'exported-name-1 :name-2 'exported-name-2})"),
                (ExportType::ExportObjectAsNamed(export_object_1_object_id, name_symbol_id), "(list #{:name-1 :name-2} 'as 'name)"),
                (ExportType::ExportObjectAsNamed(export_object_2_object_id, name_symbol_id), "(list {:name-1 'exported-name-1 :name-2 'exported-name-2} 'as 'name)"),
                (ExportType::ExportObjectAsDefault(export_object_1_object_id), "(list #{:name-1 :name-2} 'as 'default)"),
                (ExportType::ExportObjectAsDefault(export_object_2_object_id), "(list {:name-1 'exported-name-1 :name-2 'exported-name-2} 'as 'default)"),
                (ExportType::Reexport(name_symbol_id, module_path_string.clone()), "(list 'name 'from \"./module\")"),
                (ExportType::ReexportDefault(module_path_string.clone()), "(list 'default 'from \"./module\")"),
                (ExportType::ReexportAsNamed(name_symbol_id, exported_name_symbol_id, module_path_string.clone()), "(list 'name 'as 'exported-name 'from \"./module\")"),
                (ExportType::ReexportAsDefault(name_symbol_id, module_path_string.clone()), "(list 'name 'as 'default 'from \"./module\")"),
                (ExportType::ReexportAll(module_path_string.clone()), "(list '* 'from \"./module\")"),
                (ExportType::ReexportAllAsNamed(name_symbol_id, module_path_string.clone()), "(list '* 'as 'name 'from \"./module\")"),
                (ExportType::ReexportAllAsDefault(module_path_string.clone()), "(list '* 'as 'default 'from \"./module\")"),
                (ExportType::ReexportObject(export_object_1_object_id, module_path_string.clone()), "(list #{:name-1 :name-2} 'from \"./module\")"),
                (ExportType::ReexportObject(export_object_2_object_id, module_path_string.clone()), "(list {:name-1 'exported-name-1 :name-2 'exported-name-2} 'from \"./module\")"),
                (ExportType::ReexportObjectAsNamed(export_object_1_object_id, name_symbol_id, module_path_string.clone()), "(list #{:name-1 :name-2} 'as 'name 'from \"./module\")"),
                (ExportType::ReexportObjectAsNamed(export_object_2_object_id, name_symbol_id, module_path_string.clone()), "(list {:name-1 'exported-name-1 :name-2 'exported-name-2} 'as 'name 'from \"./module\")"),
                (ExportType::ReexportObjectAsDefault(export_object_1_object_id, module_path_string.clone()), "(list #{:name-1 :name-2} 'as 'default 'from \"./module\")"),
                (ExportType::ReexportObjectAsDefault(export_object_2_object_id, module_path_string.clone()), "(list {:name-1 'exported-name-1 :name-2 'exported-name-2} 'as 'default 'from \"./module\")"),
            );

            for (expected, code) in specs {
                let args = interpreter.execute_in_main_environment(code).unwrap();
                let cons_id: ConsId = args.try_into().unwrap();
                let args = interpreter.list_to_vec(cons_id).unwrap();
                println!("{}", code);
                println!("{:?}", expected);

                let result = read_export_type(&mut interpreter, args.clone()).unwrap();

                println!("{:?}", result);

                nia_assert(deep_equal_export_type(&mut interpreter, expected, result).unwrap());
            }
        }

        // todo: add negative tests here
        // that would be hard 😑
    }
}

mod eval_export {
    use super::*;
    use crate::ModuleId;

    enum Export {
        Named(SymbolId, Value),
        Default(Value),
    }

    fn try_intern_module(interpreter: &mut Interpreter, path: String) -> Result<ModuleId, Error> {
        interpreter.intern_module(&path)
    }

    fn read_value_from_environment(
        interpreter: &mut Interpreter,
        module_environment: EnvironmentId,
        module_name_symbol_id: SymbolId,
    ) -> Result<Value, Error> {
        match interpreter.lookup_variable(module_environment, module_name_symbol_id)? {
            Some(value) => Ok(value),
            None => {
                let module_name_symbol_name = interpreter.get_symbol_name(module_name_symbol_id)?;

                Error::generic_execution_error(&format!(
                    "Cannot resolve export \"{}\".",
                    module_name_symbol_name
                ))
                    .into()
            }
        }
    }

    fn read_export_object_properties(
        interpreter: &mut Interpreter,
        module_environment: EnvironmentId,
        object_id: ObjectId,
    ) -> Result<Vec<(SymbolId, Value)>, Error> {
        let object = interpreter.get_object(object_id)?;
        let mut pairs = Vec::new();

        for (module_name_symbol_id, export_name_value_wrapper) in object.get_properties() {
            let export_name_value = export_name_value_wrapper.get_value()?;
            let object_property_key_symbol_id = library::read_as_symbol_id(export_name_value)?;
            pairs.push((object_property_key_symbol_id, *module_name_symbol_id));
        }

        let mut result = Vec::new();

        for (object_property_key_symbol_id, module_name_symbol_id) in pairs {
            let module_name_value = read_value_from_environment(
                interpreter,
                module_environment,
                module_name_symbol_id,
            )?;

            result.push((object_property_key_symbol_id, module_name_value));
        }

        Ok(result)
    }

    fn read_key_value_vector_from_object(
        interpreter: &mut Interpreter,
        object_id: ObjectId,
    ) -> Result<Vec<(SymbolId, Value)>, Error> {
        let mut vector = Vec::new();

        let mut object = interpreter.get_object_mut(object_id)?;

        for (symbol_id, value) in object.get_properties() {
            let value = value.get_value()?;
            vector.push((*symbol_id, value));
        }

        Ok(vector)
    }

    fn construct_exportation_object(
        interpreter: &mut Interpreter,
        module_environment: EnvironmentId,
        object_id: ObjectId,
    ) -> Result<Value, Error> {
        let object_properties =
            read_export_object_properties(interpreter, module_environment, object_id)?;

        let mut export_object_id = interpreter.make_object();
        let export_object = interpreter.get_object_mut(export_object_id)?;

        for (key_symbol_id, value) in object_properties {
            export_object.set_property(key_symbol_id, value);
        }

        let export_object_value = export_object_id.into();

        Ok(export_object_value)
    }

    fn get_module_environment(
        interpreter: &mut Interpreter,
        module_path: &str,
    ) -> Result<EnvironmentId, Error> {
        let import_module_id = interpreter.intern_module(module_path)?;
        let import_module = interpreter.get_module_required_soft(import_module_id)?;
        let import_module_environment = import_module.get_environment();

        Ok(import_module_environment)
    }

    fn read_all_exports_of_module(
        interpreter: &mut Interpreter,
        module_path: &str,
    ) -> Result<Vec<(SymbolId, Value)>, Error> {
        let module_id = interpreter.intern_module(module_path)?;
        let module = interpreter.get_module_required_soft(module_id)?;

        let exports = module
            .get_exports()
            .iter()
            .map(|(symbol_id, value)| (*symbol_id, *value))
            .collect::<Vec<(SymbolId, Value)>>();

        Ok(exports)
    }

    fn construct_exportation_object_from_vector(
        interpreter: &mut Interpreter,
        vector: Vec<(SymbolId, Value)>,
    ) -> Result<Value, Error> {
        let object_id = interpreter.make_object();
        let object = interpreter.get_object_mut(object_id)?;

        for (property_name_symbol_id, property_value) in vector {
            object.set_property(property_name_symbol_id, property_value)?;
        }

        Ok(object_id.into())
    }

    fn read_reexports(
        interpreter: &mut Interpreter,
        object_properties: Vec<(SymbolId, Value)>,
        module_path: &str,
    ) -> Result<Vec<(SymbolId, Value)>, Error> {
        let mut result = Vec::new();

        let module_id = interpreter.intern_module(module_path)?;
        let module = interpreter.get_module_required_soft(module_id)?;

        for (module_name_symbol_id, export_name_value) in object_properties {
            let export_name_symbol_id = library::read_as_symbol_id(export_name_value)?;
            let module_export_value = match module.get_export(module_name_symbol_id) {
                Some(value) => value,
                None => return Error::generic_execution_error("Module has no export.").into(),
            };
            result.push((export_name_symbol_id, module_export_value))
        }

        Ok(result)
    }

    fn read_exportation_vector(
        interpreter: &mut Interpreter,
        current_module_environment: EnvironmentId,
        export_type: ExportType,
    ) -> Result<Vec<Export>, Error> {
        let exportation_vector = match export_type {
            ExportType::Export(module_name_symbol_id) => {
                let module_name_value = read_value_from_environment(
                    interpreter,
                    current_module_environment,
                    module_name_symbol_id,
                )?;
                vec![Export::Named(module_name_symbol_id, module_name_value)]
            }
            ExportType::ExportAsNamed(module_name_symbol_id, export_name_symbol_id) => {
                let module_name_value = read_value_from_environment(
                    interpreter,
                    current_module_environment,
                    module_name_symbol_id,
                )?;

                vec![Export::Named(export_name_symbol_id, module_name_value)]
            }
            ExportType::ExportAsDefault(module_name_symbol_id) => {
                let module_name_value = read_value_from_environment(
                    interpreter,
                    current_module_environment,
                    module_name_symbol_id,
                )?;

                vec![Export::Default(module_name_value)]
            }
            ExportType::ExportObject(object_id) => {
                let object_properties = read_export_object_properties(
                    interpreter,
                    current_module_environment,
                    object_id,
                )?;

                object_properties
                    .into_iter()
                    .map(|(symbol_id, value)| Export::Named(symbol_id, value))
                    .collect()
            }
            ExportType::ExportObjectAsNamed(object_id, export_name_symbol_id) => {
                let export_object_value = construct_exportation_object(
                    interpreter,
                    current_module_environment,
                    object_id,
                )?;

                vec![Export::Named(export_name_symbol_id, export_object_value)]
            }
            ExportType::ExportObjectAsDefault(object_id) => {
                let export_object_value = construct_exportation_object(
                    interpreter,
                    current_module_environment,
                    object_id,
                )?;

                vec![Export::Default(export_object_value)]
            }
            ExportType::Reexport(export_name_symbol_id, module_path) => {
                let module_id = interpreter.intern_module(&module_path)?;
                let module = interpreter.get_module_required_soft(module_id)?;

                let module_name_value = module
                    .get_export(export_name_symbol_id)
                    .ok_or_else(|| Error::generic_execution_error("Module has no named export."))?;

                vec![Export::Named(export_name_symbol_id, module_name_value)]
            }
            ExportType::ReexportDefault(module_path) => {
                let module_id = interpreter.intern_module(&module_path)?;
                let module = interpreter.get_module_required_soft(module_id)?;

                let module_name_value = module.get_default_export().ok_or_else(|| {
                    Error::generic_execution_error("Module has no default export.")
                })?;

                vec![Export::Default(module_name_value)]
            }
            ExportType::ReexportAsNamed(
                module_name_symbol_id,
                export_name_symbol_id,
                module_path,
            ) => {
                let module_id = interpreter.intern_module(&module_path)?;
                let module = interpreter.get_module_required_soft(module_id)?;

                let module_name_value =
                    module.get_export(module_name_symbol_id).ok_or_else(|| {
                        Error::generic_execution_error("Module has no default export.")
                    })?;

                vec![Export::Named(export_name_symbol_id, module_name_value)]
            }
            ExportType::ReexportAsDefault(module_name_symbol_id, module_path) => {
                let module_id = interpreter.intern_module(&module_path)?;
                let module = interpreter.get_module_required_soft(module_id)?;

                let module_name_value =
                    module.get_export(module_name_symbol_id).ok_or_else(|| {
                        Error::generic_execution_error("Module has no default export.")
                    })?;

                vec![Export::Default(module_name_value)]
            }
            ExportType::ReexportAll(module_path) => {
                let module_exports = read_all_exports_of_module(interpreter, &module_path)?;

                module_exports
                    .into_iter()
                    .map(|(symbol_id, value)| Export::Named(symbol_id, value))
                    .collect()
            }
            ExportType::ReexportAllAsNamed(export_name_symbol_id, module_path) => {
                let module_exports = read_all_exports_of_module(interpreter, &module_path)?;

                let export_object_value =
                    construct_exportation_object_from_vector(interpreter, module_exports)?;

                vec![Export::Named(export_name_symbol_id, export_object_value)]
            }
            ExportType::ReexportAllAsDefault(module_path) => {
                let module_exports = read_all_exports_of_module(interpreter, &module_path)?;

                let export_object_value =
                    construct_exportation_object_from_vector(interpreter, module_exports)?;

                vec![Export::Default(export_object_value)]
            }
            ExportType::ReexportObject(object_id, module_path) => {
                let object_properties = read_key_value_vector_from_object(interpreter, object_id)?;

                read_reexports(interpreter, object_properties, &module_path)?
                    .into_iter()
                    .map(|(symbol_id, value)| Export::Named(symbol_id, value))
                    .collect()
            }
            ExportType::ReexportObjectAsNamed(object_id, export_name_symbol_id, module_path) => {
                let object_properties = read_key_value_vector_from_object(interpreter, object_id)?;
                let reexports = read_reexports(interpreter, object_properties, &module_path)?;

                let export_object_id = interpreter.make_object();
                let export_object = interpreter.get_object_mut(export_object_id)?;

                for (symbol_id, value) in reexports {
                    export_object.set_property(symbol_id, value)?;
                }

                let export_object_value = export_object_id.into();

                vec![Export::Named(export_name_symbol_id, export_object_value)]
            }
            ExportType::ReexportObjectAsDefault(object_id, module_path) => {
                let object_properties = read_key_value_vector_from_object(interpreter, object_id)?;

                let reexports = read_reexports(interpreter, object_properties, &module_path)?;

                let export_object_id = interpreter.make_object();
                let export_object = interpreter.get_object_mut(export_object_id)?;

                for (symbol_id, value) in reexports {
                    export_object.set_property(symbol_id, value)?;
                }

                let export_object_value = export_object_id.into();

                vec![Export::Default(export_object_value)]
            }
        };

        Ok(exportation_vector)
    }

    pub fn eval_export(
        interpreter: &mut Interpreter,
        environment_id: EnvironmentId,
        export_type: ExportType,
    ) -> Result<(), Error> {
        let exportation_vector = read_exportation_vector(interpreter, environment_id, export_type)?;

        let current_module = interpreter.get_current_module_mut();

        for export in exportation_vector {
            match export {
                Export::Named(symbol_id, value) => current_module.add_export(symbol_id, value)?,
                Export::Default(value) => current_module.add_default_export(value)?,
            }
        }

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[allow(unused_imports)]
        use nia_basic_assertions::*;

        #[allow(unused_imports)]
        use crate::utils::assertion;
        use std::io::Write;

        fn assert_export_type_evaluated_correctly(
            interpreter: &mut Interpreter,
            export_type: ExportType,
            expected_default_export: Option<Value>,
            expected_exports: Vec<(SymbolId, Value)>,
        ) {
            let main_environment_id = interpreter.get_main_environment_id();
            eval_export(interpreter, main_environment_id, export_type).unwrap();

            let exports = interpreter.get_main_module().get_exports();
            let default_export = interpreter.get_main_module().get_default_export();

            assertion::assert_option_deep_equal(
                interpreter,
                expected_default_export,
                default_export,
            );
            nia_assert_equal(expected_exports.len(), exports.len());

            for (expected_symbol_id, expected_value) in expected_exports {
                nia_assert(exports.contains_key(&expected_symbol_id));
                assertion::assert_deep_equal(
                    interpreter,
                    expected_value,
                    *exports.get(&expected_symbol_id).unwrap(),
                );
            }
        }

        #[test]
        fn evaluates_export_correctly() {
            let mut interpreter = Interpreter::new();

            let main_environment_id = interpreter.get_main_environment_id();
            let module_symbol = interpreter.intern("test-nia-name");
            let value = Value::Integer(1);

            let main_environment_id = interpreter.get_main_environment_id();
            interpreter
                .define_variable(main_environment_id, module_symbol, value)
                .unwrap();

            assert_export_type_evaluated_correctly(
                &mut interpreter,
                ExportType::Export(module_symbol),
                None,
                vec![(module_symbol, value)],
            );
        }

        #[test]
        fn evaluates_export_as_named_correctly() {
            let mut interpreter = Interpreter::new();

            let main_environment_id = interpreter.get_main_environment_id();
            let module_symbol = interpreter.intern("test-nia-name");
            let export_symbol = interpreter.intern("test-nia-exported-name");
            let value = Value::Integer(1);

            let main_environment_id = interpreter.get_main_environment_id();
            interpreter
                .define_variable(main_environment_id, module_symbol, value)
                .unwrap();

            assert_export_type_evaluated_correctly(
                &mut interpreter,
                ExportType::ExportAsNamed(module_symbol, export_symbol),
                None,
                vec![(export_symbol, value)],
            );
        }

        #[test]
        fn evaluates_export_default_correctly() {
            let mut interpreter = Interpreter::new();

            let main_environment_id = interpreter.get_main_environment_id();
            let module_symbol = interpreter.intern("test-nia-name");
            let value = Value::Integer(1);

            let main_environment_id = interpreter.get_main_environment_id();
            interpreter
                .define_variable(main_environment_id, module_symbol, value)
                .unwrap();

            assert_export_type_evaluated_correctly(
                &mut interpreter,
                ExportType::ExportAsDefault(module_symbol),
                Some(value),
                vec![],
            );
        }

        #[test]
        fn evaluates_export_object_correctly() {
            let mut interpreter = Interpreter::new();

            let main_environment_id = interpreter.get_main_environment_id();
            let module_symbol_1 = interpreter.intern("test-nia-name-1");
            let module_symbol_2 = interpreter.intern("test-nia-name-2");
            let export_symbol_1 = interpreter.intern("test-nia-exported-name-1");
            let export_symbol_2 = interpreter.intern("test-nia-exported-name-2");
            let value_1 = Value::Integer(1);
            let value_2 = Value::Integer(2);

            let main_environment_id = interpreter.get_main_environment_id();
            interpreter
                .define_variable(main_environment_id, module_symbol_1, value_1)
                .unwrap();
            interpreter
                .define_variable(main_environment_id, module_symbol_2, value_2)
                .unwrap();

            let object_id = interpreter.make_object();
            interpreter.set_object_property(object_id, module_symbol_1, export_symbol_1.into());
            interpreter.set_object_property(object_id, module_symbol_2, export_symbol_2.into());

            assert_export_type_evaluated_correctly(
                &mut interpreter,
                ExportType::ExportObject(object_id),
                None,
                vec![(export_symbol_1, value_1), (export_symbol_2, value_2)],
            );
        }

        #[test]
        fn evaluates_export_object_as_named_correctly() {
            let mut interpreter = Interpreter::new();

            let main_environment_id = interpreter.get_main_environment_id();

            let object_name_symbol = interpreter.intern("test-nia-object");
            let module_symbol_1 = interpreter.intern("test-nia-name-1");
            let module_symbol_2 = interpreter.intern("test-nia-name-2");
            let export_symbol_1 = interpreter.intern("test-nia-exported-name-1");
            let export_symbol_2 = interpreter.intern("test-nia-exported-name-2");
            let value_1 = Value::Integer(1);
            let value_2 = Value::Integer(2);

            let main_environment_id = interpreter.get_main_environment_id();
            interpreter
                .define_variable(main_environment_id, module_symbol_1, value_1)
                .unwrap();
            interpreter
                .define_variable(main_environment_id, module_symbol_2, value_2)
                .unwrap();

            let object_id = interpreter.make_object();
            interpreter.set_object_property(object_id, module_symbol_1, export_symbol_1.into());
            interpreter.set_object_property(object_id, module_symbol_2, export_symbol_2.into());

            let expected_object_id = interpreter.make_object();
            interpreter.set_object_property(expected_object_id, export_symbol_1, value_1);
            interpreter.set_object_property(expected_object_id, export_symbol_2, value_2);

            assert_export_type_evaluated_correctly(
                &mut interpreter,
                ExportType::ExportObjectAsNamed(object_id, object_name_symbol),
                None,
                vec![(object_name_symbol, expected_object_id.into())],
            );
        }

        #[test]
        fn evaluates_export_object_default_correctly() {
            let mut interpreter = Interpreter::new();

            let main_environment_id = interpreter.get_main_environment_id();

            let object_name_symbol = interpreter.intern("test-nia-object");
            let module_symbol_1 = interpreter.intern("test-nia-name-1");
            let module_symbol_2 = interpreter.intern("test-nia-name-2");
            let export_symbol_1 = interpreter.intern("test-nia-exported-name-1");
            let export_symbol_2 = interpreter.intern("test-nia-exported-name-2");
            let value_1 = Value::Integer(1);
            let value_2 = Value::Integer(2);

            let main_environment_id = interpreter.get_main_environment_id();
            interpreter
                .define_variable(main_environment_id, module_symbol_1, value_1)
                .unwrap();
            interpreter
                .define_variable(main_environment_id, module_symbol_2, value_2)
                .unwrap();

            let object_id = interpreter.make_object();
            interpreter.set_object_property(object_id, module_symbol_1, export_symbol_1.into());
            interpreter.set_object_property(object_id, module_symbol_2, export_symbol_2.into());

            let expected_object_id = interpreter.make_object();
            interpreter.set_object_property(expected_object_id, export_symbol_1, value_1);
            interpreter.set_object_property(expected_object_id, export_symbol_2, value_2);

            assert_export_type_evaluated_correctly(
                &mut interpreter,
                ExportType::ExportObjectAsDefault(object_id),
                Some(expected_object_id.into()),
                vec![],
            );
        }

        #[test]
        fn evaluates_reexport_correctly() {
            crate::utils::with_tempfile(
                "(defc test-nia-name 1) (export test-nia-name)",
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();
                    let module_symbol = interpreter.intern("test-nia-name");
                    let value = Value::Integer(1);

                    let main_environment_id = interpreter.get_main_environment_id();

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::Reexport(module_symbol, module_path),
                        None,
                        vec![(module_symbol, value)],
                    );
                },
            );
        }

        #[test]
        fn evaluates_reexport_default_correctly() {
            crate::utils::with_tempfile(
                "(defc test-nia-name 1) (export test-nia-name as default)",
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();
                    let module_symbol = interpreter.intern("test-nia-name");
                    let value = Value::Integer(1);

                    let main_environment_id = interpreter.get_main_environment_id();

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportDefault(module_path),
                        Some(value),
                        vec![],
                    );
                },
            );
        }

        #[test]
        fn evaluates_reexport_as_named_correctly() {
            crate::utils::with_tempfile(
                "(defc test-nia-name 1) (export test-nia-name)",
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();
                    let module_symbol = interpreter.intern("test-nia-name");
                    let export_symbol = interpreter.intern("test-nia-exported-name");
                    let value = Value::Integer(1);

                    let main_environment_id = interpreter.get_main_environment_id();

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportAsNamed(module_symbol, export_symbol, module_path),
                        None,
                        vec![(export_symbol, value)],
                    );
                },
            );
        }

        #[test]
        fn evaluates_reexport_as_default_correctly() {
            crate::utils::with_tempfile(
                "(defc test-nia-name 1) (export test-nia-name)",
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();
                    let module_symbol = interpreter.intern("test-nia-name");
                    let value = Value::Integer(1);

                    let main_environment_id = interpreter.get_main_environment_id();

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportAsDefault(module_symbol, module_path),
                        Some(value),
                        vec![],
                    );
                },
            )
        }

        #[test]
        fn evaluates_reexport_all_correctly() {
            crate::utils::with_tempfile(
                r#"
        (defc test-nia-name-1 1)
        (defc test-nia-name-2 2)
        (export test-nia-name-1)
        (export test-nia-name-2)"#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();

                    let module_symbol_1 = interpreter.intern("test-nia-name-1");
                    let module_symbol_2 = interpreter.intern("test-nia-name-2");
                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportAll(module_path),
                        None,
                        vec![(module_symbol_1, value_1), (module_symbol_2, value_2)],
                    );
                },
            )
        }

        #[test]
        fn evaluates_reexport_all_as_named_correctly() {
            crate::utils::with_tempfile(
                r#"
        (defc test-nia-name-1 1)
        (defc test-nia-name-2 2)
        (export test-nia-name-1)
        (export test-nia-name-2)"#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();

                    let object_name_symbol = interpreter.intern("test-nia-object");
                    let module_symbol_1 = interpreter.intern("test-nia-name-1");
                    let module_symbol_2 = interpreter.intern("test-nia-name-2");
                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    let expected_object_id = interpreter.make_object();
                    interpreter.set_object_property(expected_object_id, module_symbol_1, value_1);
                    interpreter.set_object_property(expected_object_id, module_symbol_2, value_2);

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportAllAsNamed(object_name_symbol, module_path),
                        None,
                        vec![(object_name_symbol, expected_object_id.into())],
                    );
                },
            )
        }

        #[test]
        fn evaluates_reexport_all_as_default_correctly() {
            crate::utils::with_tempfile(
                r#"
        (defc test-nia-name-1 1)
        (defc test-nia-name-2 2)
        (export test-nia-name-1)
        (export test-nia-name-2)"#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();

                    let module_symbol_1 = interpreter.intern("test-nia-name-1");
                    let module_symbol_2 = interpreter.intern("test-nia-name-2");
                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    let expected_object_id = interpreter.make_object();
                    interpreter.set_object_property(expected_object_id, module_symbol_1, value_1);
                    interpreter.set_object_property(expected_object_id, module_symbol_2, value_2);

                    let expected_object_value = expected_object_id.into();

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportAllAsDefault(module_path),
                        Some(expected_object_value),
                        vec![],
                    );
                },
            )
        }

        #[test]
        fn evaluates_reexport_object_correctly() {
            crate::utils::with_tempfile(
                r#"
        (defc test-nia-name-1 1)
        (defc test-nia-name-2 2)
        (export test-nia-name-1)
        (export test-nia-name-2)"#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();
                    let module_symbol_1 = interpreter.intern("test-nia-name-1");
                    let module_symbol_2 = interpreter.intern("test-nia-name-2");
                    let export_symbol_1 = interpreter.intern("test-nia-exported-name-1");
                    let export_symbol_2 = interpreter.intern("test-nia-exported-name-2");
                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    let main_environment_id = interpreter.get_main_environment_id();

                    let object_id = interpreter.make_object();
                    interpreter.set_object_property(
                        object_id,
                        module_symbol_1,
                        export_symbol_1.into(),
                    );
                    interpreter.set_object_property(
                        object_id,
                        module_symbol_2,
                        export_symbol_2.into(),
                    );

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportObject(object_id, module_path),
                        None,
                        vec![(export_symbol_1, value_1), (export_symbol_2, value_2)],
                    );
                },
            );
        }

        #[test]
        fn evaluates_reexport_object_as_named_correctly() {
            crate::utils::with_tempfile(
                r#"
        (defc test-nia-name-1 1)
        (defc test-nia-name-2 2)
        (export test-nia-name-1)
        (export test-nia-name-2)"#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();

                    let object_name_symbol = interpreter.intern("test-nia-object");
                    let module_symbol_1 = interpreter.intern("test-nia-name-1");
                    let module_symbol_2 = interpreter.intern("test-nia-name-2");
                    let export_symbol_1 = interpreter.intern("test-nia-exported-name-1");
                    let export_symbol_2 = interpreter.intern("test-nia-exported-name-2");
                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    let main_environment_id = interpreter.get_main_environment_id();

                    let object_id = interpreter.make_object();
                    interpreter.set_object_property(
                        object_id,
                        module_symbol_1,
                        export_symbol_1.into(),
                    );
                    interpreter.set_object_property(
                        object_id,
                        module_symbol_2,
                        export_symbol_2.into(),
                    );

                    let expected_object_id = interpreter.make_object();
                    interpreter.set_object_property(expected_object_id, export_symbol_1, value_1);
                    interpreter.set_object_property(expected_object_id, export_symbol_2, value_2);

                    let expected_object_value = expected_object_id.into();

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportObjectAsNamed(
                            object_id,
                            object_name_symbol,
                            module_path,
                        ),
                        None,
                        vec![(object_name_symbol, expected_object_value)],
                    );
                },
            );
        }

        #[test]
        fn evaluates_reexport_object_as_default_correctly() {
            crate::utils::with_tempfile(
                r#"
        (defc test-nia-name-1 1)
        (defc test-nia-name-2 2)
        (export test-nia-name-1)
        (export test-nia-name-2)"#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment_id = interpreter.get_main_environment_id();

                    let object_name_symbol = interpreter.intern("test-nia-object");
                    let module_symbol_1 = interpreter.intern("test-nia-name-1");
                    let module_symbol_2 = interpreter.intern("test-nia-name-2");
                    let export_symbol_1 = interpreter.intern("test-nia-exported-name-1");
                    let export_symbol_2 = interpreter.intern("test-nia-exported-name-2");
                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    let main_environment_id = interpreter.get_main_environment_id();

                    let object_id = interpreter.make_object();
                    interpreter.set_object_property(
                        object_id,
                        module_symbol_1,
                        export_symbol_1.into(),
                    );
                    interpreter.set_object_property(
                        object_id,
                        module_symbol_2,
                        export_symbol_2.into(),
                    );

                    let expected_object_id = interpreter.make_object();
                    interpreter.set_object_property(expected_object_id, export_symbol_1, value_1);
                    interpreter.set_object_property(expected_object_id, export_symbol_2, value_2);

                    let expected_object_value = expected_object_id.into();

                    assert_export_type_evaluated_correctly(
                        &mut interpreter,
                        ExportType::ReexportObjectAsDefault(object_id, module_path),
                        Some(expected_object_value),
                        vec![],
                    );
                },
            );
        }
    }
}

pub fn export(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let evaluated_values = evaluate_values(interpreter, environment_id, values)?;

    let export_type = read_export_type::read_export_type(interpreter, evaluated_values)?;

    // todo: resolve relative module path
    eval_export::eval_export(interpreter, environment_id, export_type)?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;
    use crate::utils::assertion;

    #[test]
    #[ignore]
    fn exports_correctly() {
        let mut specs = vec![
            (
                r#""#,
                r#"(defc name 1)
                   (export name)"#,
                r#"(import #{:name} from "./module2.nia")"#,
                r#"name"#,
                r#"1"#,
            ),
            (
                r#""#,
                r#"(defc name 1)
                   (export name as name-2)"#,
                r#"(import #{:name-2} from "./module2.nia")"#,
                r#"name-2"#,
                r#"1"#,
            ),
            (
                r#""#,
                r#"(defc name 1)
                   (export name as default)"#,
                r#"(import name from "./module2.nia")"#,
                r#"name"#,
                r#"1"#,
            ),
            (
                r#""#,
                r#"(defc name-1 1)
                   (defc name-2 2)
                   (export {:name-1 'exported-name-1 :name-2 'exported-name-2})"#,
                r#"(import #{:exported-name-1 :exported-name-2} from "./module2.nia")"#,
                r#"(list exported-name-1 exported-name-2)"#,
                r#"(list 1 2)"#,
            ),
            (
                r#""#,
                r#"(defc name 1)
                   (export {:name 'name-2} as obj)"#,
                r#"(import #{:obj} from "./module2.nia")"#,
                r#"obj:name-2"#,
                r#"1"#,
            ),
            (
                r#""#,
                r#"(defc name 1)
                   (export {:name 'name-2} as default)"#,
                r#"(import obj from "./module2.nia")"#,
                r#"obj:name-2"#,
                r#"1"#,
            ),
            (
                r#"(defc name 1)
                   (export name)"#,
                r#"(export name from "./module1.nia")"#,
                r#"(import #{:name} from "./module2.nia")"#,
                r#"name"#,
                r#"1"#,
            ),
            (
                r#"(defc name 1)
                   (export name as default)"#,
                r#"(export default from "./module1.nia")"#,
                r#"(import name from "./module2.nia")"#,
                r#"name"#,
                r#"1"#,
            ),
            (
                r#"(defc name 1)
                   (export name)"#,
                r#"(export name as name-2 from "./module1.nia")"#,
                r#"(import #{:name-2} from "./module2.nia")"#,
                r#"name-2"#,
                r#"1"#,
            ),
            (
                r#"(defc name 1)
                   (export name)"#,
                r#"(export name as default from "./module1.nia")"#,
                r#"(import name from "./module2.nia")"#,
                r#"name"#,
                r#"1"#,
            ),
            (
                r#"(defc name 1)
                   (export name)"#,
                r#"(export * from "./module1.nia")"#,
                r#"(import #{:name} from "./module2.nia")"#,
                r#"name"#,
                r#"1"#,
            ),
            (
                r#"(defc name 1)
                   (export name)"#,
                r#"(export * as obj from "./module1.nia")"#,
                r#"(import #{:obj} from "./module2.nia")"#,
                r#"obj:name"#,
                r#"1"#,
            ),
            (
                r#"(defc name 1)
                   (export name)"#,
                r#"(export * as default from "./module1.nia")"#,
                r#"(import obj from "./module2.nia")"#,
                r#"obj:name"#,
                r#"1"#,
            ),
            (
                r#"(defc name-1 1)
                   (defc name-2 2)
                   (export name-1)
                   (export name-2)"#,
                r#"(export {:name-1 'exported-name-1 :name-2 'exported-name-2} from "./module1.nia")"#,
                r#"(import #{:exported-name-1 :exported-name-2} from "./module2.nia")"#,
                r#"(list exported-name-1 exported-name-2)"#,
                r#"(list 1 2)"#,
            ),
            (
                r#"(defc name-1 1)
                   (defc name-2 2)
                   (export name-1)
                   (export name-2)"#,
                r#"(export {:name-1 'exported-name-1 :name-2 'exported-name-2} as obj from "./module1.nia")"#,
                r#"(import #{:obj} from "./module2.nia")"#,
                r#"(list obj:exported-name-1 obj:exported-name-2)"#,
                r#"(list 1 2)"#,
            ),
            (
                r#"(defc name-1 1)
                   (defc name-2 2)
                   (export name-1)
                   (export name-2)"#,
                r#"(export {:name-1 'exported-name-1 :name-2 'exported-name-2} as default from "./module1.nia")"#,
                r#"(import obj from "./module2.nia")"#,
                r#"(list obj:exported-name-1 obj:exported-name-2)"#,
                r#"(list 1 2)"#,
            ),
        ];

        for spec in specs {
            let module_1_content = spec.0;
            let module_2_content = spec.1;
            let import_code = spec.2;
            let result_code = spec.3;
            let expected_code = spec.3;
            println!("{:?}", spec);

            utils::with_tempdir(|directory| {
                utils::with_named_file(&directory, "module1.nia", module_1_content, || {
                    utils::with_named_file(&directory, "module2.nia", module_2_content, || {
                        utils::with_working_directory(&directory, || {
                            let mut interpreter = Interpreter::new();

                            interpreter.execute_in_main_environment(import_code)
                                .unwrap();

                            let result = interpreter.execute_in_main_environment(result_code)
                                .unwrap();
                            let expected = interpreter.execute_in_main_environment(expected_code)
                                .unwrap();

                            assertion::assert_deep_equal(
                                &mut interpreter,
                                expected,
                                result,
                            );
                        })
                    });
                });
            });
        }
    }

    #[test]
    #[ignore]
    fn respects_relative_paths() {
        utils::with_tempdir(|directory| {
            utils::with_working_directory(&directory, || {
                utils::with_named_file(&directory, "first.nia", "(export default from \"./first/second.nia\")", || {
                    utils::with_named_dir(&directory, "first", |directory| {
                        utils::with_named_file(&directory, "second.nia", "(export default from \"./third.nia\")", || {
                            utils::with_named_file(&directory, "third.nia", "(defc nya 1) (export nya as default)", || {
                                let mut interpreter = Interpreter::new();

                                interpreter.execute_in_main_environment("(import a from \"./first.nia\")")
                                    .unwrap();
                                let result = interpreter.execute_in_main_environment("a").unwrap();
                                let expected = Value::Integer(1);

                                assertion::assert_deep_equal(&mut interpreter, expected, result);
                            })
                        })
                    })
                })
            })
        })
    }
}
