use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::value::ConsId;
use crate::interpreter::value::FunctionId;
use crate::interpreter::value::KeywordId;
use crate::interpreter::value::ObjectId;
use crate::interpreter::value::StringId;
use crate::interpreter::value::SymbolId;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait GarbageCollectable {
    fn get_gc_items(&self) -> Vec<Value>;
    fn get_gc_environments(&self) -> Vec<EnvironmentId>;
}

struct GarbageCollector {
    ignored_environment_ids: Vec<EnvironmentId>,
    candidate_environment_ids: Vec<EnvironmentId>,
    environment_ids: Vec<EnvironmentId>,

    candidate_string_ids: Vec<StringId>,
    candidate_keyword_ids: Vec<KeywordId>,
    candidate_symbol_ids: Vec<SymbolId>,

    candidate_cons_ids: Vec<ConsId>,
    candidate_object_ids: Vec<ObjectId>,
    candidate_function_ids: Vec<FunctionId>,

    items: Vec<Value>,
}

fn remove_value_from_vector<T>(vector: &mut Vec<T>, value: T)
where
    T: Copy + PartialEq + Eq,
{
    let mut i = 0;
    let length = vector.len();

    while i < length {
        if vector[i] == value {
            break;
        }
        i += 1;
    }

    if i < length {
        vector.remove(i);
    }
}

impl GarbageCollector {
    pub fn new(interpreter: &mut Interpreter) -> GarbageCollector {
        // these would be ignored
        let ignored_environment_ids = Vec::new();

        // these are candidates for deletion
        let candidate_environment_ids =
            interpreter.get_environment_arena().get_all_environments();

        let candidate_string_ids =
            interpreter.get_string_arena().get_all_string_identifiers();
        let candidate_keyword_ids = interpreter
            .get_keyword_arena()
            .get_all_keyword_identifiers();
        let mut candidate_symbol_ids =
            interpreter.get_symbol_arena().get_all_symbol_identifiers();

        let candidate_cons_ids =
            interpreter.get_cons_arena().get_all_cons_identifiers();
        let candidate_object_ids =
            interpreter.get_object_arena().get_all_object_identifiers();
        let mut candidate_function_ids = interpreter
            .get_function_arena()
            .get_all_function_identifiers();

        // remove which should be persisted
        for symbol_id in interpreter.get_ignored_symbols() {
            candidate_symbol_ids.retain(|id| *id != symbol_id);
        }

        for function_id in interpreter.get_ignored_functions() {
            candidate_function_ids.retain(|id| *id != function_id);
        }

        // iteration base
        let mut environment_ids = vec![interpreter.get_root_environment_id()];

        environment_ids
            .extend(interpreter.get_module_arena().get_gc_environments());

        let items = Vec::new();

        GarbageCollector {
            ignored_environment_ids,
            candidate_environment_ids,
            environment_ids,

            candidate_string_ids,
            candidate_keyword_ids,
            candidate_symbol_ids,
            candidate_cons_ids,
            candidate_object_ids,
            candidate_function_ids,

            items,
        }
    }

    fn retain_string_id(&mut self, string_id: StringId) {
        remove_value_from_vector(&mut self.candidate_string_ids, string_id);
    }

    fn retain_keyword_id(&mut self, keyword_id: KeywordId) {
        remove_value_from_vector(&mut self.candidate_keyword_ids, keyword_id);
    }

    fn retain_symbol_id(&mut self, symbol_id: SymbolId) {
        remove_value_from_vector(&mut self.candidate_symbol_ids, symbol_id);
    }

    fn retain_cons_id(
        &mut self,
        interpreter: &mut Interpreter,
        cons_id: ConsId,
    ) -> Result<(), Error> {
        remove_value_from_vector(&mut self.candidate_cons_ids, cons_id);

        self.items.push(interpreter.get_car(cons_id)?);
        self.items.push(interpreter.get_cdr(cons_id)?);

        Ok(())
    }

    fn retain_object_id(
        &mut self,
        interpreter: &mut Interpreter,
        object_id: ObjectId,
    ) -> Result<(), Error> {
        remove_value_from_vector(&mut self.candidate_object_ids, object_id);

        let mut gc_items =
            interpreter.get_object_arena().get_gc_items(object_id)?;

        self.items.append(&mut gc_items);

        Ok(())
    }

    fn retain_function_id(
        &mut self,
        interpreter: &mut Interpreter,
        function_id: FunctionId,
    ) -> Result<(), Error> {
        remove_value_from_vector(&mut self.candidate_function_ids, function_id);

        match interpreter.get_function_arena().get_gc_items(function_id)? {
            Some(mut gc_items) => self.items.append(&mut gc_items),
            _ => {}
        }

        match interpreter
            .get_function_arena()
            .get_gc_environment(function_id)?
        {
            Some(gc_environment_id) => {
                if !self.ignored_environment_ids.contains(&gc_environment_id) {
                    self.environment_ids.push(gc_environment_id)
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn retain_value(
        &mut self,
        interpreter: &mut Interpreter,
        value: Value,
    ) -> Result<(), Error> {
        match value {
            Value::String(string_id) => {
                self.retain_string_id(string_id);
            }
            Value::Keyword(keyword_id) => {
                self.retain_keyword_id(keyword_id);
            }
            Value::Symbol(symbol_id) => {
                self.retain_symbol_id(symbol_id);
            }
            Value::Cons(cons_id) => {
                self.retain_cons_id(interpreter, cons_id)?;
            }
            Value::Object(object_id) => {
                self.retain_object_id(interpreter, object_id)?;
            }
            Value::Function(function_id) => {
                self.retain_function_id(interpreter, function_id)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn collect_context_items(
        &mut self,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        // remove context values
        let mut context_items = interpreter.get_context().get_gc_items();

        while !context_items.is_empty() {
            let context_item = context_items.remove(0);

            self.retain_value(interpreter, context_item)?;
        }

        Ok(())
    }

    fn collect_ordinary_items(
        &mut self,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        // iteration over accessible environments
        while !self.environment_ids.is_empty() {
            let environment_id = self.environment_ids.remove(0);

            remove_value_from_vector(
                &mut self.candidate_environment_ids,
                environment_id,
            );
            self.ignored_environment_ids.push(environment_id);

            match interpreter
                .get_environment_arena()
                .get_parent(environment_id)?
            {
                Some(parent_id) => {
                    if !self.ignored_environment_ids.contains(&parent_id) {
                        self.environment_ids.push(parent_id)
                    }
                }
                _ => {}
            }
            let mut items_to_persist = interpreter
                .get_environment_arena()
                .get_environment_gc_items(environment_id)?;

            self.items.append(&mut items_to_persist);

            while !self.items.is_empty() {
                let item = self.items.remove(0);

                self.retain_value(interpreter, item)?;
            }
        }

        Ok(())
    }

    fn free(self, interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.free_environments(self.candidate_environment_ids)?;

        interpreter.free_strings(self.candidate_string_ids)?;
        interpreter.free_keywords(self.candidate_keyword_ids)?;
        interpreter.free_symbols(self.candidate_symbol_ids)?;
        interpreter.free_cons_cells(self.candidate_cons_ids)?;
        interpreter.free_objects(self.candidate_object_ids)?;
        interpreter.free_functions(self.candidate_function_ids)?;

        Ok(())
    }

    pub fn collect(
        mut self,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        self.collect_context_items(interpreter)?;
        self.collect_ordinary_items(interpreter)?;
        self.free(interpreter)?;

        Ok(())
    }
}

pub fn collect_garbage(interpreter: &mut Interpreter) -> Result<(), Error> {
    let gc = GarbageCollector::new(interpreter);

    let before = SystemTime::now().duration_since(UNIX_EPOCH).expect("");
    gc.collect(interpreter)?;
    let after = SystemTime::now().duration_since(UNIX_EPOCH).expect("");
    let diff = after - before;

    println!(
        "Collected garbage in {}.{} ms.",
        diff.as_millis(),
        diff.as_micros() % 1000
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use std::convert::TryInto;

    use crate::interpreter::environment::EnvironmentId;
    use crate::interpreter::value::BuiltinFunction;
    use crate::interpreter::value::Function;
    use crate::interpreter::value::SpecialFormFunction;
    use crate::utils::with_named_file;
    use crate::utils::with_tempdir;
    use crate::utils::with_working_directory;

    fn builtin_test_function(
        _interpreter: &mut Interpreter,
        _environment_id: EnvironmentId,
        _values: Vec<Value>,
    ) -> Result<Value, Error> {
        Ok(Value::Integer(1))
    }

    fn test_special_form(
        _interpreter: &mut Interpreter,
        _environment_id: EnvironmentId,
        _values: Vec<Value>,
    ) -> Result<Value, Error> {
        Ok(Value::Integer(1))
    }

    #[test]
    fn collects_strings() {
        let mut interpreter = Interpreter::new();

        let string_id = interpreter
            .execute_in_main_environment("\"string\"")
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_string(string_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_string(string_id).is_err());
    }

    #[test]
    fn collects_keywords() {
        let mut interpreter = Interpreter::new();

        let keyword_id = interpreter
            .execute_in_main_environment(":some-unused-keyword")
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_keyword(keyword_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_keyword(keyword_id).is_err());
    }

    #[test]
    fn collects_symbols() {
        let mut interpreter = Interpreter::new();

        let symbol_id = interpreter
            .execute_in_main_environment("'some-not-used-long-named-symbol")
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_symbol(symbol_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_symbol(symbol_id).is_err());
    }

    #[test]
    fn collects_cons_cells() {
        let mut interpreter = Interpreter::new();

        let cons_id = interpreter
            .execute_in_main_environment("(cons 1 2)")
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_car(cons_id).is_ok());
        nia_assert(interpreter.get_cdr(cons_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_car(cons_id).is_err());
        nia_assert(interpreter.get_cdr(cons_id).is_err());
    }

    #[test]
    fn collects_objects() {
        let mut interpreter = Interpreter::new();

        let object_id = interpreter
            .execute_in_main_environment("{}")
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_object_prototype(object_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_object_prototype(object_id).is_err());
    }

    #[test]
    fn collects_builtin_functions() {
        let mut interpreter = Interpreter::new();

        let function =
            Function::Builtin(BuiltinFunction::new(builtin_test_function));
        let function_id = interpreter.register_function(function);

        nia_assert(interpreter.get_function(function_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_special_forms() {
        let mut interpreter = Interpreter::new();

        let function =
            Function::SpecialForm(SpecialFormFunction::new(test_special_form));
        let function_id = interpreter.register_function(function);

        nia_assert(interpreter.get_function(function_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_interpreted_functions() {
        let mut interpreter = Interpreter::new();

        let function_id = interpreter
            .execute_in_main_environment("(fn () 1)")
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_function(function_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_macro_functions() {
        let mut interpreter = Interpreter::new();

        let function_id = interpreter
            .execute_in_main_environment("(function (macro () 1))")
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_function(function_id).is_ok());
        nia_assert(collect_garbage(&mut interpreter).is_ok());
        nia_assert(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn respects_cons_content() {
        let mut interpreter = Interpreter::new();

        let symbol_1 = interpreter
            .execute_in_main_environment("'some-unused-symbol-1")
            .unwrap()
            .try_into()
            .unwrap();
        let symbol_2 = interpreter
            .execute_in_main_environment("'some-unused-symbol-2")
            .unwrap()
            .try_into()
            .unwrap();

        let cons_id = interpreter
            .execute_in_main_environment(
                "(defv kekurus (cons 'some-unused-symbol-1 'some-unused-symbol-2)) kekurus",
            )
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_car(cons_id).is_ok());
        nia_assert(interpreter.get_cdr(cons_id).is_ok());
        nia_assert(interpreter.get_symbol(symbol_1).is_ok());
        nia_assert(interpreter.get_symbol(symbol_2).is_ok());

        nia_assert(collect_garbage(&mut interpreter).is_ok());

        nia_assert(interpreter.get_car(cons_id).is_ok());
        nia_assert(interpreter.get_cdr(cons_id).is_ok());
        nia_assert(interpreter.get_symbol(symbol_1).is_ok());
        nia_assert(interpreter.get_symbol(symbol_2).is_ok());
    }

    #[test]
    fn respects_object_contents() {
        let mut interpreter = Interpreter::new();

        let item_key = interpreter
            .execute_in_main_environment(":some-unused-keyword-1")
            .unwrap()
            .try_into()
            .unwrap();

        let item_key_symbol = interpreter
            .execute_in_main_environment("'some-unused-keyword-1")
            .unwrap()
            .try_into()
            .unwrap();

        let item_value = interpreter
            .execute_in_main_environment(":some-unused-keyword-2")
            .unwrap()
            .try_into()
            .unwrap();

        let object_id = interpreter
            .execute_in_main_environment(
                "(defv kekurus {:some-unused-keyword-1 :some-unused-keyword-2}) kekurus",
            )
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_keyword(item_key).is_ok());
        nia_assert(interpreter.get_symbol(item_key_symbol).is_ok());
        nia_assert(interpreter.get_keyword(item_value).is_ok());
        nia_assert(interpreter.get_object_prototype(object_id).is_ok());

        nia_assert(collect_garbage(&mut interpreter).is_ok());

        nia_assert(interpreter.get_keyword(item_key).is_err());
        nia_assert(interpreter.get_symbol(item_key_symbol).is_ok());
        nia_assert(interpreter.get_keyword(item_value).is_ok());
        nia_assert(interpreter.get_object_prototype(object_id).is_ok());
    }

    #[test]
    fn respects_function_code() {
        let mut interpreter = Interpreter::new();

        let function_id = interpreter
            .execute_in_main_environment(
                "(defv some-kekurus-variable (fn () 'some-unused-symbol)) some-kekurus-variable",
            )
            .unwrap()
            .try_into()
            .unwrap();

        let symbol_id = interpreter
            .execute_in_main_environment("'some-unused-symbol")
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(collect_garbage(&mut interpreter).is_ok());

        nia_assert(interpreter.get_function(function_id).is_ok());
        nia_assert(interpreter.get_symbol(symbol_id).is_ok());
    }

    #[test]
    fn respects_function_environment() {
        let mut interpreter = Interpreter::new();

        let symbol1 = interpreter
            .execute_in_main_environment("'kekurus-1")
            .unwrap()
            .try_into()
            .unwrap();

        let symbol2 = interpreter
            .execute_in_main_environment("'kekurus-1")
            .unwrap()
            .try_into()
            .unwrap();

        let function_id = interpreter
            .execute_in_main_environment(
                "(defv kekurus (let ((kekurus-1 1) (kekurus-2 2)) (fn () (+ 1 2)))) kekurus",
            )
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_function(function_id).is_ok());
        nia_assert(interpreter.get_symbol(symbol1).is_ok());
        nia_assert(interpreter.get_symbol(symbol2).is_ok());

        nia_assert(collect_garbage(&mut interpreter).is_ok());

        nia_assert(interpreter.get_function(function_id).is_ok());
        nia_assert(interpreter.get_symbol(symbol1).is_ok());
        nia_assert(interpreter.get_symbol(symbol2).is_ok());
    }

    #[test]
    fn respects_function_arguments() {
        let mut interpreter = Interpreter::new();

        let symbol1 = interpreter
            .execute_in_main_environment("'kekurus-arg-1")
            .unwrap()
            .try_into()
            .unwrap();

        let symbol2 = interpreter
            .execute_in_main_environment("'kekurus-arg-2")
            .unwrap()
            .try_into()
            .unwrap();

        let function1 = interpreter
            .execute_in_main_environment(
                "(defv kekurus-1 (fn (#opt (a 'kekurus-arg-1)) 1)) kekurus-1",
            )
            .unwrap()
            .try_into()
            .unwrap();

        let function2 = interpreter
            .execute_in_main_environment(
                "(defv kekurus-2 (fn (#opt (a 'kekurus-arg-2)) 1)) kekurus-2",
            )
            .unwrap()
            .try_into()
            .unwrap();

        nia_assert(interpreter.get_function(function1).is_ok());
        nia_assert(interpreter.get_function(function2).is_ok());
        nia_assert(interpreter.get_symbol(symbol1).is_ok());
        nia_assert(interpreter.get_symbol(symbol2).is_ok());

        nia_assert(collect_garbage(&mut interpreter).is_ok());

        nia_assert(interpreter.get_function(function1).is_ok());
        nia_assert(interpreter.get_function(function2).is_ok());
        nia_assert(interpreter.get_symbol(symbol1).is_ok());
        nia_assert(interpreter.get_symbol(symbol2).is_ok());
    }

    #[test]
    fn respects_context_values() {
        let mut interpreter = Interpreter::new();

        let symbols = vec![
            interpreter.intern_symbol_id("kekurus-closure-parameter"),
            interpreter.intern_symbol_id("kekurus-arg-1"),
            interpreter.intern_symbol_id("kekurus-arg-2"),
            interpreter.intern_symbol_id("kekurus-opt-default-parameter"),
            interpreter.intern_symbol_id("kekurus-key-default-parameter"),
        ];

        let pairs = vec!(
            (interpreter.intern_symbol_id("kekurus-1"), interpreter.execute_in_main_environment("\"kekurus-string\"").unwrap()),
            (interpreter.intern_symbol_id("kekurus-2"), interpreter.execute_in_main_environment(":kekurus-keyword").unwrap()),
            (interpreter.intern_symbol_id("kekurus-3"), interpreter.execute_in_main_environment("'kekurus-symbol").unwrap()),
            (interpreter.intern_symbol_id("kekurus-4"), interpreter.execute_in_main_environment("(cons 1 2)").unwrap()),
            (interpreter.intern_symbol_id("kekurus-5"), interpreter.execute_in_main_environment("{:a 1}").unwrap()),
            (interpreter.intern_symbol_id("kekurus-6"),
             interpreter.execute_in_main_environment(
                 "(let ((kekurus-closure-parameter 0)) (fn (kekurus-arg-1 #opt (kekurus-arg-2 'kekurus-opt-default-parameter)) (+ kekurus-arg-1 kekurus-arg-2)))"
             ).unwrap()),
            (interpreter.intern_symbol_id("kekurus-7"),
             interpreter.execute_in_main_environment(
                 "(let ((kekurus-closure-parameter 0)) (fn (kekurus-arg-1 #keys (kekurus-arg-2 'kekurus-key-default-parameter)) (+ kekurus-arg-1 kekurus-arg-2)))"
             ).unwrap()),
        );

        for pair in &pairs {
            interpreter.set_context_value(pair.0, pair.1).unwrap();
        }

        nia_assert(collect_garbage(&mut interpreter).is_ok());

        nia_assert(interpreter.get_symbol(pairs[0].0).is_ok());
        nia_assert(interpreter.get_symbol(pairs[1].0).is_ok());
        nia_assert(interpreter.get_symbol(pairs[2].0).is_ok());
        nia_assert(interpreter.get_symbol(pairs[3].0).is_ok());
        nia_assert(interpreter.get_symbol(pairs[4].0).is_ok());
        nia_assert(interpreter.get_symbol(pairs[5].0).is_ok());
        nia_assert(interpreter.get_symbol(pairs[6].0).is_ok());

        // todo: rewrite that
        nia_assert(
            interpreter
                .get_string(pairs[0].1.try_into().unwrap())
                .is_ok(),
        );
        nia_assert(
            interpreter
                .get_keyword(pairs[1].1.try_into().unwrap())
                .is_ok(),
        );
        nia_assert(
            interpreter
                .get_symbol(pairs[2].1.try_into().unwrap())
                .is_ok(),
        );
        nia_assert(interpreter.get_car(pairs[3].1.try_into().unwrap()).is_ok());
        nia_assert(interpreter.get_cdr(pairs[3].1.try_into().unwrap()).is_ok());
        nia_assert(
            interpreter
                .get_object_prototype(pairs[4].1.try_into().unwrap())
                .is_ok(),
        );
        nia_assert(
            interpreter
                .get_function(pairs[5].1.try_into().unwrap())
                .is_ok(),
        );
        nia_assert(
            interpreter
                .get_function(pairs[6].1.try_into().unwrap())
                .is_ok(),
        );

        for symbol_id in symbols {
            nia_assert(interpreter.get_symbol(symbol_id).is_ok());
        }
    }

    #[test]
    fn respects_special_variables() {
        let mut interpreter = Interpreter::new();

        let special_variables = vec![
            // todo: remainder, when new special variables is introduced, add them here
            interpreter.intern_symbol_id("this"),
            interpreter.intern_symbol_id("super"),
        ];

        collect_garbage(&mut interpreter).unwrap();

        for special_variable_symbol_id in special_variables {
            nia_assert_is_ok(
                &interpreter.get_symbol(special_variable_symbol_id),
            );
        }
    }

    #[test]
    #[ignore]
    fn respects_module_contents() {
        let content = r#"
        (defc nia-a 1)
        (defc nia-b 1.1)
        (defc nia-c #t)
        (defc nia-d #f)
        (defc nia-e "nia-string")
        (defc nia-f :nia-keyword)
        (defc nia-g 'nia-symbol)
        "#;

        with_tempdir(|directory| {
            with_named_file(&directory, "test.nia", content, || {
                with_working_directory(&directory, || {
                    let mut interpreter = Interpreter::new();

                    let symbol_identifiers = vec![
                        interpreter.intern_symbol_id("nia-a"),
                        interpreter.intern_symbol_id("nia-b"),
                        interpreter.intern_symbol_id("nia-c"),
                        interpreter.intern_symbol_id("nia-d"),
                        interpreter.intern_symbol_id("nia-e"),
                        interpreter.intern_symbol_id("nia-f"),
                        interpreter.intern_symbol_id("nia-g"),
                    ];

                    let string_id = interpreter.intern_string_id("nia-string");
                    let keyword_id =
                        interpreter.intern_keyword_id("nia-keyword");
                    let symbol_id = interpreter.intern_symbol_id("nia-symbol");

                    interpreter
                        .execute_in_main_environment("(import \"./test.nia\")")
                        .unwrap();

                    collect_garbage(&mut interpreter).unwrap();

                    for symbol_id in symbol_identifiers {
                        nia_assert_is_ok(&interpreter.get_symbol(symbol_id));
                    }

                    nia_assert_is_ok(&interpreter.get_string(string_id));
                    nia_assert_is_ok(&interpreter.get_keyword(keyword_id));
                    nia_assert_is_ok(&interpreter.get_symbol(symbol_id));
                })
            })
        })
    }
}
