use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn collect_garbage(interpreter: &mut Interpreter) -> Result<(), Error> {
    // these would be ignored
    let mut ignored_environment_ids = Vec::new();

    // these are candidates for deletion
    let mut candidate_environment_ids = interpreter.get_environment_arena().get_all_environments();

    let mut candidate_cons_ids = interpreter.get_cons_arena().get_all_cons_identifiers();
    let mut candidate_function_ids = interpreter.get_function_arena().get_all_function_identifiers();
    let mut candidate_object_ids = interpreter.get_object_arena().get_all_object_identifiers();

    let mut candidate_keyword_ids = interpreter.get_keyword_arena().get_all_keyword_identifiers();
    let mut candidate_string_ids = interpreter.get_string_arena().get_all_string_identifiers();
    let mut candidate_symbol_ids = interpreter.get_symbol_arena().get_all_symbol_identifiers();

    // remove which should be persisted
    for symbol_id in interpreter.get_ignored_symbols() {
        candidate_symbol_ids.retain(|id| *id != symbol_id);
    }

    for function_id in interpreter.get_ignored_functions() {
        candidate_function_ids.retain(|id| *id != function_id);
    }

    // iteration base
    let mut environment_ids = vec!(interpreter.get_root_environment());

    // iteration over accessible environments
    while !environment_ids.is_empty() {
        let environment_id = environment_ids.remove(0);

        candidate_environment_ids.retain(|id| *id != environment_id);

        ignored_environment_ids.push(environment_id);

        match interpreter.get_environment_arena().get_parent(environment_id)? {
            Some(parent_id) => {
                if !ignored_environment_ids.contains(&parent_id) {
                    environment_ids.push(parent_id)
                }
            },
            _ => {}
        }

        let mut items = interpreter.get_environment_items(environment_id)?;

        while !items.is_empty() {
            let item = items.remove(0);

            match item {
                Value::String(string_id) => {
                    candidate_string_ids.retain(|id| *id != string_id);
                },
                Value::Keyword(keyword_id) => {
                    candidate_keyword_ids.retain(|id| *id != keyword_id);
                },
                Value::Symbol(symbol_id) => {
                    candidate_symbol_ids.retain(|id| *id != symbol_id);
                },
                Value::Cons(cons_id) => {
                    candidate_cons_ids.retain(|id| *id != cons_id);

                    items.push(interpreter.get_car(cons_id)?);
                    items.push(interpreter.get_cdr(cons_id)?);
                },
                Value::Object(object_id) => {
                    candidate_object_ids.retain(|id| *id != object_id);

                    let mut gc_items = interpreter.get_object_arena().get_gc_items(object_id)?;

                    items.append(&mut gc_items);
                },
                Value::Function(function_id) => {
                    candidate_function_ids.retain(|id| *id != function_id);

                    match interpreter.get_function_arena().get_gc_items(function_id)? {
                        Some(mut gc_items) => items.append(&mut gc_items),
                        _ => {}
                    }

                    match interpreter.get_function_arena().get_gc_environment(function_id)? {
                        Some(gc_environment_id) => {
                            if !ignored_environment_ids.contains(&gc_environment_id) {
                                environment_ids.push(gc_environment_id)
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    // free garbage
    interpreter.free_environments(candidate_environment_ids);
    interpreter.free_strings(candidate_string_ids);
    interpreter.free_keywords(candidate_keyword_ids);
    interpreter.free_symbols(candidate_symbol_ids);
    interpreter.free_cons_cells(candidate_cons_ids);
    interpreter.free_objects(candidate_object_ids);
    interpreter.free_functions(candidate_function_ids);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::function::{Function, BuiltinFunction, SpecialFormFunction};
    use crate::interpreter::environment::EnvironmentId;
    use crate::interpreter::function::Function::SpecialForm;

    fn builtin_test_function(
        _interpreter: &mut Interpreter,
        _environment_id: EnvironmentId,
        _values: Vec<Value>
    ) -> Result<Value, Error> {
        Ok(Value::Integer(1))
    }

    fn test_special_form(
        _interpreter: &mut Interpreter,
        _environment_id: EnvironmentId,
        _values: Vec<Value>
    ) -> Result<Value, Error> {
        Ok(Value::Integer(1))
    }

    #[test]
    fn collects_strings() {
        let mut interpreter = Interpreter::new();

        let string_id = interpreter.execute("\"string\"")
            .unwrap().as_string_id();

        assert!(interpreter.get_string(string_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_string(string_id).is_err());
    }

    #[test]
    fn collects_keywords() {
        let mut interpreter = Interpreter::new();

        let keyword_id = interpreter.execute(":some-unused-keyword")
            .unwrap().as_keyword_id();

        assert!(interpreter.get_keyword(keyword_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_keyword(keyword_id).is_err());
    }

    #[test]
    fn collects_symbols() {
        let mut interpreter = Interpreter::new();

        let symbol_id = interpreter.execute("'some-not-used-long-named-symbol")
            .unwrap().as_symbol_id();

        assert!(interpreter.get_symbol(symbol_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_symbol(symbol_id).is_err());
    }

    #[test]
    fn collects_cons_cells() {
        let mut interpreter = Interpreter::new();

        let cons_id = interpreter.execute("(cons 1 2)")
            .unwrap().as_cons_id();

        assert!(interpreter.get_car(cons_id).is_ok());
        assert!(interpreter.get_cdr(cons_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_car(cons_id).is_err());
        assert!(interpreter.get_cdr(cons_id).is_err());
    }

    #[test]
    fn collects_objects() {
        let mut interpreter = Interpreter::new();

        let object_id = interpreter.execute("{}")
            .unwrap().as_object_id();

        assert!(interpreter.get_object_proto(object_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_object_proto(object_id).is_err());
    }

    #[test]
    fn collects_builtin_functions() {
        let mut interpreter = Interpreter::new();

        let function = Function::Builtin(BuiltinFunction::new(builtin_test_function));
        let function_id = interpreter.register_function(function);

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_special_forms() {
        let mut interpreter = Interpreter::new();

        let function = Function::SpecialForm(SpecialFormFunction::new(test_special_form));
        let function_id = interpreter.register_function(function);

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_interpreted_functions() {
        let mut interpreter = Interpreter::new();

        let function_id = interpreter.execute("(fn () 1)").unwrap().as_function_id();

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_macro_functions() {
        let mut interpreter = Interpreter::new();

        let function_id = interpreter.execute("(function (macro () 1))").unwrap().as_function_id();

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(collect_garbage(&mut interpreter).is_ok());
        assert!(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn respects_cons_content() {
        let mut interpreter = Interpreter::new();

        let symbol_1 = interpreter.execute("'some-unused-symbol-1")
            .unwrap().as_symbol_id();
        let symbol_2 = interpreter.execute("'some-unused-symbol-2")
            .unwrap().as_symbol_id();

        let cons_id = interpreter.execute("(defv kekurus (cons 'some-unused-symbol-1 'some-unused-symbol-2)) kekurus")
            .unwrap().as_cons_id();

        assert!(interpreter.get_car(cons_id).is_ok());
        assert!(interpreter.get_cdr(cons_id).is_ok());
        assert!(interpreter.get_symbol(symbol_1).is_ok());
        assert!(interpreter.get_symbol(symbol_2).is_ok());

        assert!(collect_garbage(&mut interpreter).is_ok());

        assert!(interpreter.get_car(cons_id).is_ok());
        assert!(interpreter.get_cdr(cons_id).is_ok());
        assert!(interpreter.get_symbol(symbol_1).is_ok());
        assert!(interpreter.get_symbol(symbol_2).is_ok());
    }

    #[test]
    fn respects_object_contents() {
        let mut interpreter = Interpreter::new();

        let item_key = interpreter.execute(":some-unused-keyword-1")
            .unwrap().as_keyword_id();

        let item_key_symbol = interpreter.execute("'some-unused-keyword-1")
            .unwrap().as_symbol_id();

        let item_value = interpreter.execute(":some-unused-keyword-2")
            .unwrap();

        let object_id = interpreter.execute("(defv kekurus {:some-unused-keyword-1 :some-unused-keyword-2}) kekurus")
            .unwrap().as_object_id();

        assert!(interpreter.get_keyword(item_key).is_ok());
        assert!(interpreter.get_symbol(item_key_symbol).is_ok());
        assert!(interpreter.get_keyword(item_value.as_keyword_id()).is_ok());
        assert!(interpreter.get_object_proto(object_id).is_ok());

        assert!(collect_garbage(&mut interpreter).is_ok());

        assert!(interpreter.get_keyword(item_key).is_err());
        assert!(interpreter.get_symbol(item_key_symbol).is_ok());
        assert!(interpreter.get_keyword(item_value.as_keyword_id()).is_ok());
        assert!(interpreter.get_object_proto(object_id).is_ok());
    }

    #[test]
    fn respects_function_code() {
        let mut interpreter = Interpreter::new();

        let function_id = interpreter.execute(
            "(defv some-kekurus-variable (fn () 'some-unused-symbol)) some-kekurus-variable"
        ).unwrap().as_function_id();

        let symbol_id = interpreter.execute("'some-unused-symbol")
            .unwrap().as_symbol_id();

        assert!(collect_garbage(&mut interpreter).is_ok());

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(interpreter.get_symbol(symbol_id).is_ok());
    }

    #[test]
    fn respects_function_environment() {
        let mut interpreter = Interpreter::new();

        let symbol1 = interpreter.execute("'kekurus-1")
            .unwrap().as_symbol_id();

        let symbol2 = interpreter.execute("'kekurus-1")
            .unwrap().as_symbol_id();

        let function_id = interpreter.execute(
            "(defv kekurus (let ((kekurus-1 1) (kekurus-2 2)) (fn () (+ 1 2)))) kekurus"
        ).unwrap().as_function_id();

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(interpreter.get_symbol(symbol1).is_ok());
        assert!(interpreter.get_symbol(symbol2).is_ok());

        assert!(collect_garbage(&mut interpreter).is_ok());

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(interpreter.get_symbol(symbol1).is_ok());
        assert!(interpreter.get_symbol(symbol2).is_ok());
    }

    #[test]
    fn respects_function_arguments() {
        let mut interpreter = Interpreter::new();

        let symbol1 = interpreter.execute("'kekurus-arg-1")
            .unwrap().as_symbol_id();

        let symbol2 = interpreter.execute("'kekurus-arg-2")
            .unwrap().as_symbol_id();

        let function1 = interpreter.execute("(defv kekurus-1 (fn (#opt (a 'kekurus-arg-1)) 1)) kekurus-1")
            .unwrap().as_function_id();

        let function2 = interpreter.execute("(defv kekurus-2 (fn (#opt (a 'kekurus-arg-2)) 1)) kekurus-2")
            .unwrap().as_function_id();

        assert!(interpreter.get_function(function1).is_ok());
        assert!(interpreter.get_function(function2).is_ok());
        assert!(interpreter.get_symbol(symbol1).is_ok());
        assert!(interpreter.get_symbol(symbol2).is_ok());

        assert!(collect_garbage(&mut interpreter).is_ok());

        assert!(interpreter.get_function(function1).is_ok());
        assert!(interpreter.get_function(function2).is_ok());
        assert!(interpreter.get_symbol(symbol1).is_ok());
        assert!(interpreter.get_symbol(symbol2).is_ok());
    }
}