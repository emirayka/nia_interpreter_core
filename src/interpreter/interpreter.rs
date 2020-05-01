use std::collections::HashMap;
use std::convert::TryInto;

use crate::interpreter::value::Value;
use crate::interpreter::value::Function;
use crate::interpreter::value::InterpretedFunction;
use crate::interpreter::value::BuiltinFunction;
use crate::interpreter::value::SpecialFormFunction;
use crate::interpreter::value::{SymbolId, SymbolArena, Symbol};
use crate::interpreter::value::MacroFunction;
use crate::interpreter::value::ObjectId;
use crate::interpreter::value::ObjectValueWrapper;
use crate::interpreter::value::Object;
use crate::interpreter::value::ObjectArena;
use crate::interpreter::value::{ConsArena, ConsId};
use crate::interpreter::value::{FunctionArena, FunctionId};
use crate::interpreter::value::{StringArena, StringId};
use crate::interpreter::value::NiaString;
use crate::interpreter::value::{KeywordArena, KeywordId};
use crate::interpreter::value::Keyword;
use crate::interpreter::value::FunctionArguments;
use crate::interpreter::environment::EnvironmentArena;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::context::Context;
use crate::interpreter::library;
use crate::parser::parse;
use crate::interpreter::special_variables::SpecialVariableFunction;

use crate::interpreter::reader::read_elements;
use crate::interpreter::stdlib::infect_stdlib;
use crate::interpreter::garbage_collector::collect_garbage;

#[derive(Clone)]
pub struct Interpreter {
    environment_arena: EnvironmentArena,

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

    root_environment: EnvironmentId,
    this_object: Option<ObjectId>,
    is_listening: bool,
}

impl Interpreter {
    pub fn raw() -> Interpreter {
        let mut environment_arena = EnvironmentArena::new();
        let root_environment = environment_arena.alloc();

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
        let this_object = None;

        // nil
        let nil_symbol_id = symbol_arena.intern("nil");
        let nil_value = nil_symbol_id.into();

        environment_arena.define_const_variable(
            root_environment,
            nil_symbol_id,
            nil_value,
        ).expect("Cannot define `nil' symbol.");

        // construct interpreter
        let mut interpreter = Interpreter {
            environment_arena,
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

            root_environment,
            this_object,
            is_listening: false,
        };

        // break
        let break_function = Function::Builtin(BuiltinFunction::new(
            crate::interpreter::internal_functions::_break
        ));
        let break_function_id = interpreter.register_function(
            break_function
        );
        interpreter.internal_functions.insert(
            String::from("break"),
            break_function_id,
        );

        // continue
        let continue_function = Function::Builtin(BuiltinFunction::new(
            crate::interpreter::internal_functions::_continue
        ));
        let continue_function_id = interpreter.register_function(
            continue_function
        );
        interpreter.internal_functions.insert(
            String::from("continue"),
            continue_function_id,
        );

        // special variables
        let this_symbol_id = interpreter.intern("this");
        let super_symbol_id = interpreter.intern("super");

        interpreter.special_variables.insert(
            this_symbol_id,
            crate::interpreter::special_variables::_this,
        );
        interpreter.special_variables.insert(
            super_symbol_id,
            crate::interpreter::special_variables::_super,
        );

        interpreter
    }

    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter::raw();
        // let root_environment = interpreter.get_root_environment();

        match infect_stdlib(&mut interpreter) {
            Ok(()) => {}
            Err(error) => panic!("Cannot construct interpreter: {:?}", error)
        }

        // collect_garbage(&mut interpreter, root_environment)
        //     .expect("Garbage collector is broken");

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
    pub fn print_value(&mut self, value: Value) {
        let string = match value {
            Value::String(string_id) => {
                let vstring = match self.get_string(string_id) {
                    Ok(string) => string,
                    _ => panic!("Cannot print value")
                };

                let mut result = String::from("\"");
                result.push_str(vstring.get_string());
                result.push_str("\"");

                result
            }
            _ => {
                match library::value_to_string(self, value) {
                    Ok(string) => {
                        string
                    }
                    Err(_) => panic!("Cannot print value")
                }
            }
        };

        println!("{}", string)
    }
}

impl Interpreter {
    pub fn get_string_arena(&self) -> &StringArena {
        &self.string_arena
    }

    pub fn free_strings(&mut self, string_ids: Vec<StringId>) -> Result<(), Error> {
        for string_id in string_ids {
            self.string_arena.free_string(string_id)?;
        }

        Ok(())
    }

    pub fn intern_string(&mut self, string: &str) -> StringId {
        self.string_arena.intern_string(string)
    }

    pub fn intern_string_value(&mut self, string: &str) -> Value {
        Value::String(self.intern_string(string))
    }

    pub fn get_string(&self, string_id: StringId) -> Result<&NiaString, Error> {
        self.string_arena
            .get_string(string_id)
    }
}

impl Interpreter {
    pub fn get_keyword_arena(&self) -> &KeywordArena {
        &self.keyword_arena
    }

    pub fn free_keywords(&mut self, keyword_ids: Vec<KeywordId>) -> Result<(), Error> {
        for keyword_id in keyword_ids {
            self.keyword_arena.free_keyword(keyword_id)?;
        }

        Ok(())
    }

    pub fn intern_keyword(&mut self, keyword_name: &str) -> KeywordId {
        self.keyword_arena.intern_keyword(keyword_name)
    }

    pub fn intern_keyword_value(&mut self, keyword_name: &str) -> Value {
        self.intern_keyword(keyword_name)
            .into()
    }

    pub fn get_keyword(&self, keyword_id: KeywordId) -> Result<&Keyword, Error> {
        self.keyword_arena
            .get_keyword(keyword_id)
            .map(|keyword| keyword)
    }
}

impl Interpreter {
    pub fn get_symbol_arena(&self) -> &SymbolArena {
        &self.symbol_arena
    }

    pub fn free_symbols(&mut self, symbol_ids: Vec<SymbolId>) -> Result<(), Error> {
        for symbol_id in symbol_ids {
            self.symbol_arena.free_symbol(symbol_id)?;
        }

        Ok(())
    }

    pub fn get_symbol(&self, symbol_id: SymbolId) -> Result<&Symbol, Error> {
        self.symbol_arena
            .get_symbol(symbol_id)
    }

    pub fn get_symbol_name(&self, symbol_id: SymbolId) -> Result<&String, Error> {
        let symbol = self.get_symbol(symbol_id)?;

        Ok(symbol.get_name())
    }

    pub fn intern(&mut self, symbol_name: &str) -> SymbolId {
        self.symbol_arena.intern(symbol_name)
    }

    pub fn intern_symbol(&mut self, symbol_name: &str) -> &Symbol {
        let symbol_id = self.symbol_arena.intern(symbol_name);

        self.get_symbol(symbol_id).unwrap()
    }

    pub fn intern_symbol_value(&mut self, symbol_name: &str) -> Value {
        Value::Symbol(self.symbol_arena.intern(symbol_name))
    }

    pub fn intern_nil(&mut self) -> SymbolId {
        self.intern("nil")
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

    pub fn symbol_is_not_nil(&mut self, symbol_id: SymbolId) -> Result<bool, Error> {
        let symbol = self.get_symbol(symbol_id)?;

        Ok(symbol.get_name() != "nil" || symbol.get_gensym_id() != 0)
    }

    pub fn check_if_symbol_special(&self, symbol_id: SymbolId) -> Result<bool, Error> {
        let symbol_name = self.get_symbol_name(symbol_id)?;

        let result = symbol_name == "#opt" ||
            symbol_name == "#rest" ||
            symbol_name == "#keys";

        Ok(result)
    }

    pub fn check_if_symbol_constant(&self, symbol_id: SymbolId) -> Result<bool, Error> {
        let symbol_name = self.get_symbol_name(symbol_id)?;

        let result = symbol_name == "nil" ||
            symbol_name == "this" ||
            symbol_name == "super";

        Ok(result)
    }

    pub fn check_if_symbol_assignable(&self, symbol_id: SymbolId) -> Result<bool, Error> {
        let is_not_constant = !self.check_if_symbol_constant(symbol_id)?;
        let is_not_special = !self.check_if_symbol_special(symbol_id)?;

        Ok(is_not_constant && is_not_special)
    }

    pub fn check_if_symbol_internable(&mut self, symbol_id: SymbolId) -> Result<bool, Error> {
        let is_not_special = !self.check_if_symbol_special(symbol_id)?;

        Ok(is_not_special)
    }
}

impl Interpreter {
    pub fn get_cons_arena(&self) -> &ConsArena {
        &self.cons_arena
    }

    pub fn free_cons_cells(&mut self, cons_ids: Vec<ConsId>) -> Result<(), Error> {
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
        self.cons_arena
            .get_car(cons_id)
    }

    pub fn get_cdr(&self, cons_id: ConsId) -> Result<Value, Error> {
        self.cons_arena
            .get_cdr(cons_id)
    }

    pub fn get_cadr(&self, cons_id: ConsId) -> Result<Value, Error> {
        let cdr = self.get_cdr(cons_id)?;

        match cdr {
            Value::Cons(cdr_cons_id) => {
                self.get_car(cdr_cons_id)
            }
            _ => return Error::generic_execution_error(
                "Cannot get car of not a cons value"
            ).into()
        }
    }

    pub fn get_cddr(&self, cons_id: ConsId) -> Result<Value, Error> {
        let cdr = self.get_cdr(cons_id)?;

        match cdr {
            Value::Cons(cdr_cons_id) => {
                self.get_cdr(cdr_cons_id)
            }
            _ => return Error::generic_execution_error(
                "Cannot get cdr of not a cons value"
            ).into()
        }
    }

    pub fn set_car(&mut self, cons_id: ConsId, value: Value) -> Result<(), Error> {
        self.cons_arena
            .set_car(cons_id, value)
    }

    pub fn set_cdr(&mut self, cons_id: ConsId, value: Value) -> Result<(), Error> {
        self.cons_arena
            .set_cdr(cons_id, value)
    }

    pub fn vec_to_list(&mut self, vector: Vec<Value>) -> Value {
        let nil = self.intern_nil_symbol_value();

        self.cons_arena.vec_to_list(nil, vector)
    }

    pub fn list_to_vec(&self, cons_id: ConsId) -> Result<Vec<Value>, Error> {
        let mut vector = self.cons_arena
            .list_to_vec(cons_id)?;

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
            }
            _ => {}
        }

        Ok(vector)
    }
}

impl Interpreter {
    pub fn get_object_arena(&self) -> &ObjectArena {
        &self.object_arena
    }

    pub fn free_objects(&mut self, object_ids: Vec<ObjectId>) -> Result<(), Error> {
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

    pub fn get_object_item(&mut self, object_id: ObjectId, key_symbol_id: SymbolId) -> Result<Option<Value>, Error> {
        self.object_arena
            .get_item(object_id, key_symbol_id)
    }

    pub fn set_object_item(&mut self, object_id: ObjectId, key_symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        self.object_arena
            .set_item(object_id, key_symbol_id, value)
    }

    pub fn get_object_proto(&self, object_id: ObjectId) -> Result<Option<ObjectId>, Error> {
        let object = self.object_arena.get_object(object_id)?;

        Ok(object.get_prototype())
    }

    pub fn set_object_proto(&mut self, object_id: ObjectId, proto_id: ObjectId) -> Result<(), Error> {
        let object = self.object_arena.get_object_mut(object_id)?;
        object.set_prototype(proto_id);

        Ok(())
    }

    pub fn get_object(&self, object_id: ObjectId) -> Result<&Object, Error> {
        let object = self.object_arena.get_object(object_id)?;

        Ok(object)
    }

    pub fn get_object_items(&self, object_id: ObjectId) -> Result<&HashMap<SymbolId, ObjectValueWrapper>, Error> {
        let object = self.object_arena.get_object(object_id)?;

        Ok(object.get_items())
    }
}

impl Interpreter {
    pub fn get_function_arena(&self) -> &FunctionArena {
        &self.function_arena
    }

    pub fn free_functions(&mut self, function_ids: Vec<FunctionId>) -> Result<(), Error> {
        for function_id in function_ids {
            self.function_arena.free_function(function_id)?;
        }

        Ok(())
    }

    pub fn register_function(&mut self, function: Function) -> FunctionId {
        self.function_arena.register_function(function)
    }

    pub fn get_function(&self, function_id: FunctionId) -> Result<&Function, Error> {
        self.function_arena
            .get_function(function_id)
    }

    pub fn get_internal_function(&self, name: &str) -> Result<FunctionId, Error> {
        match self.internal_functions.get(name) {
            Some(function_id) => Ok(*function_id),
            _ => Error::failure(
                format!("Cannot find internal function: {}", name)
            ).into()
        }
    }
}

impl Interpreter {
    pub fn get_environment_arena(&self) -> &EnvironmentArena {
        &self.environment_arena
    }

    pub fn free_environments(&mut self, environment_ids: Vec<EnvironmentId>) -> Result<(), Error> {
        for environment_id in environment_ids {
            self.environment_arena.free_environment(environment_id)?;
        }

        Ok(())
    }

    pub fn get_root_environment(&self) -> EnvironmentId {
        self.root_environment
    }

    pub fn lookup_environment_by_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
    ) -> Result<Option<EnvironmentId>, Error> {
        self.environment_arena.lookup_environment_by_variable(
            environment_id,
            variable_symbol_id,
        )
    }

    pub fn lookup_environment_by_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
    ) -> Result<Option<EnvironmentId>, Error> {
        self.environment_arena.lookup_environment_by_function(
            environment_id,
            function_symbol_id,
        )
    }

    pub fn has_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        self.environment_arena.has_variable(
            environment_id,
            variable_symbol_id,
        )
    }

    pub fn has_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
    ) -> Result<bool, Error> {
        self.environment_arena.has_function(
            environment_id,
            function_symbol_id,
        )
    }

    pub fn define_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena
            .define_variable(environment_id, variable_symbol_id, value)
    }

    pub fn define_const_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena
            .define_const_variable(environment_id, variable_symbol_id, value)
    }

    pub fn define_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena
            .define_function(environment_id, function_symbol_id, value)
    }

    pub fn define_const_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena
            .define_const_function(environment_id, function_symbol_id, value)
    }

    pub fn set_environment_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena
            .set_environment_variable(environment_id, variable_symbol_id, value)
    }

    pub fn set_environment_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena
            .set_environment_function(environment_id, function_symbol_id, value)
    }

    pub fn set_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena
            .set_variable(environment_id, variable_symbol_id, value)
    }

    pub fn set_function(
        &mut self,
        environment_id: EnvironmentId,
        function_symbol_id: SymbolId,
        value: Value,
    ) -> Result<(), Error> {
        self.environment_arena
            .set_function(environment_id, function_symbol_id, value)
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

    pub fn make_environment(&mut self, parent_environment: EnvironmentId) -> Result<EnvironmentId, Error> {
        self.environment_arena
            .alloc_child(parent_environment)
    }

    pub fn remove_environment(&mut self, environment_id: EnvironmentId) -> Result<(), Error> {
        self.environment_arena
            .free_environment(environment_id)
    }

    pub fn get_environment_gc_items(&self, environment_id: EnvironmentId) -> Result<Vec<Value>, Error> {
        self.environment_arena
            .get_environment_gc_items(environment_id)
    }
}

impl Interpreter {
    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn get_context_value(&self, symbol_id: SymbolId) -> Result<Value, Error> {
        self.context
            .get_value(symbol_id)
    }

    pub fn set_context_value(&mut self, symbol_id: SymbolId, value: Value) -> Result<(), Error> {
        self.context
            .set_value(symbol_id, value)
    }
}

impl Interpreter {
    fn evaluate_symbol(
        &self,
        environment_id: EnvironmentId,
        symbol_id: SymbolId,
    ) -> Result<Value, Error> {
        if self.check_if_symbol_special(symbol_id)? {
            return Error::generic_execution_error(
                "Cannot evaluate special symbols."
            ).into();
        }

        let evaluation_result = match self.lookup_variable(
            environment_id,
            symbol_id,
        )? {
            Some(result) => result,
            None => {
                match self.special_variables.get(&symbol_id) {
                    Some(func) => return func(self),
                    None => return Error::generic_execution_error(
                        "Cannot find variable."
                    ).into()
                }
            }
        };

        Ok(evaluation_result)
    }

    fn extract_arguments(&mut self, cons_id: ConsId) -> Result<Vec<Value>, Error> {
        let cdr = self.cons_arena
            .get_cdr(cons_id)?;

        match cdr {
            Value::Cons(cons) => self.list_to_vec(cons),
            Value::Symbol(symbol_id) => {
                if self.symbol_is_nil(symbol_id)? {
                    Ok(Vec::new())
                } else {
                    Error::generic_execution_error(
                        "Cannot extract arguments from not a list."
                    ).into()
                }
            }
            _ => Error::generic_execution_error(
                "Cannot extract arguments from not a list."
            ).into()
        }
    }

    fn evaluate_arguments(
        &mut self,
        environment_id: EnvironmentId,
        arguments: Vec<Value>,
    ) -> Result<Vec<Value>, Error> {
        let mut evaluated_arguments = Vec::new();

        for argument in arguments {
            let evaluated_argument = self
                .evaluate_value(environment_id, argument)
                .map_err(|err|
                    Error::generic_execution_error_caused(
                        "Cannot evaluate arguments.",
                        err,
                    )
                )?;

            evaluated_arguments.push(evaluated_argument)
        }

        Ok(evaluated_arguments)
    }

    fn define_environment_variables(
        &mut self,
        execution_environment_id: EnvironmentId,
        arguments: &FunctionArguments,
        values: &Vec<Value>,
    ) -> Result<(), Error> {
        let mut current_argument = 0;

        // ordinary
        for variable_name in arguments.get_ordinary_arguments().iter() {
            let variable_symbol_id = self.intern(variable_name);
            let value = values[current_argument];

            self.define_variable(execution_environment_id, variable_symbol_id, value)?;

            current_argument += 1;
        }

        // optional
        for optional_argument in arguments.get_optional_arguments().iter() {
            let variable_symbol_id = self.intern(optional_argument.get_name());

            if current_argument < values.len() {
                let value = values[current_argument];

                self.define_variable(execution_environment_id, variable_symbol_id, value)?;

                if let Some(provided_name) = optional_argument.get_provided() {
                    let variable_symbol_id = self.intern(provided_name);

                    self.define_variable(
                        execution_environment_id,
                        variable_symbol_id,
                        Value::Boolean(true),
                    )?;
                }

                current_argument += 1;
            } else {
                let value = match optional_argument.get_default() {
                    Some(default_value) => {
                        self.evaluate_value(execution_environment_id, default_value)?
                    }
                    None => self.intern_nil_symbol_value()
                };

                self.define_variable(execution_environment_id, variable_symbol_id, value)?;

                if let Some(provided_name) = optional_argument.get_provided() {
                    let variable_symbol_id = self.intern(provided_name);

                    self.define_variable(
                        execution_environment_id,
                        variable_symbol_id,
                        Value::Boolean(false),
                    )?;
                }
            }
        }

        // rest
        if let Some(rest_argument_name) = arguments.get_rest_argument() {
            let variable_symbol_id = self.intern(rest_argument_name);

            let rest_values_slice = &values[current_argument..];
            let rest_values = Vec::from(rest_values_slice);
            let rest_values_cons = self.vec_to_list(rest_values);

            self.define_variable(
                execution_environment_id,
                variable_symbol_id,
                rest_values_cons,
            )?;

            return Ok(());
        }

        let values = &values[current_argument..];
        let mut current_argument = 0;

        // key arguments
        if arguments.get_key_arguments().len() != 0 {
            if values.len() % 2 != 0 {
                return Error::generic_execution_error(
                    "Invalid usage of key arguments."
                ).into();
            }

            for key_argument in arguments.get_key_arguments() {
                let variable_symbol_id = self.intern(key_argument.get_name());
                let value = self.exclusive_nil_value;

                self.define_variable(execution_environment_id, variable_symbol_id, value)?;
            }

            loop {
                if current_argument >= values.len() {
                    break;
                }

                let keyword = values[current_argument];

                let variable_symbol_id = if let Value::Keyword(keyword_id) = keyword {
                    let keyword_name = self.get_keyword(keyword_id)?
                        .get_name().clone();

                    self.intern(&keyword_name)
                } else {
                    return Error::generic_execution_error("")
                        .into();
                };

                let value = values[current_argument + 1];

                self.set_environment_variable(execution_environment_id, variable_symbol_id, value)?;

                current_argument += 2;
            }

            for key_argument in arguments.get_key_arguments() {
                let variable_symbol_id = self.intern(key_argument.get_name());
                let looked_up_variable = match self.lookup_variable(
                    execution_environment_id,
                    variable_symbol_id,
                )? {
                    Some(variable_value) => variable_value,
                    None => return Error::generic_execution_error("Cannot find variable")
                        .into()
                };

                if looked_up_variable == self.exclusive_nil_value {
                    let value = if let Some(default_value) = key_argument.get_default() {
                        self.evaluate_value(execution_environment_id, default_value)?
                    } else {
                        self.intern_nil_symbol_value()
                    };

                    self.set_environment_variable(execution_environment_id, variable_symbol_id, value)?;

                    if let Some(provided_name) = key_argument.get_provided() {
                        let variable_symbol_id = self.intern(provided_name);
                        let value = Value::Boolean(false);

                        self.define_variable(execution_environment_id, variable_symbol_id, value)?;
                    }
                } else {
                    if let Some(provided_name) = key_argument.get_provided() {
                        let variable_symbol_id = self.intern(provided_name);
                        let value = Value::Boolean(true);

                        self.define_variable(execution_environment_id, variable_symbol_id, value)?;
                    }
                }
            }
        }

        if values.len() > current_argument {
            return Error::generic_execution_error(
                "Function was called with too many arguments."
            ).into();
        } else if values.len() < current_argument {
            return Error::generic_execution_error(
                "Function was called with too little arguments."
            ).into();
        }

        Ok(())
    }

    fn define_environment_functions(
        &mut self,
        execution_environment_id: EnvironmentId,
        arguments: &FunctionArguments,
        values: &Vec<Value>,
    ) -> Result<(), Error> {
        let mut current_argument = 0;

        // ordinary
        for function_name in arguments.get_ordinary_arguments().iter() {
            let function_symbol_id = self.intern(function_name);
            let value = values[current_argument];

            self
                .define_function(execution_environment_id, function_symbol_id, value)?;

            current_argument += 1;
        }

        // optional
        for optional_argument in arguments.get_optional_arguments().iter() {
            let function_symbol_id = self.intern(optional_argument.get_name());

            if current_argument < values.len() {
                let value = values[current_argument];

                self.define_function(execution_environment_id, function_symbol_id, value)?;

                if let Some(provided_name) = optional_argument.get_provided() {
                    let function_symbol_id = self.intern(provided_name);

                    self.define_function(execution_environment_id, function_symbol_id, Value::Boolean(true))?;
                }

                current_argument += 1;
            } else {
                let value = match optional_argument.get_default() {
                    Some(default_value) => {
                        self.evaluate_value(execution_environment_id, default_value)?
                    }
                    None => self.intern_nil_symbol_value()
                };

                self.define_function(execution_environment_id, function_symbol_id, value)?;

                if let Some(provided_name) = optional_argument.get_provided() {
                    let function_symbol_id = self.intern(provided_name);

                    self.define_function(execution_environment_id, function_symbol_id, Value::Boolean(false))?;
                }
            }
        }

        Ok(())
    }

    fn execute_code(&mut self, execution_environment: EnvironmentId, code: &Vec<Value>) -> Result<Option<Value>, Error> {
        let mut last_result = None;

        for value in code {
            last_result = self.evaluate_value(execution_environment, *value)
                .map(|v| Some(v))?;
        }

        Ok(last_result)
    }

    pub fn evaluate_interpreted_function_invocation(
        &mut self,
        func: &InterpretedFunction,
        evaluated_arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        if func.get_arguments().required_len() > evaluated_arguments.len() {
            return Error::generic_execution_error(
                "Not enough arguments to call a function."
            ).into();
        }

        // 1) make new environment
        let execution_environment_id = self.make_environment(
            func.get_environment()
        )?;

        // 2) setup environment variables and functions
        self.define_environment_variables(
            execution_environment_id,
            func.get_arguments(),
            &evaluated_arguments,
        )?;

        self.define_environment_functions(
            execution_environment_id,
            func.get_arguments(),
            &evaluated_arguments,
        )?;

        // 3) execute code
        let execution_result = self.execute_code(
            execution_environment_id,
            func.get_code(),
        )?;

        // 4) return result
        let result = execution_result.unwrap_or_else(|| self.intern_nil_symbol_value());

        Ok(result)
    }

    pub fn evaluate_builtin_function_invocation(
        &mut self,
        builtin_function: &BuiltinFunction,
        execution_environment: EnvironmentId,
        evaluated_arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        (builtin_function.get_func())(self, execution_environment, evaluated_arguments)
    }

    pub fn evaluate_special_form_invocation(
        &mut self,
        execution_environment: EnvironmentId,
        special_form: &SpecialFormFunction,
        arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        (special_form.get_func())(self, execution_environment, arguments)
    }

    pub fn evaluate_macro_invocation(
        &mut self,
        func: &MacroFunction,
        arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        if func.get_arguments().required_len() > arguments.len() {
            return Error::generic_execution_error(
                "Not enough arguments to call a macro."
            ).into();
        }

        // 1) make new environment
        let execution_environment_id = self.make_environment(
            func.get_environment()
        )?;

        // 2) set arguments in that environment
        self.define_environment_variables(
            execution_environment_id,
            func.get_arguments(),
            &arguments,
        )?;

        self.define_environment_functions(
            execution_environment_id,
            func.get_arguments(),
            &arguments,
        )?;

        // 3) execute code
        let execution_result = self.execute_code(
            execution_environment_id,
            func.get_code(),
        )?;

        // 4) return result
        let result = execution_result.unwrap_or_else(|| self.intern_nil_symbol_value());

        Ok(result)
    }

    pub fn evaluate_s_expression_function_invocation(
        &mut self,
        environment_id: EnvironmentId,
        function: FunctionId,
        cons_id: ConsId,
    ) -> Result<Value, Error> {
        let function = self.get_function(function)
            .map(|function| function.clone())?;

        match function {
            Function::Builtin(builtin_function) => {
                // 2) evaluate arguments
                let arguments = self.extract_arguments(cons_id)?;
                let evaluated_arguments = self.evaluate_arguments(environment_id, arguments)?;

                // 3) apply function from step 1 to arguments from step 2
                self.evaluate_builtin_function_invocation(
                    &builtin_function,
                    environment_id,
                    evaluated_arguments,
                )
            }
            Function::Interpreted(interpreted_function) => {
                // 2) evaluate arguments
                let arguments = self.extract_arguments(cons_id)?;
                let evaluated_arguments = self.evaluate_arguments(environment_id, arguments)?;

                // 3) apply function from step 1 to arguments from step 2
                self.evaluate_interpreted_function_invocation(
                    &interpreted_function,
                    evaluated_arguments,
                )
            }
            Function::SpecialForm(special_form) => {
                let arguments = self.extract_arguments(cons_id)?;

                self.evaluate_special_form_invocation(
                    environment_id,
                    &special_form,
                    arguments,
                )
            }
            Function::Macro(macro_function) => {
                let arguments = self.extract_arguments(cons_id)?;
                let evaluation_result = self.evaluate_macro_invocation(&macro_function, arguments)?;

                self.evaluate_value(environment_id, evaluation_result)
            }
        }
    }

    fn evaluate_s_expression_keyword(
        &mut self,
        environment_id: EnvironmentId,
        keyword_id: KeywordId,
        cons_id: ConsId,
    ) -> Result<Value, Error> {
        let keyword_name = self.get_keyword(keyword_id)
            .map(|keyword| keyword.get_name().clone())?;

        let symbol_id = self.intern(&keyword_name);

        let mut arguments = self.extract_arguments(cons_id)?;

        if arguments.len() != 1 {
            return Error::generic_execution_error(
                "Invalid argument count in keyword s-expression."
            ).into();
        }

        let argument = arguments.remove(0);

        let evaluated_argument = self.evaluate_value(
            environment_id,
            argument,
        )?;

        match evaluated_argument {
            Value::Object(object_id) => {
                self.object_arena.get_item(object_id, symbol_id)?
                    .ok_or_else(|| Error::generic_execution_error(
                        "Object have not an item to yield."
                    ))
            }
            _ => return Error::generic_execution_error(
                "Cannot get an item of not an object."
            ).into()
        }
    }

    fn evaluate_s_expression(
        &mut self,
        environment_id: EnvironmentId,
        s_expression: ConsId,
    ) -> Result<Value, Error> {
        // 1) evaluate first symbol
        let car = self.cons_arena
            .get_car(s_expression)?;

        match car {
            Value::Symbol(func_symbol_id) => {
                let function_value = match self.lookup_function(
                    environment_id,
                    func_symbol_id,
                )? {
                    Some(function_value) => function_value,
                    None => {
                        return Error::generic_execution_error("Cannot find function.")
                            .into();
                    }
                };

                let function_id = match function_value {
                    Value::Function(function_id) => function_id,
                    _ => return Error::generic_execution_error(
                        "The result of evaluation of first item of an s-expression must be a function or keyword."
                    ).into(),
                };

                self.evaluate_s_expression_function_invocation(
                    environment_id,
                    function_id,
                    s_expression,
                )
            }
            Value::Function(function_id) => self.evaluate_s_expression_function_invocation(
                environment_id,
                function_id,
                s_expression,
            ),
            Value::Cons(cons_id) => {
                let evaluation_result = self.evaluate_s_expression(
                    environment_id,
                    cons_id,
                )?;

                let function_id = match evaluation_result {
                    Value::Function(function_id) => function_id,
                    _ => return Error::generic_execution_error(
                        "."
                    ).into(),
                };

                self.evaluate_s_expression_function_invocation(
                    environment_id,
                    function_id,
                    s_expression,
                )
            }
            Value::Keyword(keyword_id) => self.evaluate_s_expression_keyword(
                environment_id,
                keyword_id,
                s_expression,
            ),
            _ => return Error::generic_execution_error(
                "The result of evaluation of first item of an s-expression must be a function or keyword."
            ).into(),
        }
    }

    pub fn evaluate_value(&mut self, environment: EnvironmentId, value: Value) -> Result<Value, Error> {
        match value {
            Value::Symbol(symbol_name) => self.evaluate_symbol(environment, symbol_name),
            Value::Cons(cons) => self.evaluate_s_expression(environment, cons),
            _ => Ok(value)
        }
    }
}

impl Interpreter {
    pub fn execute_value(&mut self, environment: EnvironmentId, value: Value) -> Result<Value, Error> {
        self.evaluate_value(environment, value)
    }

    pub fn execute_function(&mut self, value: Value) -> Result<Value, Error> {
        match value {
            Value::Function(function_id) => {
                let nil = self.intern_nil_symbol_value();
                let function_invocation_cons = self.make_cons_value(value, nil);
                let root_environment_id = self.get_root_environment();

                self.execute_value(root_environment_id, function_invocation_cons)
            }
            _ => Error::invalid_argument_error("")
                .into()
        }
    }

    pub fn execute(&mut self, code: &str) -> Result<Value, Error> {
        // first step: parse code
        let code = parse(code)
            .map(|result| result.1)
            .map_err(|err| Error::parse_error(
                format!("Error while parsing code: {:?}", err)
                    .as_str()
            ))?;

        // second step: read forms
        let values = read_elements(self, code.get_elements())?;

        // third step: evaluate
        let mut results: Vec<Value> = Vec::new();

        for value in values {
            let result = self.execute_value(self.root_environment, value)?;

            results.push(result);
        }

        let last_result = match results.last() {
            Some(result) => *result,
            None => self.intern_nil_symbol_value()
        };

        Ok(last_result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::testing_helpers::make_value_pairs_evaluated_ifbsyko;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::assertion::assert_deep_equal;

    #[cfg(test)]
    mod evaluation {
        use super::*;

        macro_rules! assert_execution_result_eq {
            ($expected:expr, $code:expr) => {
                let mut interpreter = Interpreter::new();
                let result = interpreter.execute($code);

                assert_eq!($expected, result.unwrap())
            }
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
            let result = interpreter.execute(r#""tas""#).unwrap();

            assertion::assert_deep_equal(
                &mut interpreter,
                expected,
                result,
            );
        }

        #[test]
        pub fn executes_symbol_correctly() {
            let mut interpreter = Interpreter::new();
            let name = interpreter.intern("test");

            interpreter.environment_arena.define_variable(
                interpreter.root_environment,
                name,
                Value::Integer(1),
            ).unwrap();

            let result = interpreter.execute("test");

            assert_eq!(Value::Integer(1), result.unwrap());
        }

        #[test]
        pub fn returns_error_during_execution_of_special_symbols() {
            let special_symbol_names = vec!(
                "#opt",
                "#rest",
                "#keys",
            );
            for special_symbol_name in special_symbol_names {
                let mut interpreter = Interpreter::new();
                let symbol_id = interpreter.intern(special_symbol_name);

                interpreter.environment_arena.define_variable(
                    interpreter.root_environment,
                    symbol_id,
                    Value::Integer(1),
                ).unwrap();

                let result = interpreter.execute(special_symbol_name);
                assertion::assert_is_error(&result);
            }
        }

        #[test]
        pub fn executes_keyword_correctly() {
            let mut interpreter = Interpreter::new();

            let specs = vec!(
                ":a",
                ":b",
                ":c",
            );

            for spec in specs {
                let result = interpreter.execute(spec).unwrap();
                let keyword_id = result.try_into()
                    .unwrap();
                let keyword = interpreter.get_keyword(keyword_id)
                    .unwrap();

                let keyword_name = keyword.get_name();
                let expected = &spec[1..];

                assert_eq!(expected, keyword_name);
            }
        }

        #[test]
        pub fn executes_keyword_s_expression_correctly() {
            let mut interpreter = Interpreter::new();

            let result = interpreter.execute("(:a {:a 1})");

            assert_eq!(Value::Integer(1), result.unwrap());
        }

        #[test]
        fn executes_object_expression_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = make_value_pairs_evaluated_ifbsyko(&mut interpreter);

            let key = interpreter.intern("value");

            for pair in pairs {
                let code = String::from("{:value ") + &pair.0 + "}";
                let result = interpreter.execute(&code);

                let object_id = match result {
                    Ok(Value::Object(object_id)) => {
                        object_id
                    }
                    _ => panic!()
                };

                let expected = pair.1;
                let result = interpreter.get_object_item(object_id, key).unwrap().unwrap();

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

                let specs = vec!(
                    ("(let ((obj {:value 1})) obj:value)", "1"),
                    ("(let ((obj {:value 1.1})) obj:value)", "1.1"),
                    ("(let ((obj {:value #t})) obj:value)", "#t"),
                    ("(let ((obj {:value #f})) obj:value)", "#f"),
                    ("(let ((obj {:value \"string\"})) obj:value)", "\"string\""),
                    ("(let ((obj {:value :keyword})) obj:value)", ":keyword"),
                    ("(let ((obj {:value 'symbol})) obj:value)", "'symbol"),
                    ("(let ((obj {:value {:a 1}})) obj:value)", "{:a 1}"),
                    ("(let ((obj {:value #()})) obj:value)", "#()"),
                );

                assertion::assert_results_are_equal(
                    &mut interpreter,
                    specs,
                );
            }

            #[test]
            fn executes_sequences_correctly() {
                let mut interpreter = Interpreter::new();

                let specs = vec!(
                    ("(let ((obj {:a 1})) obj:a)", "1"),
                    ("(let ((obj {:a {:b 2}})) obj:a:b)", "2"),
                    ("(let ((obj {:a {:b {:c 3}}})) obj:a:b:c)", "3"),
                );

                assertion::assert_results_are_equal(
                    &mut interpreter,
                    specs,
                );
            }

            #[test]
            fn executes_this_bindings_correctly() {
                let mut interpreter = Interpreter::new();

                let specs = vec!(
                    ("(let ((obj {:a 1 :b 2 :c (fn () (+ this:a this:b))})) (obj:c))", "3"),
                    ("(let ((obj {:a (fn () 1) :b (fn () 2) :c (fn () (+ (this:a) (this:b)))})) (obj:c))", "3"),
                    ("(defv a {:a (fn () 1) :b (fn () 2) :c (fn () (+ (this:a) (this:b)))}) (a:c)", "3"),
                    ("(defv b {:a (fn () 1) :b (fn () 2) :c (fn () (+ (this:a) (this:b)))}) (with-this b (this:c))", "3"),
                );

                assertion::assert_results_are_equal(
                    &mut interpreter,
                    specs,
                );
            }

            #[test]
            fn executes_super_bindings_correctly() {
                let mut interpreter = Interpreter::new();

                let specs = vec!(
                    (
                        r#"
                        (let ((obj-1 (object:make :a (fn () 1)))
                              (obj-2 (object:make :a (fn () (super:a)))))
                          (object:set-proto! obj-2 obj-1)
                          (obj-2:a))
                        "#,
                        "1"
                    ),
                    (
                        r#"
                        (let ((obj-1 (object:make :c (fn () 1) :b (fn () (this:c))))
                              (obj-2 (object:make :a (fn () (super:b)))))
                          (object:set-proto! obj-2 obj-1)
                          (obj-2:a))
                        "#,
                        "1"
                    ),
                    (
                        r#"
                        (let ((obj-1 (object:make :c (fn () 1) :b (fn () (this:c))))
                              (obj-2 (object:make :a (fn () (super:b)) :c (fn () 10))))
                          (object:set-proto! obj-2 obj-1)
                          (obj-2:a))
                        "#,
                        "10"
                    ),
                );

                assertion::assert_results_are_equal(
                    &mut interpreter,
                    specs,
                );
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

                let result = interpreter.execute("(#())").unwrap();
                assert_deep_equal(&mut interpreter, nil, result);

                let result = interpreter.execute("(#(+ 3 2))").unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);

                let result = interpreter.execute("(#(+ %1 2) 1)").unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(3), result);

                let result = interpreter.execute("(#(+ %1 %2) 1 3)").unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(4), result);

                let result = interpreter.execute("(#(+ 0 %5) 1 2 3 4 5)").unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);
            }

            #[test]
            fn able_to_use_short_lambda_in_flet() {
                let mut interpreter = Interpreter::new();

                let result = interpreter.execute("(flet ((test () #((lookup '%1)))) ((test) #(+ 3 2)))").unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);

                let result = interpreter.execute("(flet ((test () #((flookup '%1)))) ((test) #(+ 3 2)))").unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);

                let result = interpreter.execute("(flet ((test () #(%1))) ((test) #(+ 3 2)))").unwrap();
                assert_deep_equal(&mut interpreter, Value::Integer(5), result);
            }
        }

        #[test]
        pub fn builtin_function_works_correctly() {
            let mut interpreter = Interpreter::new();

            let result = interpreter.execute("(+ 1 2)");
            assert_eq!(Value::Integer(3), result.unwrap());

            let result = interpreter.execute("(+ 1 2.2)");
            assert_eq!(Value::Float(3.2), result.unwrap());

            let result = interpreter.execute("(+ 1.1 2.4)");
            assert_eq!(Value::Float(3.5), result.unwrap());

            let result = interpreter.execute("(+ (+ (+ 1 2) 3) 4)");
            assert_eq!(Value::Integer(10), result.unwrap());
        }

        #[test]
        pub fn interpreted_function_works_correctly() {
            let mut interpreter = Interpreter::new();

            let a = interpreter.intern_symbol_value("a");
            let b = interpreter.intern_symbol_value("b");
            let plus = interpreter.intern_symbol_value("+");
            let nil = interpreter.intern_nil_symbol_value();

            let value = Value::Cons(interpreter.make_cons(
                b,
                nil,
            ));

            let value = Value::Cons(interpreter.make_cons(
                a,
                value,
            ));

            let value = Value::Cons(interpreter.make_cons(
                plus,
                value,
            ));

            let code = vec!(
                value
            );

            let name = interpreter.intern("test");
            let mut arguments = FunctionArguments::new();

            arguments.add_ordinary_argument(String::from("a")).unwrap();
            arguments.add_ordinary_argument(String::from("b")).unwrap();

            let function = Function::Interpreted(InterpretedFunction::new(
                interpreter.root_environment,
                arguments,
                code,
            ));

            let function_id = interpreter.register_function(function);

            interpreter.environment_arena.define_function(
                interpreter.root_environment,
                name,
                Value::Function(function_id),
            ).unwrap();

            let result = interpreter.execute("(test 3 2)");
            assert_eq!(Value::Integer(5), result.unwrap());
        }

        #[test]
        fn executes_functions_with_optional_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("((function (lambda (#opt a b c) (list a b c))))", "(list nil nil nil)"),
                ("((function (lambda (#opt a b c) (list a b c))) 1)", "(list 1 nil nil)"),
                ("((function (lambda (#opt a b c) (list a b c))) 1 2)", "(list 1 2 nil)"),
                ("((function (lambda (#opt a b c) (list a b c))) 1 2 3)", "(list 1 2 3)"),
                ("((function (lambda (#opt (a 4) (b 5) (c 6)) (list a b c))))", "(list 4 5 6)"),
                ("((function (lambda (#opt (a 4) (b 5) (c 6)) (list a b c))) 1)", "(list 1 5 6)"),
                ("((function (lambda (#opt (a 4) (b 5) (c 6)) (list a b c))) 1 2)", "(list 1 2 6)"),
                ("((function (lambda (#opt (a 4) (b 5) (c 6)) (list a b c))) 1 2 3)", "(list 1 2 3)"),
                ("((function (lambda (#opt (a 3 a?) (b 4 b?)) (list a a? b b?))))", "(list 3 #f 4 #f)"),
                ("((function (lambda (#opt (a 3 a?) (b 4 b?)) (list a a? b b?))) 1)", "(list 1 #t 4 #f)"),
                ("((function (lambda (#opt (a 3 a?) (b 4 b?)) (list a a? b b?))) 1 2)", "(list 1 #t 2 #t)"),
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs,
            );
        }

        #[test]
        fn executes_functions_with_rest_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("((function (lambda (#rest a) a)))", "nil"),
                ("((function (lambda (#rest a) a)) 1)", "(list 1)"),
                ("((function (lambda (#rest a) a)) 1 2)", "(list 1 2)"),
                ("((function (lambda (#rest a) a)) 1 2 3)", "(list 1 2 3)"),
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs,
            );
        }

        #[test]
        fn executes_functions_with_key_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("((function (lambda (#keys a b) (list a b))))", "(list nil nil)"),
                ("((function (lambda (#keys a b) (list a b))) :a 1)", "(list 1 nil)"),
                ("((function (lambda (#keys a b) (list a b))) :b 2)", "(list nil 2)"),
                ("((function (lambda (#keys a b) (list a b))) :a 1 :b 2)", "(list 1 2)"),
                ("((function (lambda (#keys a b) (list a b))) :b 2 :a 1)", "(list 1 2)"),
                ("((function (lambda (#keys (a 3) (b 4)) (list a b))))", "(list 3 4)"),
                ("((function (lambda (#keys (a 3) (b 4)) (list a b))) :a 1)", "(list 1 4)"),
                ("((function (lambda (#keys (a 3) (b 4)) (list a b))) :b 2)", "(list 3 2)"),
                ("((function (lambda (#keys (a 3) (b 4)) (list a b))) :a 1 :b 2)", "(list 1 2)"),
                ("((function (lambda (#keys (a 3) (b 4)) (list a b))) :b 2 :a 1)", "(list 1 2)"),
                ("((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))))", "(list 3 #f 4 #f)"),
                ("((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))) :a 1)", "(list 1 #t 4 #f)"),
                ("((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))) :b 2)", "(list 3 #f 2 #t)"),
                ("((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))) :a 1 :b 2)", "(list 1 #t 2 #t)"),
                ("((function (lambda (#keys (a 3 a?) (b 4 b?)) (list a a? b b?))) :b 2 :a 1)", "(list 1 #t 2 #t)"),
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs,
            );
        }

        #[test]
        pub fn special_form_invocation_evaluates_correctly() {
            let mut interpreter = Interpreter::new();

            let name = interpreter.intern("testif");
            let function = Function::SpecialForm(SpecialFormFunction::new(
                |interpreter: &mut Interpreter, environment: EnvironmentId, values: Vec<Value>| -> Result<Value, Error> {
                    let mut values = values;

                    let condition = values.remove(0);
                    let then_clause = values.remove(0);
                    let else_clause = values.remove(0);

                    let evaluated_condition = interpreter.evaluate_value(environment, condition);

                    match evaluated_condition {
                        Ok(Value::Boolean(true)) => interpreter.evaluate_value(environment, then_clause),
                        Ok(Value::Boolean(false)) => interpreter.evaluate_value(environment, else_clause),
                        _ => Error::generic_execution_error("").into()
                    }
                }
            ));

            let function_id = interpreter.register_function(function);
            let function_value = Value::Function(function_id);

            interpreter.environment_arena.define_function(
                interpreter.root_environment,
                name,
                function_value,
            ).unwrap();

            let pairs = vec!(
                ("(testif #t 1 2)", Value::Integer(1)),
                ("(testif #f 1 2)", Value::Integer(2)),
                ("(testif (testif #t #t #f) 1 2)", Value::Integer(1)),
                ("(testif (testif #f #t #f) 1 2)", Value::Integer(2)),
            );

            for (code, expected) in pairs {
                let result = interpreter.execute(code).unwrap();

                assertion::assert_deep_equal(&mut interpreter, expected, result);
            }
        }

        #[test]
        pub fn macro_invocation_evaluates_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("((function (macro (a b c) (list 'list (list 'quote a) (list 'quote b) (list 'quote c)))) aa bb cc)", "(list 'aa 'bb 'cc)")
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs,
            );
        }
    }
}