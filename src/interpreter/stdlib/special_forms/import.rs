use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::Value;
use crate::{ObjectId, SymbolId};

use crate::library;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportType {
    Import(String),
    ImportDefault(SymbolId, String),
    ImportObject(ObjectId, String),
    ImportObjectAsNamed(ObjectId, SymbolId, String),
    ImportAll(String),
    ImportAllAsNamed(SymbolId, String),
}

fn deep_equal_import_type(
    interpreter: &mut Interpreter,
    v1: ImportType,
    v2: ImportType,
) -> Result<bool, Error> {
    use ImportType::*;

    let result = match (v1, v2) {
        (ImportDefault(s1, string1), ImportDefault(s2, string2)) => {
            library::deep_equal(interpreter, s1.into(), s2.into())? && string1 == string2
        }
        (ImportObject(o1, string1), ImportObject(o2, string2)) => {
            library::deep_equal(interpreter, o1.into(), o2.into())? && string1 == string2
        }
        (ImportObjectAsNamed(o1, s1, string1), ImportObjectAsNamed(o2, s2, string2)) => {
            string1 == string2
                && library::deep_equal(interpreter, o1.into(), o2.into())?
                && library::deep_equal(interpreter, s1.into(), s2.into())?
        }
        (ImportAllAsNamed(s1, string1), ImportAllAsNamed(s2, string2)) => {
            library::deep_equal(interpreter, s1.into(), s2.into())? && string1 == string2
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
                        "Special form `import' takes as arguments only s-expressions that evaluates to objects, symbols and strings."
                    ).into();
                }
            }
            Value::Symbol(_) => value,
            Value::String(_) => value,
            _ => return Error::invalid_argument_error(
                "Special form `import' takes as arguments only s-expressions that evaluates to objects, symbols and strings."
            ).into()
        };

        evaluated_values.push(evaluated_value);
    }

    Ok(evaluated_values)
}

mod read_import_type {
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

    fn try_read_import(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ImportType, Error> {
        if values.len() != 1 {
            return Error::generic_execution_error("").into();
        }

        let module_path = library::read_as_string(interpreter, values[0])?.clone();

        let module_path = interpreter.resolve_with_current_module_path(
            module_path
        )?;

        Ok(ImportType::Import(module_path))
    }

    fn try_read_import_default(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ImportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let import_name_symbol_id = library::read_as_symbol_id(values[0])?;
        let from_symbol_id = library::read_as_symbol_id(values[1])?;
        let module_path = library::read_as_string(interpreter, values[2])?.clone();

        let module_path = interpreter.resolve_with_current_module_path(
            module_path
        )?;

        check_interned_symbol_is_not_asterisk(interpreter, import_name_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        Ok(ImportType::ImportDefault(
            import_name_symbol_id,
            module_path,
        ))
    }

    fn try_read_import_object(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ImportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let import_object_id = library::read_as_object_id(values[0])?;
        let from_symbol_id = library::read_as_symbol_id(values[1])?;
        let module_path = library::read_as_string(interpreter, values[2])?.clone();

        let module_path = interpreter.resolve_with_current_module_path(
            module_path
        )?;

        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        Ok(ImportType::ImportObject(import_object_id, module_path))
    }

    fn try_read_import_object_as_named(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ImportType, Error> {
        if values.len() != 5 {
            return Error::generic_execution_error("").into();
        }

        let import_object_id = library::read_as_object_id(values[0])?;
        let as_symbol_id = library::read_as_symbol_id(values[1])?.clone();
        let import_name_symbol_id = library::read_as_symbol_id(values[2])?.clone();
        let from_symbol_id = library::read_as_symbol_id(values[3])?;
        let module_path = library::read_as_string(interpreter, values[4])?.clone();

        let module_path = interpreter.resolve_with_current_module_path(
            module_path
        )?;

        check_interned_symbol_is_as(interpreter, as_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        Ok(ImportType::ImportObjectAsNamed(
            import_object_id,
            import_name_symbol_id,
            module_path,
        ))
    }

    fn try_read_import_all(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ImportType, Error> {
        if values.len() != 3 {
            return Error::generic_execution_error("").into();
        }

        let asterisk_symbol_id = library::read_as_symbol_id(values[0])?;
        let from_symbol_id = library::read_as_symbol_id(values[1])?;
        let module_path = library::read_as_string(interpreter, values[2])?.clone();

        let module_path = interpreter.resolve_with_current_module_path(
            module_path
        )?;

        check_interned_symbol_is_asterisk(interpreter, asterisk_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        Ok(ImportType::ImportAll(module_path))
    }

    fn try_read_import_all_as_named(
        interpreter: &mut Interpreter,
        values: &Vec<Value>,
    ) -> Result<ImportType, Error> {
        if values.len() != 5 {
            return Error::generic_execution_error("").into();
        }

        let asterisk_symbol_id = library::read_as_symbol_id(values[0])?;
        let as_symbol_id = library::read_as_symbol_id(values[1])?;
        let import_name_symbol_id = library::read_as_symbol_id(values[2])?.clone();
        let from_symbol_id = library::read_as_symbol_id(values[3])?;
        let module_path = library::read_as_string(interpreter, values[4])?.clone();

        let module_path = interpreter.resolve_with_current_module_path(
            module_path
        )?;

        check_interned_symbol_is_asterisk(interpreter, asterisk_symbol_id)?;
        check_interned_symbol_is_as(interpreter, as_symbol_id)?;
        check_interned_symbol_is_from(interpreter, from_symbol_id)?;

        Ok(ImportType::ImportAllAsNamed(
            import_name_symbol_id,
            module_path,
        ))
    }

    fn read_import_type_runner(
        funcs: Vec<fn(&mut Interpreter, &Vec<Value>) -> Result<ImportType, Error>>,
        interpreter: &mut Interpreter,
        values: Vec<Value>,
    ) -> Result<ImportType, Error> {
        for func in funcs {
            match func(interpreter, &values) {
                Ok(import_type) => return Ok(import_type),
                Err(error) => {
                    if error.is_failure() {
                        return Err(error);
                    }
                }
            }
        }

        Error::generic_execution_error("Cannot parse import.").into()
    }

    pub fn read_import_type(
        interpreter: &mut Interpreter,
        values: Vec<Value>,
    ) -> Result<ImportType, Error> {
        read_import_type_runner(
            vec![
                try_read_import,
                try_read_import_default,
                try_read_import_object,
                try_read_import_object_as_named,
                try_read_import_all,
                try_read_import_all_as_named,
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
        fn reads_import_types_correctly() {
            let mut interpreter = Interpreter::new();

            let name_symbol_id = interpreter.intern("name");
            let name_1_symbol_id = interpreter.intern("name-1");
            let name_2_symbol_id = interpreter.intern("name-2");
            let imported_name_1_symbol_id = interpreter.intern("imported-name-1");
            let imported_name_2_symbol_id = interpreter.intern("imported-name-2");
            let module_string = String::from("./module");

            let expected_object_id_1 = interpreter.make_object();
            interpreter
                .set_object_property(
                    expected_object_id_1,
                    name_1_symbol_id,
                    name_1_symbol_id.into(),
                )
                .unwrap();
            interpreter
                .set_object_property(
                    expected_object_id_1,
                    name_2_symbol_id,
                    name_2_symbol_id.into(),
                )
                .unwrap();

            let expected_object_id_2 = interpreter.make_object();
            interpreter
                .set_object_property(
                    expected_object_id_2,
                    name_1_symbol_id,
                    imported_name_1_symbol_id.into(),
                )
                .unwrap();
            interpreter
                .set_object_property(
                    expected_object_id_2,
                    name_2_symbol_id,
                    imported_name_2_symbol_id.into(),
                )
                .unwrap();

            let specs = vec!(
                (ImportType::Import(module_string.clone()), "(list \"./module\")"),
                (ImportType::ImportDefault(name_symbol_id, module_string.clone()), "(list 'name 'from \"./module\")"),
                (ImportType::ImportObject(expected_object_id_1, module_string.clone()), "(list #{:name-1 :name-2} 'from \"./module\")"),
                (ImportType::ImportObject(expected_object_id_2, module_string.clone()), "(list {:name-1 'imported-name-1 :name-2 'imported-name-2} 'from \"./module\")"),
                (ImportType::ImportObjectAsNamed(expected_object_id_1, name_symbol_id, module_string.clone()), "(list #{:name-1 :name-2} 'as 'name 'from \"./module\")"),
                (ImportType::ImportObjectAsNamed(expected_object_id_2, name_symbol_id, module_string.clone()), "(list {:name-1 'imported-name-1 :name-2 'imported-name-2} 'as 'name 'from \"./module\")"),
                (ImportType::ImportAll(module_string.clone()), "(list '* 'from \"./module\")"),
                (ImportType::ImportAllAsNamed(name_symbol_id, module_string.clone()), "(list '* 'as 'name 'from \"./module\")"),
            );

            for (expected, code) in specs {
                let args = interpreter.execute_in_main_environment(code).unwrap();
                let cons_id: ConsId = args.try_into().unwrap();
                let args = interpreter.list_to_vec(cons_id).unwrap();

                let result = read_import_type(&mut interpreter, args.clone()).unwrap();

                nia_assert(deep_equal_import_type(&mut interpreter, expected, result).unwrap());
            }
        }

        // todo: add negative tests here
        // that would be hard 😑
    }
}

mod eval_import {
    use super::*;
    use crate::ModuleId;

    enum Import {
        Named(SymbolId, Value),
    }

    fn read_module_variable_import_variable_vector(
        interpreter: &mut Interpreter,
        object_id: ObjectId,
    ) -> Result<Vec<(SymbolId, SymbolId)>, Error> {
        let mut result = Vec::new();
        let object = interpreter.get_object(object_id)?;

        for (module_variable_symbol_id, object_value_wrapper) in object.get_properties() {
            let value = object_value_wrapper.get_value()?;
            let import_variable_symbol_id = library::read_as_symbol_id(value)?;

            result.push((*module_variable_symbol_id, import_variable_symbol_id));
        }

        Ok(result)
    }

    fn read_exports_from_module(
        interpreter: &mut Interpreter,
        object_id: ObjectId,
        module_id: ModuleId,
    ) -> Result<Vec<(SymbolId, Value)>, Error> {
        let bindings = read_module_variable_import_variable_vector(interpreter, object_id)?;

        let mut result = Vec::new();
        let module = interpreter.get_module_required_soft(module_id)?;

        for (module_variable_symbol_id, import_variable_symbol_id) in bindings {
            let module_variable_value = module
                .get_export(module_variable_symbol_id)
                .ok_or_else(|| Error::generic_execution_error("Module has no export."))?;

            result.push((import_variable_symbol_id, module_variable_value));
        }

        Ok(result)
    }

    fn make_object_from_exports(
        interpreter: &mut Interpreter,
        exports: Vec<(SymbolId, Value)>,
    ) -> Result<Value, Error> {
        let object_id = interpreter.make_object();
        let mut object = interpreter.get_object(object_id)?;

        for (variable_symbol_id, value) in exports {
            interpreter.set_object_property(object_id, variable_symbol_id, value)?;
        }

        Ok(object_id.into())
    }

    fn read_exports_from_module_as_object(
        interpreter: &mut Interpreter,
        object_id: ObjectId,
        module_id: ModuleId,
    ) -> Result<Value, Error> {
        let exports = read_exports_from_module(interpreter, object_id, module_id)?;

        make_object_from_exports(interpreter, exports)
    }

    fn read_all_exports_from_module(
        interpreter: &mut Interpreter,
        module_id: ModuleId,
    ) -> Result<Vec<(SymbolId, Value)>, Error> {
        let module = interpreter.get_module_required_soft(module_id)?;

        let mut result = Vec::new();

        for (symbol_id, value) in module.get_exports() {
            result.push((*symbol_id, *value));
        }

        Ok(result)
    }

    fn read_importation_vector(
        interpreter: &mut Interpreter,
        current_module_environment: EnvironmentId,
        import_type: ImportType,
    ) -> Result<Vec<Import>, Error> {
        let importation_vector = match import_type {
            ImportType::Import(module_path) => {
                interpreter.intern_module(&module_path)?;

                Vec::new()
            }
            ImportType::ImportDefault(variable_symbol_id, module_path) => {
                let module_id = interpreter.intern_module(&module_path)?;
                let module = interpreter.get_module_required_soft(module_id)?;

                let default_export = match module.get_default_export() {
                    Some(default_export) => default_export,
                    None => {
                        return Error::generic_execution_error("Module has no default export.")
                            .into()
                    }
                };

                vec![Import::Named(variable_symbol_id, default_export)]
            }
            ImportType::ImportObject(object_id, module_path) => {
                let module_id = interpreter.intern_module(&module_path)?;

                read_exports_from_module(interpreter, object_id, module_id)?
                    .into_iter()
                    .map(|(symbol_id, value)| Import::Named(symbol_id, value))
                    .collect()
            }
            ImportType::ImportObjectAsNamed(object_id, variable_symbol_id, module_path) => {
                let module_id = interpreter.intern_module(&module_path)?;

                let object_id =
                    read_exports_from_module_as_object(interpreter, object_id, module_id)?;

                vec![Import::Named(variable_symbol_id, object_id)]
            }
            ImportType::ImportAll(module_path) => {
                let module_id = interpreter.intern_module(&module_path)?;

                read_all_exports_from_module(interpreter, module_id)?
                    .into_iter()
                    .map(|(variable_symbol_id, value)| Import::Named(variable_symbol_id, value))
                    .collect()
            }
            ImportType::ImportAllAsNamed(variable_symbol_id, module_path) => {
                let module_id = interpreter.intern_module(&module_path)?;

                let exports = read_all_exports_from_module(interpreter, module_id)?;

                let object_id = make_object_from_exports(interpreter, exports)?;

                vec![Import::Named(variable_symbol_id, object_id)]
            }
        };

        Ok(importation_vector)
    }

    pub fn eval_import(
        interpreter: &mut Interpreter,
        environment_id: EnvironmentId,
        import_type: ImportType,
    ) -> Result<(), Error> {
        let importation_vector = read_importation_vector(interpreter, environment_id, import_type)?;

        for import in importation_vector {
            match import {
                Import::Named(symbol_id, value) => {
                    interpreter.define_const_variable(environment_id, symbol_id, value)?;
                }
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

        #[test]
        fn evaluates_import_correctly() {
            crate::utils::with_tempfile(
                r#"
            (set! kek 2)
            "#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let root_environment = interpreter.get_root_environment_id();
                    let main_environment = interpreter.get_main_environment_id();
                    let variable_symbol_id = interpreter.intern("kek");
                    let variable_value = Value::Integer(1);
                    let expected_value = Value::Integer(2);

                    interpreter
                        .define_variable(root_environment, variable_symbol_id, variable_value)
                        .unwrap();

                    eval_import(
                        &mut interpreter,
                        main_environment,
                        ImportType::Import(module_path),
                    )
                    .unwrap();

                    let result_value = interpreter
                        .lookup_variable(root_environment, variable_symbol_id)
                        .unwrap()
                        .unwrap();

                    assertion::assert_deep_equal(&mut interpreter, expected_value, result_value);
                },
            );
        }

        #[test]
        fn evaluates_import_default_correctly() {
            crate::utils::with_tempfile(
                r#"
            (defc name 1)
            (export name as default)
            "#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment = interpreter.get_main_environment_id();
                    let variable_symbol_id = interpreter.intern("name");

                    eval_import(
                        &mut interpreter,
                        main_environment,
                        ImportType::ImportDefault(variable_symbol_id, module_path),
                    )
                    .unwrap();

                    let result_value = interpreter
                        .lookup_variable(main_environment, variable_symbol_id)
                        .unwrap()
                        .unwrap();

                    let expected_value = Value::Integer(1);
                    assertion::assert_deep_equal(&mut interpreter, expected_value, result_value);
                },
            );
        }

        #[test]
        fn evaluates_import_object_correctly() {
            crate::utils::with_tempfile(
                r#"
            (defc name-1 1)
            (defc name-2 2)
            (export name-1)
            (export name-2)
            "#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment = interpreter.get_main_environment_id();
                    let name_1_symbol_id = interpreter.intern("name-1");
                    let name_2_symbol_id = interpreter.intern("name-2");
                    let imported_name_1_symbol_id = interpreter.intern("imported-name-1");
                    let imported_name_2_symbol_id = interpreter.intern("imported-name-2");

                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    let object_id = interpreter.make_object();

                    interpreter
                        .set_object_property(
                            object_id,
                            name_1_symbol_id,
                            imported_name_1_symbol_id.into(),
                        )
                        .unwrap();
                    interpreter
                        .set_object_property(
                            object_id,
                            name_2_symbol_id,
                            imported_name_2_symbol_id.into(),
                        )
                        .unwrap();

                    eval_import(
                        &mut interpreter,
                        main_environment,
                        ImportType::ImportObject(object_id, module_path),
                    )
                    .unwrap();

                    let result_1 = interpreter
                        .lookup_variable(main_environment, imported_name_1_symbol_id)
                        .unwrap()
                        .unwrap();

                    let result_2 = interpreter
                        .lookup_variable(main_environment, imported_name_2_symbol_id)
                        .unwrap()
                        .unwrap();

                    assertion::assert_deep_equal(&mut interpreter, value_1, result_1);

                    assertion::assert_deep_equal(&mut interpreter, value_2, result_2);
                },
            );
        }

        #[test]
        fn evaluates_import_object_as_named_correctly() {
            crate::utils::with_tempfile(
                r#"
            (defc name-1 1)
            (defc name-2 2)
            (export name-1)
            (export name-2)
            "#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment = interpreter.get_main_environment_id();
                    let name_symbol_id = interpreter.intern("name");
                    let name_1_symbol_id = interpreter.intern("name-1");
                    let name_2_symbol_id = interpreter.intern("name-2");
                    let imported_name_1_symbol_id = interpreter.intern("imported-name-1");
                    let imported_name_2_symbol_id = interpreter.intern("imported-name-2");

                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    let object_id = interpreter.make_object();

                    interpreter
                        .set_object_property(
                            object_id,
                            name_1_symbol_id,
                            imported_name_1_symbol_id.into(),
                        )
                        .unwrap();
                    interpreter
                        .set_object_property(
                            object_id,
                            name_2_symbol_id,
                            imported_name_2_symbol_id.into(),
                        )
                        .unwrap();

                    let expected_object_id = interpreter.make_object();

                    interpreter
                        .set_object_property(expected_object_id, imported_name_1_symbol_id, value_1)
                        .unwrap();
                    interpreter
                        .set_object_property(expected_object_id, imported_name_2_symbol_id, value_2)
                        .unwrap();

                    let expected_object_value = expected_object_id.into();

                    eval_import(
                        &mut interpreter,
                        main_environment,
                        ImportType::ImportObjectAsNamed(object_id, name_symbol_id, module_path),
                    )
                    .unwrap();

                    let result = interpreter
                        .lookup_variable(main_environment, name_symbol_id)
                        .unwrap()
                        .unwrap();

                    assertion::assert_deep_equal(&mut interpreter, expected_object_value, result);
                },
            );
        }

        #[test]
        fn evaluates_import_all_correctly() {
            crate::utils::with_tempfile(
                r#"
            (defc name-1 1)
            (defc name-2 2)
            (export name-1)
            (export name-2)
            "#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment = interpreter.get_main_environment_id();
                    let name_1_symbol_id = interpreter.intern("name-1");
                    let name_2_symbol_id = interpreter.intern("name-2");

                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    eval_import(
                        &mut interpreter,
                        main_environment,
                        ImportType::ImportAll(module_path),
                    )
                    .unwrap();

                    let result_1 = interpreter
                        .lookup_variable(main_environment, name_1_symbol_id)
                        .unwrap()
                        .unwrap();

                    let result_2 = interpreter
                        .lookup_variable(main_environment, name_2_symbol_id)
                        .unwrap()
                        .unwrap();

                    assertion::assert_deep_equal(&mut interpreter, value_1, result_1);

                    assertion::assert_deep_equal(&mut interpreter, value_2, result_2);
                },
            );
        }

        #[test]
        fn evaluates_import_all_as_named_correctly() {
            crate::utils::with_tempfile(
                r#"
            (defc name-1 1)
            (defc name-2 2)
            (export name-1)
            (export name-2)
            "#,
                |module_path| {
                    let mut interpreter = Interpreter::new();

                    let main_environment = interpreter.get_main_environment_id();
                    let name_symbol_id = interpreter.intern("name");
                    let name_1_symbol_id = interpreter.intern("name-1");
                    let name_2_symbol_id = interpreter.intern("name-2");

                    let value_1 = Value::Integer(1);
                    let value_2 = Value::Integer(2);

                    let expected_object_id = interpreter.make_object();

                    interpreter
                        .set_object_property(expected_object_id, name_1_symbol_id, value_1)
                        .unwrap();
                    interpreter
                        .set_object_property(expected_object_id, name_2_symbol_id, value_2)
                        .unwrap();

                    eval_import(
                        &mut interpreter,
                        main_environment,
                        ImportType::ImportAllAsNamed(name_symbol_id, module_path),
                    )
                    .unwrap();

                    let expected_object_value = expected_object_id.into();

                    let result = interpreter
                        .lookup_variable(main_environment, name_symbol_id)
                        .unwrap()
                        .unwrap();

                    assertion::assert_deep_equal(&mut interpreter, expected_object_value, result);
                },
            );
        }
    }
}

pub fn import(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let evaluated_values = evaluate_values(interpreter, environment_id, values)?;

    let import_type = read_import_type::read_import_type(interpreter, evaluated_values)?;

    // todo: resolve relative module path
    eval_import::eval_import(interpreter, environment_id, import_type)?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use crate::utils::assertion;

    #[test]
    #[ignore]
    fn imports_correctly() {
        let mut specs = vec![
            (
                r#"(defv nia-test 1)"#,
                r#""#,
                r#"(set! nia-test 2)"#,
                r#"(import "./module2.nia")"#,
                r#"nia-test"#,
                r#"2"#,
            ),
            (
                r#""#,
                r#""#,
                r#"(defc name 1)
                   (export name as default)"#,
                r#"(import name from "./module2.nia")"#,
                r#"name"#,
                r#"1"#,
            ),
            (
                r#""#,
                r#""#,
                r#"(defc name-1 1)
                   (defc name-2 2)
                   (export #{:name-1 :name-2})"#,
                r#"(import {:name-1 'imported-name-1 :name-2 'imported-name-2} from "./module2.nia")"#,
                r#"(list imported-name-1 imported-name-2)"#,
                r#"(list 1 2)"#,
            ),
            (
                r#""#,
                r#""#,
                r#"(defc name-1 1)
                   (defc name-2 2)
                   (export #{:name-1 :name-2})"#,
                r#"(import {:name-1 'imported-name-1 :name-2 'imported-name-2} as obj from "./module2.nia")"#,
                r#"(list obj:imported-name-1 obj:imported-name-2)"#,
                r#"(list 1 2)"#,
            ),
            (
                r#""#,
                r#""#,
                r#"(defc name-1 1)
                   (defc name-2 2)
                   (export #{:name-1 :name-2})"#,
                r#"(import * from "./module2.nia")"#,
                r#"(list name-1 name-2)"#,
                r#"(list 1 2)"#,
            ),
            (
                r#""#,
                r#""#,
                r#"(defc name-1 1)
                   (defc name-2 2)
                   (export #{:name-1 :name-2})"#,
                r#"(import * as obj from "./module2.nia")"#,
                r#"(list obj:name-1 obj:name-2)"#,
                r#"(list 1 2)"#,
            ),
        ];

        for spec in specs {
            let root_code = spec.0;
            let module_1_content = spec.1;
            let module_2_content = spec.2;
            let import_code = spec.3;
            let result_code = spec.4;
            let expected_code = spec.5;
            println!("{:?}", spec);

            utils::with_tempdir(|directory| {
                utils::with_named_file(&directory, "module1.nia", module_1_content, || {
                    utils::with_named_file(&directory, "module2.nia", module_2_content, || {
                        utils::with_working_directory(&directory, || {
                            let mut interpreter = Interpreter::new();

                            interpreter.execute_in_root_environment(root_code)
                                .unwrap();

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
                utils::with_named_file(&directory, "first.nia", "(import nya from \"./first/second.nia\") (export nya as default)", || {
                    utils::with_named_dir(&directory, "first", |directory| {
                        utils::with_named_file(&directory, "second.nia", "(import nya from \"./third.nia\") (export nya as default)", || {
                            utils::with_named_file(&directory, "third.nia", "(defc nya 1) (export nya as default)", || {
                                let mut interpreter = Interpreter::new();

                                interpreter.execute_in_main_environment("(import nya from \"./first.nia\")")
                                    .unwrap();
                                let result = interpreter.execute_in_main_environment("nya").unwrap();
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
