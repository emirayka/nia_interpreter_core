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