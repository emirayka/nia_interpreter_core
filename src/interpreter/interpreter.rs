use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;
use std::path::PathBuf;

use crate::BuiltinFunction;
use crate::CallStack;
use crate::ConsArena;
use crate::ConsId;
use crate::Context;
use crate::EnvironmentArena;
use crate::EnvironmentId;
use crate::Error;
use crate::Function;
use crate::FunctionArena;
use crate::FunctionArguments;
use crate::FunctionId;
use crate::InterpretedFunction;
use crate::Keyword;
use crate::KeywordArena;
use crate::KeywordId;
use crate::MacroFunction;
use crate::Module;
use crate::ModuleArena;
use crate::ModuleId;
use crate::NiaString;
use crate::Object;
use crate::ObjectArena;
use crate::ObjectId;
use crate::ObjectValueWrapper;
use crate::SpecialFormFunction;
use crate::SpecialVariableFunction;
use crate::StringArena;
use crate::StringId;
use crate::Symbol;
use crate::SymbolArena;
use crate::SymbolId;
use crate::Value;

use crate::interpreter::garbage_collector::collect_garbage;
use crate::interpreter::reader::read_elements;
use crate::parser::parse;

use crate::evaluate_builtin_function_invocation;
use crate::evaluate_interpreted_function_invocation;
use crate::evaluate_value;
use crate::evaluate_values;
use crate::interpreter::stdlib::infect_stdlib;

use crate::library;

#[derive(Clone)]
pub struct Interpreter {
    environment_arena: EnvironmentArena,

    module_arena: ModuleArena,
    root_module_id: ModuleId,
    main_module_id: ModuleId,
    current_module: ModuleId,

    string_arena: StringArena,
    keyword_arena: KeywordArena,
    symbol_arena: SymbolArena,
    cons_arena: ConsArena,
    object_arena: ObjectArena,
    function_arena: FunctionArena,

    context: Context,
    exclusive_nil: SymbolId,
    exclusive_nil_value: Value,
    internal_functions: HashMap<String, FunctionId>,
    special_variables: HashMap<SymbolId, SpecialVariableFunction>,

    call_stack: CallStack,
    this_object: Option<ObjectId>,
    is_listening: bool,
}

impl Interpreter {
    pub fn raw() -> Interpreter {
        let mut interpreter = {
            let mut environment_arena = EnvironmentArena::new();

            let root_environment_id = environment_arena.alloc();
            let main_environment_id = environment_arena
                .alloc_child(root_environment_id)
                .expect("Cannot construct main environment.");

            // making stdlib and root modules
            let mut module_arena = ModuleArena::new();

            let root_module_id =
                module_arena.make_with_empty_path(root_environment_id);
            let main_module_id =
                module_arena.make_with_empty_path(main_environment_id);
            let current_module = main_module_id;

            let string_arena = StringArena::new();
            let keyword_arena = KeywordArena::new();
            let mut symbol_arena = SymbolArena::new();
            let cons_arena = ConsArena::new();
            let object_arena = ObjectArena::new();
            let function_arena = FunctionArena::new();

            let context = Context::new();
            let exclusive_nil = symbol_arena.gensym("saika");
            let exclusive_nil_value = Value::Symbol(exclusive_nil);
            let internal_functions = HashMap::new();
            let special_variables = HashMap::new();

            let call_stack = CallStack::new();
            let this_object = None;
            let is_listening = false;

            // construct interpreter
            Interpreter {
                environment_arena,

                module_arena,
                root_module_id,
                main_module_id,
                current_module,

                string_arena,
                keyword_arena,
                symbol_arena,
                cons_arena,
                object_arena,
                function_arena,

                context,
                exclusive_nil,
                exclusive_nil_value,
                internal_functions,
                special_variables,

                call_stack,
                this_object,
                is_listening,
            }
        };

        // nil
        let root_environment_id = interpreter.get_root_environment_id();
        let nil_symbol_id = interpreter.intern_symbol_id("nil");
        let nil_value = nil_symbol_id.into();

        interpreter
            .define_const_variable(
                root_environment_id,
                nil_symbol_id,
                nil_value,
            )
            .expect("Cannot define `nil' symbol.");

        // break
        let break_function = Function::Builtin(BuiltinFunction::new(
            crate::interpreter::internal_functions::_break,
        ));
        let break_function_id = interpreter.register_function(break_function);
        interpreter
            .internal_functions
            .insert(String::from("break"), break_function_id);

        // continue
        let continue_function = Function::Builtin(BuiltinFunction::new(
            crate::interpreter::internal_functions::_continue,
        ));
        let continue_function_id =
            interpreter.register_function(continue_function);
        interpreter
            .internal_functions
            .insert(String::from("continue"), continue_function_id);

        // special variables
        let this_symbol_id = interpreter.intern_symbol_id("this");
        let super_symbol_id = interpreter.intern_symbol_id("super");

        interpreter.special_variables.insert(
            this_symbol_id,
            crate::interpreter::special_variables::_this,
        );
        interpreter.special_variables.insert(
            super_symbol_id,
            crate::interpreter::special_variables::_super,
        );

        // return constructed interpreter
        interpreter
    }

    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter::raw();

        match infect_stdlib(&mut interpreter) {
            Ok(()) => {},
            Err(error) => panic!("Cannot construct interpreter: {:?}", error),
        }

        interpreter
    }

    pub fn get_ignored_symbols(&self) -> Vec<SymbolId> {
        let mut vector = Vec::new();

        vector.push(self.exclusive_nil);
        vector.extend(self.special_variables.keys().into_iter());

        vector
    }

    pub fn get_ignored_functions(&self) -> Vec<FunctionId> {
        self.internal_functions
            .values()
            .into_iter()
            .map(|id| *id)
            .collect()
    }

    pub fn get_exclusive_nil_symbol_id(&self) -> SymbolId {
        self.exclusive_nil
    }

    pub fn get_exclusive_nil_value(&self) -> Value {
        self.exclusive_nil_value
    }

    pub fn get_special_variable(
        &self,
        symbol_id: SymbolId,
    ) -> Option<&SpecialVariableFunction> {
        self.special_variables.get(&symbol_id)
    }

    pub fn get_this_object(&self) -> Option<ObjectId> {
        self.this_object
    }

    pub fn set_this_object(&mut self, object_id: ObjectId) {
        self.this_object = Some(object_id);
    }

    pub fn clear_this_object(&mut self) {
        self.this_object = None;
    }

    pub fn is_listening(&self) -> bool {
        self.is_listening
    }

    pub fn start_listening(&mut self) {
        self.is_listening = true;
    }

    pub fn stop_listening(&mut self) {
        self.is_listening = false;
    }
}

impl Interpreter {
    pub fn get_string_arena(&self) -> &StringArena {
        &self.string_arena
    }

    pub fn free_strings(
        &mut self,
        string_ids: Vec<StringId>,
    ) -> Result<(), Error> {
        for string_id in string_ids {
            self.string_arena.free_string(string_id)?;
        }

        Ok(())
    }

    pub fn intern_string_id(&mut self, string: &str) -> StringId {
        self.string_arena.intern_string(string)
    }

    pub fn intern_string_value(&mut self, string: &str) -> Value {
        Value::String(self.intern_string_id(string))
    }

    pub fn get_string(&self, string_id: StringId) -> Result<&NiaString, Error> {
        self.string_arena.get_string(string_id)
    }
}

impl Interpreter {
    pub fn get_keyword_arena(&self) -> &KeywordArena {
        &self.keyword_arena
    }

    pub fn free_keywords(
        &mut self,
        keyword_ids: Vec<KeywordId>,
    ) -> Result<(), Error> {
        for keyword_id in keyword_ids {
            self.keyword_arena.free_keyword(keyword_id)?;
        }

        Ok(())
    }

    pub fn intern_keyword_id(&mut self, keyword_name: &str) -> KeywordId {
        self.keyword_arena.intern_keyword(keyword_name)
    }

    pub fn intern_keyword_value(&mut self, keyword_name: &str) -> Value {
        self.intern_keyword_id(keyword_name).into()
    }

    pub fn get_keyword(
        &self,
        keyword_id: KeywordId,
    ) -> Result<&Keyword, Error> {
        self.keyword_arena
            .get_keyword(keyword_id)
            .map(|keyword| keyword)
    }
}

impl Interpreter {
    pub fn get_symbol_arena(&self) -> &SymbolArena {
        &self.symbol_arena
    }

    pub fn free_symbols(
        &mut self,
        symbol_ids: Vec<SymbolId>,
    ) -> Result<(), Error> {
        for symbol_id in symbol_ids {
            self.symbol_arena.free_symbol(symbol_id)?;
        }

        Ok(())
    }

    pub fn get_symbol(&self, symbol_id: SymbolId) -> Result<&Symbol, Error> {
        self.symbol_arena.get_symbol(symbol_id)
    }

    pub fn get_symbol_name(
        &self,
        symbol_id: SymbolId,
    ) -> Result<&String, Error> {
        let symbol = self.get_symbol(symbol_id)?;

        Ok(symbol.get_name())
    }

    pub fn intern_symbol_id(&mut self, symbol_name: &str) -> SymbolId {
        self.symbol_arena.intern(symbol_name)
    }

    pub fn intern_symbol(&mut self, symbol_name: &str) -> &Symbol {
        let symbol_id = self.symbol_arena.intern(symbol_name);

        self.get_symbol(symbol_id).unwrap()
    }

    pub fn intern_symbol_value(&mut self, symbol_name: &str) -> Value {
        Value::Symbol(self.symbol_arena.intern(symbol_name))
    }

    pub fn intern_nil_symbol_id(&mut self) -> SymbolId {
        self.intern_symbol_id("nil")
    }

    pub fn intern_nil_symbol(&mut self) -> &Symbol {
        self.intern_symbol("nil")
    }

    pub fn intern_nil_symbol_value(&mut self) -> Value {
        self.intern_symbol_value("nil")
    }

    pub fn gensym(&mut self, symbol_name: &str) -> SymbolId {
        self.symbol_arena.gensym(symbol_name)
    }

    pub fn gensym_symbol(&mut self, symbol_name: &str) -> &Symbol {
        let symbol_id = self.gensym(symbol_name);

        self.get_symbol(symbol_id).unwrap()
    }

    pub fn gensym_symbol_value(&mut self, symbol_name: &str) -> Value {
        let symbol_id = self.gensym(symbol_name);

        Value::Symbol(symbol_id)
    }

    pub fn symbol_is_nil(&self, symbol_id: SymbolId) -> Result<bool, Error> {
        let symbol = self.get_symbol(symbol_id)?;

        Ok(symbol.get_name() == "nil" && symbol.get_gensym_id() == 0)
    }

    pub fn symbol_is_not_nil(
        &mut self,
        symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let symbol = self.get_symbol(symbol_id)?;

        Ok(symbol.get_name() != "nil" || symbol.get_gensym_id() != 0)
    }

    pub fn check_if_symbol_special(
        &self,
        symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let symbol_name = self.get_symbol_name(symbol_id)?;

        let result = symbol_name == "#opt"
            || symbol_name == "#rest"
            || symbol_name == "#keys";

        Ok(result)
    }

    pub fn check_if_symbol_constant(
        &self,
        symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let symbol_name = self.get_symbol_name(symbol_id)?;

        let result = symbol_name == "nil"
            || symbol_name == "this"
            || symbol_name == "super";

        Ok(result)
    }

    pub fn check_if_symbol_assignable(
        &self,
        symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let is_not_constant = !self.check_if_symbol_constant(symbol_id)?;
        let is_not_special = !self.check_if_symbol_special(symbol_id)?;

        Ok(is_not_constant && is_not_special)
    }

    pub fn check_if_symbol_internable(
        &mut self,
        symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        let is_not_special = !self.check_if_symbol_special(symbol_id)?;

        Ok(is_not_special)
    }
}

impl Interpreter {
    pub fn get_cons_arena(&self) -> &ConsArena {
        &self.cons_arena
    }

    pub fn free_cons_cells(
        &mut self,
        cons_ids: Vec<ConsId>,
    ) -> Result<(), Error> {
        for cons_id in cons_ids {
            self.cons_arena.free_cons(cons_id)?;
        }

        Ok(())
    }

    pub fn make_cons(&mut self, car: Value, cdr: Value) -> ConsId {
        self.cons_arena.make_cons(car, cdr)
    }

    pub fn make_cons_value(&mut self, car: Value, cdr: Value) -> Value {
        Value::Cons(self.cons_arena.make_cons(car, cdr))
    }

    pub fn get_car(&self, cons_id: ConsId) -> Result<Value, Error> {
        self.cons_arena.get_car(cons_id)
    }

    pub fn get_cdr(&self, cons_id: ConsId) -> Result<Value, Error> {
        self.cons_arena.get_cdr(cons_id)
    }

    pub fn get_cadr(&self, cons_id: ConsId) -> Result<Value, Error> {
        let cdr = self.get_cdr(cons_id)?;

        match cdr {
            Value::Cons(cdr_cons_id) => self.get_car(cdr_cons_id),
            _ => {
                return Error::generic_execution_error(
                    "Cannot get car of not a cons value",
                )
                .into();
            },
        }
    }

    pub fn get_cddr(&self, cons_id: ConsId) -> Result<Value, Error> {
        let cdr = self.get_cdr(cons_id)?;

        match cdr {
            Value::Cons(cdr_cons_id) => self.get_cdr(cdr_cons_id),
            _ => {
                return Error::generic_execution_error(
                    "Cannot get cdr of not a cons value",
                )
                .into();
            },
        }
    }

    pub fn set_car(
        &mut self,
        cons_id: ConsId,
        value: Value,
    ) -> Result<(), Error> {
        self.cons_arena.set_car(cons_id, value)
    }

    pub fn set_cdr(
        &mut self,
        cons_id: ConsId,
        value: Value,
    ) -> Result<(), Error> {
        self.cons_arena.set_cdr(cons_id, value)
    }

    pub fn vec_to_list(&mut self, vector: Vec<Value>) -> Value {
        let nil = self.intern_nil_symbol_value();

        self.cons_arena.vec_to_list(nil, vector)
    }

    pub fn list_to_vec(&self, cons_id: ConsId) -> Result<Vec<Value>, Error> {
        let mut vector = self.cons_arena.list_to_vec(cons_id)?;

        // Remove last item of the vector if it's nil. It's necessary, because ConsArena can't say
        // if a SymbolId is the one registered for nil, so it returns all items in the list,
        // including the nil at the cdr of the last cell of the list.
        match vector.last() {
            Some(&val @ _) => {
                if let Value::Symbol(symbol_id) = val {
                    if self.symbol_is_nil(symbol_id)? {
                        vector.remove(vector.len() - 1);
                    }
                }
            },
            _ => {},
        }

        Ok(vector)
    }
}

impl Interpreter {
    pub fn get_object_arena(&self) -> &ObjectArena {
        &self.object_arena
    }

    pub fn free_objects(
        &mut self,
        object_ids: Vec<ObjectId>,
    ) -> Result<(), Error> {
        for object_id in object_ids {
            self.object_arena.free_object(object_id)?;
        }

        Ok(())
    }

    pub fn make_object(&mut self) -> ObjectId {
        self.object_arena.make()
    }

    pub fn make_object_value(&mut self) -> Value {
        Value::Object(self.object_arena.make())
    }

    pub fn make_child_object(&mut self, prototype_id: ObjectId) -> ObjectId {
        self.object_arena.make_child(prototype_id)
    }

    pub fn get_object(&self, object_id: ObjectId) -> Result<&Object, Error> {
        let object = self.object_arena.get_object(object_id)?;

        Ok(object)
    }

    pub fn get_object_mut(
        &mut self,
        object_id: ObjectId,
    ) -> Result<&mut Object, Error> {
        let object = self.object_arena.get_object_mut(object_id)?;

        Ok(object)
    }

    pub fn object_has_property(
        &mut self,
        object_id: ObjectId,
        key_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        self.object_arena.has_property(object_id, key_symbol_id)
    }

    pub fn get_object_property(
        &mut self,
        object_id: ObjectId,
        key_symbol_id: SymbolId,
    ) -> Result<Option<Value>, Error> {
        self.object_arena
            .get_property_value(object_id, key_symbol_id)
    }

    pub fn set_object_property(
        &mut self,
        object_id: ObjectId,
        key_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.object_arena
            .set_property(object_id, key_symbol_id, value)
    }

    pub fn get_object_prototype(
        &self,
        object_id: ObjectId,
    ) -> Result<Option<ObjectId>, Error> {
        let object = self.object_arena.get_object(object_id)?;

        Ok(object.get_prototype())
    }

    pub fn set_object_prototype(
        &mut self,
        object_id: ObjectId,
        proto_id: ObjectId,
    ) -> Result<(), Error> {
        let object = self.object_arena.get_object_mut(object_id)?;
        object.set_prototype(proto_id)?;

        Ok(())
    }

    pub fn get_object_items(
        &self,
        object_id: ObjectId,
    ) -> Result<&HashMap<SymbolId, ObjectValueWrapper>, Error> {
        let object = self.object_arena.get_object(object_id)?;

        Ok(object.get_properties())
    }
}

impl Interpreter {
    pub fn get_function_arena(&self) -> &FunctionArena {
        &self.function_arena
    }

    pub fn free_functions(
        &mut self,
        function_ids: Vec<FunctionId>,
    ) -> Result<(), Error> {
        for function_id in function_ids {
            self.function_arena.free_function(function_id)?;
        }

        Ok(())
    }

    pub fn register_function(&mut self, function: Function) -> FunctionId {
        self.function_arena.register_function(function)
    }

    pub fn get_function(
        &self,
        function_id: FunctionId,
    ) -> Result<&Function, Error> {
        self.function_arena.get_function(function_id)
    }

    pub fn get_internal_function(
        &self,
        name: &str,
    ) -> Result<FunctionId, Error> {
        match self.internal_functions.get(name) {
            Some(function_id) => Ok(*function_id),
            _ => Error::failure(format!(
                "Cannot find internal function: {}",
                name
            ))
            .into(),
        }
    }
}

impl Interpreter {
    pub fn get_environment_arena(&self) -> &EnvironmentArena {
        &self.environment_arena
    }

    pub fn free_environments(
        &mut self,
        environment_ids: Vec<EnvironmentId>,
    ) -> Result<(), Error> {
        for environment_id in environment_ids {
            self.environment_arena.free_environment(environment_id)?;
        }

        Ok(())
    }

    pub fn get_root_environment_id(&self) -> EnvironmentId {
        self.get_root_module().get_environment_id()
    }

    pub fn get_main_environment_id(&self) -> EnvironmentId {
        self.get_main_module().get_environment_id()
    }

    pub fn lookup_environment_by_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
    ) -> Result<Option<EnvironmentId>, Error> {
        self.environment_arena
            .lookup_environment_by_variable(environment_id, variable_symbol_id)
    }

    pub fn lookup_environment_by_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
    ) -> Result<Option<EnvironmentId>, Error> {
        self.environment_arena
            .lookup_environment_by_function(environment_id, function_symbol_id)
    }

    pub fn has_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        self.environment_arena
            .has_variable(environment_id, variable_symbol_id)
    }

    pub fn has_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        self.environment_arena
            .has_function(environment_id, function_symbol_id)
    }

    pub fn define_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena.define_variable(
            environment_id,
            variable_symbol_id,
            value,
        )
    }

    pub fn define_const_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena.define_const_variable(
            environment_id,
            variable_symbol_id,
            value,
        )
    }

    pub fn define_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena.define_function(
            environment_id,
            function_symbol_id,
            value,
        )
    }

    pub fn define_const_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena.define_const_function(
            environment_id,
            function_symbol_id,
            value,
        )
    }

    pub fn set_environment_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena.set_environment_variable(
            environment_id,
            variable_symbol_id,
            value,
        )
    }

    pub fn set_environment_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena.set_environment_function(
            environment_id,
            function_symbol_id,
            value,
        )
    }

    pub fn set_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena.set_variable(
            environment_id,
            variable_symbol_id,
            value,
        )
    }

    pub fn set_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena.set_function(
            environment_id,
            function_symbol_id,
            value,
        )
    }

    pub fn lookup_variable(
        &self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
    ) -> Result<Option<Value>, Error> {
        self.environment_arena
            .lookup_variable(environment_id, variable_symbol_id)
    }

    pub fn lookup_function(
        &self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
    ) -> Result<Option<Value>, Error> {
        self.environment_arena
            .lookup_function(environment_id, function_symbol_id)
    }

    pub fn make_environment(
        &mut self,
        parent_environment: EnvironmentId,
    ) -> Result<EnvironmentId, Error> {
        self.environment_arena.alloc_child(parent_environment)
    }

    pub fn remove_environment(
        &mut self,
        environment_id: EnvironmentId,
    ) -> Result<(), Error> {
        self.environment_arena.free_environment(environment_id)
    }

    pub fn get_environment_gc_items(
        &self,
        environment_id: EnvironmentId,
    ) -> Result<Vec<Value>, Error> {
        self.environment_arena
            .get_environment_gc_items(environment_id)
    }
}

impl Interpreter {
    pub fn get_module_arena(&self) -> &ModuleArena {
        &self.module_arena
    }

    fn make_module(
        &mut self,
        path: String,
        environment_id: EnvironmentId,
    ) -> ModuleId {
        self.module_arena.make(path, environment_id)
    }

    pub fn get_root_module(&self) -> &Module {
        self.module_arena.get_module(self.root_module_id).unwrap()
    }

    pub fn get_main_module(&self) -> &Module {
        self.module_arena.get_module(self.main_module_id).unwrap()
    }

    pub fn get_current_module(&self) -> &Module {
        self.module_arena.get_module(self.current_module).unwrap()
    }

    pub fn get_current_module_mut(&mut self) -> &mut Module {
        self.module_arena
            .get_module_mut(self.current_module)
            .unwrap()
    }

    pub fn get_module(&self, module_id: ModuleId) -> Result<&Module, Error> {
        self.module_arena.get_module(module_id)
    }

    pub fn get_module_mut(
        &mut self,
        module_id: ModuleId,
    ) -> Result<&mut Module, Error> {
        self.module_arena.get_module_mut(module_id)
    }

    fn load_module(&mut self, module_path: &str) -> Result<ModuleId, Error> {
        let path = Path::new(module_path);

        if !path.exists() {
            return Error::generic_execution_error(&format!(
                "File \"{}\" does not exist.",
                module_path
            ))
            .into();
        }

        let metadata = std::fs::metadata(path).map_err(|_| {
            Error::generic_execution_error(&format!(
                "Cannot check metadata of file: \"{}\".",
                module_path
            ))
        })?;

        if !metadata.is_file() {
            return Error::generic_execution_error(&format!(
                "\"{}\" is not a file.",
                module_path
            ))
            .into();
        }

        let module_content = std::fs::read_to_string(path).map_err(|_| {
            Error::generic_execution_error(&format!(
                "Cannot read file: \"{}\"",
                module_path
            ))
        })?;

        let code = parse(&module_content).map_err(|error| {
            Error::parse_error(&format!(
                "Cannot parse input file: \"{}\". Message: \"{}\"",
                module_path, error
            ))
        })?;

        let values =
            read_elements(self, code.get_elements()).map_err(|error| {
                Error::generic_execution_error("Error reading module.")
            })?;

        let root_environment_id = self.get_root_environment_id();
        let module_environment_id =
            self.make_environment(root_environment_id)?;

        let module_id =
            self.make_module(String::from(module_path), module_environment_id);
        let previous_current_module_id = self.current_module;

        self.current_module = module_id;
        evaluate_values(self, module_environment_id, &values)?;
        self.current_module = previous_current_module_id;

        Ok(module_id)
    }

    pub fn intern_module(
        &mut self,
        module_path: &str,
    ) -> Result<ModuleId, Error> {
        let module_id = match self.module_arena.get_module_id(module_path) {
            Some(module_id) => module_id,
            None => self.load_module(module_path)?,
        };

        Ok(module_id)
    }

    pub fn resolve_with_current_module_path(
        &self,
        path: String,
    ) -> Result<String, Error> {
        let current_module_path =
            self.get_module(self.current_module)?.get_path().clone();

        crate::utils::resolve_path_with_current_module_path(
            current_module_path,
            path,
        )
    }
}

impl Interpreter {
    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn get_context_value(
        &self,
        symbol_id: SymbolId,
    ) -> Result<Value, Error> {
        self.context.get_value(symbol_id)
    }

    pub fn set_context_value(
        &mut self,
        symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.context.set_value(symbol_id, value)
    }
}

impl Interpreter {
    pub fn push_named_call(
        &mut self,
        function_id: FunctionId,
        function_name_symbol_id: SymbolId,
        arguments: Vec<Value>,
    ) {
        self.call_stack.push_named_function_invocation(
            function_id,
            function_name_symbol_id,
            arguments,
        )
    }

    pub fn push_anonymous_call(
        &mut self,
        function_id: FunctionId,
        arguments: Vec<Value>,
    ) {
        self.call_stack
            .push_anonymous_function_invocation(function_id, arguments)
    }

    pub fn pop_call(&mut self) {
        self.call_stack.pop();
    }

    pub fn clear_call_stack(&mut self) {
        self.call_stack.clear()
    }

    pub fn is_overflow(&self) -> bool {
        self.call_stack.len() > 100
    }
}

impl Interpreter {
    fn execute_code(
        &mut self,
        execution_environment_id: EnvironmentId,
        code: &str,
    ) -> Result<Value, Error> {
        // first step: parse code
        let code = parse(code).map_err(|err| {
            Error::parse_error(
                format!("Error while parsing code: {:?}", err).as_str(),
            )
        })?;

        // second step: read forms
        let values = read_elements(self, code.get_elements())?;

        // third step: evaluate
        let mut results: Vec<Value> = Vec::new();

        for value in values {
            let result = self.execute_value(execution_environment_id, value)?;

            results.push(result);
        }

        let last_result = match results.last() {
            Some(result) => *result,
            None => self.intern_nil_symbol_value(),
        };

        Ok(last_result)
    }

    pub fn execute_value(
        &mut self,
        environment_id: EnvironmentId,
        value: Value,
    ) -> Result<Value, Error> {
        evaluate_value(self, environment_id, value)
    }

    pub fn execute_builtin_function(
        &mut self,
        builtin_function: &BuiltinFunction,
        execution_environment: EnvironmentId,
        evaluated_arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        evaluate_builtin_function_invocation(
            self,
            builtin_function,
            execution_environment,
            evaluated_arguments,
        )
    }

    pub fn execute_interpreted_function(
        &mut self,
        interpreted_function: &InterpretedFunction,
        evaluated_arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        evaluate_interpreted_function_invocation(
            self,
            interpreted_function,
            evaluated_arguments,
        )
    }

    pub fn execute_function_without_arguments(
        &mut self,
        value: Value,
    ) -> Result<Value, Error> {
        match value {
            Value::Function(function_id) => {
                let nil = self.intern_nil_symbol_value();
                let function_invocation_cons = self.make_cons_value(value, nil);
                let root_environment_id = self.get_main_environment_id();

                self.execute_value(
                    root_environment_id,
                    function_invocation_cons,
                )
            },
            _ => Error::invalid_argument_error("").into(),
        }
    }

    pub fn execute_in_root_environment(
        &mut self,
        code: &str,
    ) -> Result<Value, Error> {
        let root_environment_id = self.get_root_environment_id();

        self.execute_code(root_environment_id, code)
    }

    pub fn execute_in_main_environment(
        &mut self,
        code: &str,
    ) -> Result<Value, Error> {
        let main_environment_id = self.get_main_environment_id();

        self.execute_code(main_environment_id, code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;
    use crate::utils::assertion::assert_deep_equal;

    #[cfg(test)]
    mod evaluation {
        use super::*;

        macro_rules! assert_execution_result_eq {
            ($expected:expr, $code:expr) => {
                let mut interpreter = Interpreter::new();
                let result = interpreter.execute_in_main_environment($code);

                nia_assert_equal($expected, result.unwrap())
            };
        }

        #[test]
        pub fn executes_integer_correctly() {
            assert_execution_result_eq!(Value::Integer(1), "1");
        }

        #[test]
        pub fn executes_float_correctly() {
            assert_execution_result_eq!(Value::Float(1.2), "1.2");
        }

        #[test]
        pub fn executes_boolean_correctly() {
            assert_execution_result_eq!(Value::Boolean(true), "#t");
            assert_execution_result_eq!(Value::Boolean(false), "#f");
        }

        #[test]
        pub fn executes_string_correctly() {
            let mut interpreter = Interpreter::new();

            let expected = interpreter.intern_string_value("tas");
            let result =
                interpreter.execute_in_main_environment(r#""tas""#).unwrap();

            assertion::assert_deep_equal(&mut interpreter, expected, result);
        }

        #[test]
        pub fn executes_symbol_correctly() {
            let mut interpreter = Interpreter::new();
            let name = interpreter.intern_symbol_id("test");
            let root_environment_id = interpreter.get_main_environment_id();

            interpreter
                .environment_arena
                .define_variable(root_environment_id, name, Value::Integer(1))
                .unwrap();

            let result = interpreter.execute_in_main_environment("test");

            nia_assert_equal(Value::Integer(1), result.unwrap());
        }

        #[test]
        pub fn returns_error_during_execution_of_special_symbols() {
            let special_symbol_names = vec!["#opt", "#rest", "#keys"];

            for special_symbol_name in special_symbol_names {
                let mut interpreter = Interpreter::new();
                let symbol_id =
                    interpreter.intern_symbol_id(special_symbol_name);

                let root_environment_id = interpreter.get_main_environment_id();

                interpreter
                    .environment_arena
                    .define_variable(
                        root_environment_id,
                        symbol_id,
                        Value::Integer(1),
                    )
                    .unwrap();

                let result = interpreter
                    .execute_in_main_environment(special_symbol_name);
                nia_assert_is_err(&result);
            }
        }

        #[test]
        pub fn executes_keyword_correctly() {
            let mut interpreter = Interpreter::new();

            let specs = vec![":a", ":b", ":c"];

            for spec in specs {
                let result =
                    interpreter.execute_in_main_environment(spec).unwrap();
                let keyword_id = result.try_into().unwrap();
                let keyword = interpreter.get_keyword(keyword_id).unwrap();

                let keyword_name = keyword.get_name();
                let expected = &spec[1..];

                nia_assert_equal(expected, keyword_name);
            }
        }

        #[test]
        pub fn executes_keyword_s_expression_correctly() {
            let mut interpreter = Interpreter::new();

            let result = interpreter.execute_in_main_environment("(:a {:a 1})");

            nia_assert_equal(Value::Integer(1), result.unwrap());
        }

        #[test]
        fn executes_object_expression_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                ("{:value 1}", "1"),
                ("{:value 1.1}", "1.1"),
                ("{:value #t}", "#t"),
                ("{:value #f}", "#f"),
                ("{:value \"string\"}", "\"string\""),
                ("{:value :keyword}", ":keyword"),
                ("{:value 'symbol}", "'symbol"),
                ("{:value '(list)}", "'(list)"),
                ("{:value {}}", "{}"),
                ("{:value #()}", "#()"),
            ];

            let value_symbol_name = interpreter.intern_symbol_id("value");

            for (code, expected) in pairs {
                let expected =
                    interpreter.execute_in_main_environment(expected).unwrap();

                let result =
                    interpreter.execute_in_main_environment(code).unwrap();

                let object_id = result.try_into().unwrap();

                let result = interpreter
                    .get_object_property(object_id, value_symbol_name)
                    .unwrap()
                    .unwrap();

                assertion::assert_deep_equal(
                    &mut interpreter,
                    expected,
                    result,
                );
            }
        }

        #[cfg(test)]
        mod delimited_symbols {
            use super::*;

            #[test]
            fn executes_correctly() {
                let mut interpreter = Interpreter::new();

                let specs = vec![
                    ("(let ((obj {:value 1})) obj:value)", "1"),
                    ("(let ((obj {:value 1.1})) obj:value)", "1.1"),
                    ("(let ((obj {:value #t})) obj:value)", "#t"),
                    ("(let ((obj {:value #f})) obj:value)", "#f"),
                    (
                        "(let ((obj {:value \"string\"})) obj:value)",
                        "\"string\"",
                    ),
                    ("(let ((obj {:value :keyword})) obj:value)", ":keyword"),
                    ("(let ((obj {:value 'symbol})) obj:value)", "'symbol"),
                    ("(let ((obj {:value {:a 1}})) obj:value)", "{:a 1}"),
                    ("(let ((obj {:value #()})) obj:value)", "#()"),
                ];

                assertion::assert_results_are_equal(&mut interpreter, specs);
            }

            #[test]
            fn executes_sequences_correctly() {
                let mut interpreter = Interpreter::new();

                let specs = vec![
                    ("(let ((obj {:a 1})) obj:a)", "1"),
                    ("(let ((obj {:a {:b 2}})) obj:a:b)", "2"),
                    ("(let ((obj {:a {:b {:c 3}}})) obj:a:b:c)", "3"),
                ];

                assertion::assert_results_are_equal(&mut interpreter, specs);
            }

            #[test]
            fn executes_this_bindings_correctly() {
                let mut interpreter = Interpreter::new();

                let specs = vec![
                    (
                        "(let ((obj {:a 1 :b 2 :c (fn () (+ this:a this:b))})) (obj:c))",
                        "3",
                    ),
                    (
                        "(let ((obj {:a (fn () 1) :b (fn () 2) :c (fn () (+ (this:a) (this:b)))})) (obj:c))",
                        "3",
                    ),
                    (
                        "(defv a {:a (fn () 1) :b (fn () 2) :c (fn () (+ (this:a) (this:b)))}) (a:c)",
                        "3",
                    ),
                    (
                        "(defv b {:a (fn () 1) :b (fn () 2) :c (fn () (+ (this:a) (this:b)))}) (with-this b (this:c))",
                        "3",
                    ),
                ];

                assertion::assert_results_are_equal(&mut interpreter, specs);
            }

            #[test]
            fn executes_super_bindings_correctly() {
                let mut interpreter = Interpreter::new();

                let specs = vec![
                    (
                        r#"
                        (let ((obj-1 (object:make :a (fn () 1)))
                              (obj-2 (object:make :a (fn () (super:a)))))
                          (object:set-proto! obj-2 obj-1)
                          (obj-2:a))
                        "#,
                        "1",
                    ),
                    (
                        r#"
                        (let ((obj-1 (object:make :c (fn () 1) :b (fn () (this:c))))
                              (obj-2 (object:make :a (fn () (super:b)))))
                          (object:set-proto! obj-2 obj-1)
                          (obj-2:a))
                        "#,
                        "1",
                    ),
                    (
                        r#"
                        (let ((obj-1 (object:make :c (fn () 1) :b (fn () (this:c))))
                              (obj-2 (object:make :a (fn () (super:b)) :c (fn () 10))))
                          (object:set-proto! obj-2 obj-1)
                          (obj-2:a))
                        "#,
                        "10",
                    ),
                ];

                assertion::assert_results_are_equal(&mut interpreter, specs);
            }

            // todo: more edge cases
        }

        #[cfg(test)]
        mod short_lambda {
            use super::*;

            #[test]
            fn executes_short_lambda_expressions_correctly() {
                let mut interpreter = Interpreter::new();
                let nil = interpreter.intern_nil_symbol_value();

                let result =
                    interpreter.execute_in_main_environment("(#())").unwrap();
                assert_deep_equal(&mut interpreter, nil, result);

                let result = interpreter
                    .execute_in_main_environment("(#(+ 3 2))")
                    .unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);

                let result = interpreter
                    .execute_in_main_environment("(#(+ %1 2) 1)")
                    .unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(3), result);

                let result = interpreter
                    .execute_in_main_environment("(#(+ %1 %2) 1 3)")
                    .unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(4), result);

                let result = interpreter
                    .execute_in_main_environment("(#(+ 0 %5) 1 2 3 4 5)")
                    .unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);
            }

            #[test]
            fn able_to_use_short_lambda_in_flet() {
                let mut interpreter = Interpreter::new();

                let result = interpreter
                    .execute_in_main_environment(
                        "(flet ((test () #((lookup '%1)))) ((test) #(+ 3 2)))",
                    )
                    .unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);

                let result = interpreter
                    .execute_in_main_environment(
                        "(flet ((test () #((flookup '%1)))) ((test) #(+ 3 2)))",
                    )
                    .unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);

                let result = interpreter
                    .execute_in_main_environment(
                        "(flet ((test () #(%1))) ((test) #(+ 3 2)))",
                    )
                    .unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);
            }
        }

        #[test]
        pub fn builtin_function_works_correctly() {
            let mut interpreter = Interpreter::new();

            let result = interpreter.execute_in_main_environment("(+ 1 2)");
            nia_assert_equal(Value::Integer(3), result.unwrap());

            let result = interpreter.execute_in_main_environment("(+ 1 2.2)");
            nia_assert_equal(Value::Float(3.2), result.unwrap());

            let result = interpreter.execute_in_main_environment("(+ 1.1 2.4)");
            nia_assert_equal(Value::Float(3.5), result.unwrap());

            let result =
                interpreter.execute_in_main_environment("(+ (+ (+ 1 2) 3) 4)");
            nia_assert_equal(Value::Integer(10), result.unwrap());
        }

        #[test]
        pub fn interpreted_function_works_correctly() {
            let mut interpreter = Interpreter::new();
            let root_environment_id = interpreter.get_main_environment_id();

            let a = interpreter.intern_symbol_value("a");
            let b = interpreter.intern_symbol_value("b");
            let plus = interpreter.intern_symbol_value("+");
            let nil = interpreter.intern_nil_symbol_value();

            let value = Value::Cons(interpreter.make_cons(b, nil));

            let value = Value::Cons(interpreter.make_cons(a, value));

            let value = Value::Cons(interpreter.make_cons(plus, value));

            let code = vec![value];

            let name = interpreter.intern_symbol_id("test");
            let mut arguments = FunctionArguments::new();

            arguments.add_ordinary_argument(String::from("a")).unwrap();
            arguments.add_ordinary_argument(String::from("b")).unwrap();

            let function = Function::Interpreted(InterpretedFunction::new(
                root_environment_id,
                arguments,
                code,
            ));

            let function_id = interpreter.register_function(function);

            interpreter
                .environment_arena
                .define_function(
                    root_environment_id,
                    name,
                    Value::Function(function_id),
                )
                .unwrap();

            let result = interpreter.execute_in_main_environment("(test 3 2)");
            nia_assert_equal(Value::Integer(5), result.unwrap());
        }

        #[test]
        fn executes_functions_with_optional_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                (
                    "((function (lambda (#opt a b c) (list a b c))))",
                    "(list nil nil nil)",
                ),
                (
                    "((function (lambda (#opt a b c) (list a b c))) 1)",
                    "(list 1 nil nil)",
                ),
                (
                    "((function (lambda (#opt a b c) (list a b c))) 1 2)",
                    "(list 1 2 nil)",
                ),
                (
                    "((function (lambda (#opt a b c) (list a b c))) 1 2 3)",
                    "(list 1 2 3)",
                ),
                (
                    "((function (lambda (#opt (a 4) (b 5) (c 6)) (list a b c))))",
                    "(list 4 5 6)",
                ),
                (
                    "((function (lambda (#opt (a 4) (b 5) (c 6)) (list a b c))) 1)",
                    "(list 1 5 6)",
                ),
                (
                    "((function (lambda (#opt (a 4) (b 5) (c 6)) (list a b c))) 1 2)",
                    "(list 1 2 6)",
                ),
                (
                    "((function (lambda (#opt (a 4) (b 5) (c 6)) (list a b c))) 1 2 3)",
                    "(list 1 2 3)",
                ),
                (
                    "((function (lambda (#opt (a 3 a?) (b 4 b?)) (list a a? b b?))))",
                    "(list 3 #f 4 #f)",
                ),
                (
                    "((function (lambda (#opt (a 3 a?) (b 4 b?)) (list a a? b b?))) 1)",
                    "(list 1 #t 4 #f)",
                ),
                (
                    "((function (lambda (#opt (a 3 a?) (b 4 b?)) (list a a? b b?))) 1 2)",
                    "(list 1 #t 2 #t)",
                ),
            ];

            assertion::assert_results_are_equal(&mut interpreter, pairs);
        }

        #[test]
        fn executes_functions_with_rest_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                ("((function (lambda (#rest a) a)))", "nil"),
                ("((function (lambda (#rest a) a)) 1)", "(list 1)"),
                ("((function (lambda (#rest a) a)) 1 2)", "(list 1 2)"),
                ("((function (lambda (#rest a) a)) 1 2 3)", "(list 1 2 3)"),
            ];

            assertion::assert_results_are_equal(&mut interpreter, pairs);
        }

        #[test]
        fn executes_functions_with_key_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                (
                    "((function (lambda (#keys a b) (list a b))))",
                    "(list nil nil)",
                ),
                (
                    "((function (lambda (#keys a b) (list a b))) :a 1)",
                    "(list 1 nil)",
                ),
                (
                    "((function (lambda (#keys a b) (list a b))) :b 2)",
                    "(list nil 2)",
                ),
                (
                    "((function (lambda (#keys a b) (list a b))) :a 1 :b 2)",
                    "(list 1 2)",
                ),
                (
                    "((function (lambda (#keys a b) (list a b))) :b 2 :a 1)",
                    "(list 1 2)",
                ),
                (
                    "((function (lambda (#keys (a 3) (b 4)) (list a b))))",
                    "(list 3 4)",
                ),
                (
                    "((function (lambda (#keys (a 3) (b 4)) (list a b))) :a 1)",
                    "(list 1 4)",
                ),
                (
                    "((function (lambda (#keys (a 3) (b 4)) (list a b))) :b 2)",
                    "(list 3 2)",
                ),
                (
                    "((function (lambda (#keys (a 3) (b 4)) (list a b))) :a 1 :b 2)",
                    "(list 1 2)",
                ),
                (
                    "((function (lambda (#keys (a 3) (b 4)) (list a b))) :b 2 :a 1)",
                    "(list 1 2)",
                ),
                (
                    "((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))))",
                    "(list 3 #f 4 #f)",
                ),
                (
                    "((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))) :a 1)",
                    "(list 1 #t 4 #f)",
                ),
                (
                    "((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))) :b 2)",
                    "(list 3 #f 2 #t)",
                ),
                (
                    "((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))) :a 1 :b 2)",
                    "(list 1 #t 2 #t)",
                ),
                (
                    "((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))) :b 2 :a 1)",
                    "(list 1 #t 2 #t)",
                ),
            ];

            assertion::assert_results_are_equal(&mut interpreter, pairs);
        }

        #[test]
        pub fn special_form_invocation_evaluates_correctly() {
            let mut interpreter = Interpreter::new();
            let root_environment_id = interpreter.get_main_environment_id();

            let name = interpreter.intern_symbol_id("testif");
            let function =
                Function::SpecialForm(SpecialFormFunction::new(
                    |interpreter: &mut Interpreter,
                     environment: EnvironmentId,
                     values: Vec<Value>|
                     -> Result<Value, Error> {
                        let mut values = values;

                        let condition = values.remove(0);
                        let then_clause = values.remove(0);
                        let else_clause = values.remove(0);

                        let evaluated_condition =
                            interpreter.execute_value(environment, condition);

                        match evaluated_condition {
                            Ok(Value::Boolean(true)) => interpreter
                                .execute_value(environment, then_clause),
                            Ok(Value::Boolean(false)) => interpreter
                                .execute_value(environment, else_clause),
                            _ => Error::generic_execution_error("").into(),
                        }
                    },
                ));

            let function_id = interpreter.register_function(function);
            let function_value = Value::Function(function_id);

            interpreter
                .environment_arena
                .define_function(root_environment_id, name, function_value)
                .unwrap();

            let pairs = vec![
                ("(testif #t 1 2)", Value::Integer(1)),
                ("(testif #f 1 2)", Value::Integer(2)),
                ("(testif (testif #t #t #f) 1 2)", Value::Integer(1)),
                ("(testif (testif #f #t #f) 1 2)", Value::Integer(2)),
            ];

            for (code, expected) in pairs {
                let result =
                    interpreter.execute_in_main_environment(code).unwrap();

                assertion::assert_deep_equal(
                    &mut interpreter,
                    expected,
                    result,
                );
            }
        }

        #[test]
        pub fn macro_invocation_evaluates_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![(
                "((function (macro (a b c) (list 'list (list 'quote a) (list 'quote b) (list 'quote c)))) aa bb cc)",
                "(list 'aa 'bb 'cc)",
            )];

            assertion::assert_results_are_equal(&mut interpreter, pairs);
        }
    }

    #[test]
    fn handles_stack_overflow() {
        let mut interpreter = Interpreter::new();

        nia_assert_is_ok(
            &interpreter.execute_in_main_environment("(defn a () (a))"),
        );

        let result = interpreter.execute_in_main_environment("(a)");

        assertion::assert_stack_overflow_error(&result);

        // and it continues to work
        let result = interpreter.execute_in_main_environment("(a)");

        assertion::assert_stack_overflow_error(&result);
    }
}
