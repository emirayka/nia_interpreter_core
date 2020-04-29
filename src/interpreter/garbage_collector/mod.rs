use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::value::StringId;
use crate::interpreter::value::KeywordId;
use crate::interpreter::value::SymbolId;
use crate::interpreter::value::ConsId;
use crate::interpreter::value::ObjectId;
use crate::interpreter::value::FunctionId;

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

impl GarbageCollector {
    pub fn new(interpreter: &mut Interpreter, environment_id: EnvironmentId) -> GarbageCollector {
        // these would be ignored
        let ignored_environment_ids = Vec::new();

        // these are candidates for deletion
        let candidate_environment_ids = interpreter.get_environment_arena().get_all_environments();

        let candidate_string_ids = interpreter.get_string_arena().get_all_string_identifiers();
        let candidate_keyword_ids = interpreter.get_keyword_arena().get_all_keyword_identifiers();
        let mut candidate_symbol_ids = interpreter.get_symbol_arena().get_all_symbol_identifiers();

        let candidate_cons_ids = interpreter.get_cons_arena().get_all_cons_identifiers();
        let candidate_object_ids = interpreter.get_object_arena().get_all_object_identifiers();
        let mut candidate_function_ids = interpreter.get_function_arena().get_all_function_identifiers();

        // remove which should be persisted
        for symbol_id in interpreter.get_ignored_symbols() {
            candidate_symbol_ids.retain(|id| *id != symbol_id);
        }

        for function_id in interpreter.get_ignored_functions() {
            candidate_function_ids.retain(|id| *id != function_id);
        }

        // iteration base
        let environment_ids = vec!(environment_id);

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
        self.candidate_string_ids.retain(|id| *id != string_id);
    }

    fn retain_keyword_id(&mut self, keyword_id: KeywordId) {
        self.candidate_keyword_ids.retain(|id| *id != keyword_id);
    }

    fn retain_symbol_id(&mut self, symbol_id: SymbolId) {
        self.candidate_symbol_ids.retain(|id| *id != symbol_id);
    }

    fn retain_cons_id(&mut self, interpreter: &mut Interpreter, cons_id: ConsId) -> Result<(), Error> {
        self.candidate_cons_ids.retain(|id| *id != cons_id);

        self.items.push(interpreter.get_car(cons_id)?);
        self.items.push(interpreter.get_cdr(cons_id)?);

        Ok(())
    }

    fn retain_object_id(&mut self, interpreter: &mut Interpreter, object_id: ObjectId) -> Result<(), Error> {
        self.candidate_object_ids.retain(|id| *id != object_id);

        let mut gc_items = interpreter.get_object_arena().get_gc_items(object_id)?;

        self.items.append(&mut gc_items);

        Ok(())
    }

    fn retain_function_id(&mut self, interpreter: &mut Interpreter, function_id: FunctionId) -> Result<(), Error> {
        self.candidate_function_ids.retain(|id| *id != function_id);

        match interpreter.get_function_arena().get_gc_items(function_id)? {
            Some(mut gc_items) => self.items.append(&mut gc_items),
            _ => {}
        }

        match interpreter.get_function_arena().get_gc_environment(function_id)? {
            Some(gc_environment_id) => {
                if !self.ignored_environment_ids.contains(&gc_environment_id) {
                    self.environment_ids.push(gc_environment_id)
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn retain_value(&mut self, interpreter: &mut Interpreter, value: Value) -> Result<(), Error> {
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

    fn collect_context_items(&mut self, interpreter: &mut Interpreter) -> Result<(), Error> {
        // remove context values
        let mut context_items = interpreter.get_context().get_gc_items();

        while !context_items.is_empty() {
            let context_item = context_items.remove(0);

            self.retain_value(interpreter, context_item)?;
        }

        Ok(())
    }

    fn collect_ordinary_items(&mut self, interpreter: &mut Interpreter) -> Result<(), Error> {
        // iteration over accessible environments
        while !self.environment_ids.is_empty() {
            let environment_id = self.environment_ids.remove(0);

            self.candidate_environment_ids.retain(|id| *id != environment_id);
            self.ignored_environment_ids.push(environment_id);

            match interpreter.get_environment_arena().get_parent(environment_id)? {
                Some(parent_id) => {
                    if !self.ignored_environment_ids.contains(&parent_id) {
                        self.environment_ids.push(parent_id)
                    }
                }
                _ => {}
            }
            let mut items_to_persist = interpreter.get_environment_arena()
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

    pub fn collect(mut self, interpreter: &mut Interpreter) -> Result<(), Error> {
        self.collect_context_items(interpreter)?;
        self.collect_ordinary_items(interpreter)?;
        self.free(interpreter)?;

        Ok(())
    }
}

pub fn collect_garbage(interpreter: &mut Interpreter, environment_id: EnvironmentId) -> Result<(), Error> {
    let gc = GarbageCollector::new(interpreter, environment_id);

    gc.collect(interpreter)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::convert::TryInto;

    use crate::interpreter::value::{Function, BuiltinFunction, SpecialFormFunction};
    use crate::interpreter::environment::EnvironmentId;

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
        let root_environment = interpreter.get_root_environment();

        let string_id = interpreter.execute("\"string\"")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_string(string_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_string(string_id).is_err());
    }

    #[test]
    fn collects_keywords() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let keyword_id = interpreter.execute(":some-unused-keyword")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_keyword(keyword_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_keyword(keyword_id).is_err());
    }

    #[test]
    fn collects_symbols() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let symbol_id = interpreter.execute("'some-not-used-long-named-symbol")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_symbol(symbol_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_symbol(symbol_id).is_err());
    }

    #[test]
    fn collects_cons_cells() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let cons_id = interpreter.execute("(cons 1 2)")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_car(cons_id).is_ok());
        assert!(interpreter.get_cdr(cons_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_car(cons_id).is_err());
        assert!(interpreter.get_cdr(cons_id).is_err());
    }

    #[test]
    fn collects_objects() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let object_id = interpreter.execute("{}")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_object_proto(object_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_object_proto(object_id).is_err());
    }

    #[test]
    fn collects_builtin_functions() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let function = Function::Builtin(BuiltinFunction::new(builtin_test_function));
        let function_id = interpreter.register_function(function);

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_special_forms() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let function = Function::SpecialForm(SpecialFormFunction::new(test_special_form));
        let function_id = interpreter.register_function(function);

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_interpreted_functions() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let function_id = interpreter.execute("(fn () 1)")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn collects_macro_functions() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let function_id = interpreter.execute("(function (macro () 1))")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());
        assert!(interpreter.get_function(function_id).is_err());
    }

    #[test]
    fn respects_cons_content() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let symbol_1 = interpreter.execute("'some-unused-symbol-1")
            .unwrap()
            .try_into()
            .unwrap();
        let symbol_2 = interpreter.execute("'some-unused-symbol-2")
            .unwrap()
            .try_into()
            .unwrap();

        let cons_id = interpreter.execute("(defv kekurus (cons 'some-unused-symbol-1 'some-unused-symbol-2)) kekurus")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_car(cons_id).is_ok());
        assert!(interpreter.get_cdr(cons_id).is_ok());
        assert!(interpreter.get_symbol(symbol_1).is_ok());
        assert!(interpreter.get_symbol(symbol_2).is_ok());

        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());

        assert!(interpreter.get_car(cons_id).is_ok());
        assert!(interpreter.get_cdr(cons_id).is_ok());
        assert!(interpreter.get_symbol(symbol_1).is_ok());
        assert!(interpreter.get_symbol(symbol_2).is_ok());
    }

    #[test]
    fn respects_object_contents() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let item_key = interpreter.execute(":some-unused-keyword-1")
            .unwrap()
            .try_into()
            .unwrap();

        let item_key_symbol = interpreter.execute("'some-unused-keyword-1")
            .unwrap()
            .try_into()
            .unwrap();

        let item_value = interpreter.execute(":some-unused-keyword-2")
            .unwrap()
            .try_into()
            .unwrap();

        let object_id = interpreter.execute("(defv kekurus {:some-unused-keyword-1 :some-unused-keyword-2}) kekurus")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_keyword(item_key).is_ok());
        assert!(interpreter.get_symbol(item_key_symbol).is_ok());
        assert!(interpreter.get_keyword(item_value).is_ok());
        assert!(interpreter.get_object_proto(object_id).is_ok());

        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());

        assert!(interpreter.get_keyword(item_key).is_err());
        assert!(interpreter.get_symbol(item_key_symbol).is_ok());
        assert!(interpreter.get_keyword(item_value).is_ok());
        assert!(interpreter.get_object_proto(object_id).is_ok());
    }

    #[test]
    fn respects_function_code() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let function_id = interpreter.execute(
            "(defv some-kekurus-variable (fn () 'some-unused-symbol)) some-kekurus-variable"
        ).unwrap()
            .try_into()
            .unwrap();

        let symbol_id = interpreter.execute("'some-unused-symbol")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(interpreter.get_symbol(symbol_id).is_ok());
    }

    #[test]
    fn respects_function_environment() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let symbol1 = interpreter.execute("'kekurus-1")
            .unwrap()
            .try_into()
            .unwrap();

        let symbol2 = interpreter.execute("'kekurus-1")
            .unwrap()
            .try_into()
            .unwrap();

        let function_id = interpreter.execute(
            "(defv kekurus (let ((kekurus-1 1) (kekurus-2 2)) (fn () (+ 1 2)))) kekurus"
        ).unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(interpreter.get_symbol(symbol1).is_ok());
        assert!(interpreter.get_symbol(symbol2).is_ok());

        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());

        assert!(interpreter.get_function(function_id).is_ok());
        assert!(interpreter.get_symbol(symbol1).is_ok());
        assert!(interpreter.get_symbol(symbol2).is_ok());
    }

    #[test]
    fn respects_function_arguments() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let symbol1 = interpreter.execute("'kekurus-arg-1")
            .unwrap()
            .try_into()
            .unwrap();

        let symbol2 = interpreter.execute("'kekurus-arg-2")
            .unwrap()
            .try_into()
            .unwrap();

        let function1 = interpreter.execute("(defv kekurus-1 (fn (#opt (a 'kekurus-arg-1)) 1)) kekurus-1")
            .unwrap()
            .try_into()
            .unwrap();

        let function2 = interpreter.execute("(defv kekurus-2 (fn (#opt (a 'kekurus-arg-2)) 1)) kekurus-2")
            .unwrap()
            .try_into()
            .unwrap();

        assert!(interpreter.get_function(function1).is_ok());
        assert!(interpreter.get_function(function2).is_ok());
        assert!(interpreter.get_symbol(symbol1).is_ok());
        assert!(interpreter.get_symbol(symbol2).is_ok());

        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());

        assert!(interpreter.get_function(function1).is_ok());
        assert!(interpreter.get_function(function2).is_ok());
        assert!(interpreter.get_symbol(symbol1).is_ok());
        assert!(interpreter.get_symbol(symbol2).is_ok());
    }

    #[test]
    fn respects_context_values() {
        let mut interpreter = Interpreter::new();
        let root_environment = interpreter.get_root_environment();

        let symbols = vec!(
            interpreter.intern("kekurus-closure-parameter"),
            interpreter.intern("kekurus-arg-1"),
            interpreter.intern("kekurus-arg-2"),
            interpreter.intern("kekurus-opt-default-parameter"),
            interpreter.intern("kekurus-key-default-parameter"),
        );

        let pairs = vec!(
            (interpreter.intern("kekurus-1"), interpreter.execute("\"kekurus-string\"").unwrap()),
            (interpreter.intern("kekurus-2"), interpreter.execute(":kekurus-keyword").unwrap()),
            (interpreter.intern("kekurus-3"), interpreter.execute("'kekurus-symbol").unwrap()),
            (interpreter.intern("kekurus-4"), interpreter.execute("(cons 1 2)").unwrap()),
            (interpreter.intern("kekurus-5"), interpreter.execute("{:a 1}").unwrap()),
            (interpreter.intern("kekurus-6"),
             interpreter.execute(
                 "(let ((kekurus-closure-parameter 0)) (fn (kekurus-arg-1 #opt (kekurus-arg-2 'kekurus-opt-default-parameter)) (+ kekurus-arg-1 kekurus-arg-2)))"
             ).unwrap()),
            (interpreter.intern("kekurus-7"),
             interpreter.execute(
                 "(let ((kekurus-closure-parameter 0)) (fn (kekurus-arg-1 #keys (kekurus-arg-2 'kekurus-key-default-parameter)) (+ kekurus-arg-1 kekurus-arg-2)))"
             ).unwrap()),
        );

        for pair in &pairs {
            interpreter.set_context_value(pair.0, pair.1).unwrap();
        }

        assert!(collect_garbage(&mut interpreter, root_environment).is_ok());

        assert!(interpreter.get_symbol(pairs[0].0).is_ok());
        assert!(interpreter.get_symbol(pairs[1].0).is_ok());
        assert!(interpreter.get_symbol(pairs[2].0).is_ok());
        assert!(interpreter.get_symbol(pairs[3].0).is_ok());
        assert!(interpreter.get_symbol(pairs[4].0).is_ok());
        assert!(interpreter.get_symbol(pairs[5].0).is_ok());
        assert!(interpreter.get_symbol(pairs[6].0).is_ok());

        // todo: rewrite that
        assert!(interpreter.get_string(pairs[0].1.try_into().unwrap()).is_ok());
        assert!(interpreter.get_keyword(pairs[1].1.try_into().unwrap()).is_ok());
        assert!(interpreter.get_symbol(pairs[2].1.try_into().unwrap()).is_ok());
        assert!(interpreter.get_car(pairs[3].1.try_into().unwrap()).is_ok());
        assert!(interpreter.get_cdr(pairs[3].1.try_into().unwrap()).is_ok());
        assert!(interpreter.get_object_proto(pairs[4].1.try_into().unwrap()).is_ok());
        assert!(interpreter.get_function(pairs[5].1.try_into().unwrap()).is_ok());
        assert!(interpreter.get_function(pairs[6].1.try_into().unwrap()).is_ok());

        for symbol_id in symbols {
            assert!(interpreter.get_symbol(symbol_id).is_ok());
        }
    }
}