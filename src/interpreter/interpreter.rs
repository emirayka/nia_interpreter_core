use crate::interpreter::value::Value;
use crate::parser::parse_code;
use crate::interpreter::reader::read_elements;
use crate::interpreter::function::{Function};
use crate::interpreter::function::interpreted_function::InterpretedFunction;
use crate::interpreter::function::builtin_function::BuiltinFunction;
use crate::interpreter::function::special_form_function::SpecialFormFunction;
use crate::interpreter::symbol::{SymbolId, SymbolArena, Symbol};
use crate::interpreter::function::macro_function::MacroFunction;
use crate::interpreter::error::Error;

use crate::interpreter::stdlib::infect_stdlib;
use crate::interpreter::environment::environment_arena::{EnvironmentArena, EnvironmentId};
use crate::interpreter::object::object_arena::ObjectArena;
use crate::interpreter::object::object::ObjectId;
use crate::interpreter::cons::cons_arena::{ConsArena, ConsId};
use crate::interpreter::function::function_arena::{FunctionArena, FunctionId};
use crate::interpreter::string::string_arena::{StringArena, StringId};
use crate::interpreter::string::string::VString;
use crate::interpreter::keyword::keyword_arena::{KeywordArena, KeywordId};
use crate::interpreter::keyword::keyword::Keyword;

pub struct Interpreter {
    environment_arena: EnvironmentArena,

    string_arena: StringArena,
    keyword_arena: KeywordArena,
    symbol_arena: SymbolArena,
    cons_arena: ConsArena,
    object_arena: ObjectArena,
    function_arena: FunctionArena,

    root_environment: EnvironmentId,
    call_stack: (),
}

impl Interpreter {
    pub fn raw() -> Interpreter {
        let mut environment_arena = EnvironmentArena::new();
        let root_env_id = environment_arena.alloc();

        let string_arena = StringArena::new();
        let keyword_arena = KeywordArena::new();
        let symbol_arena = SymbolArena::new();
        let cons_arena = ConsArena::new();
        let object_arena = ObjectArena::new();
        let function_arena = FunctionArena::new();

        Interpreter {
            environment_arena,
            string_arena,
            keyword_arena,
            symbol_arena,
            cons_arena,
            object_arena,
            function_arena,
            root_environment: root_env_id,
            call_stack: (),
        }
    }

    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter::raw();

        match infect_stdlib(&mut interpreter) {
            Ok(()) => interpreter,
            Err(_error) => panic!("Cannot construct interpreter")
        }
    }
}

impl Interpreter {
    pub fn make_empty_error(&mut self) -> Error {
        let symbol = self.intern_nil();

        Error::generic_error(symbol, "")
    }

    pub fn make_generic_error(&mut self, symbol: SymbolId, message: &str) -> Error {
        Error::generic_error(symbol, message)
    }

    pub fn make_generic_execution_error(&mut self, message: &str) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_GENERIC_EXECUTION_ERROR
        );

        Error::generic_execution_error(symbol, message)
    }

    pub fn make_generic_execution_error_caused(&mut self, message: &str, cause: Error) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_GENERIC_EXECUTION_ERROR
        );

        Error::generic_execution_error_caused(symbol, message, cause)
    }

    pub fn make_overflow_error(&mut self, message: &str) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_OVERFLOW_ERROR
        );

        Error::overflow_error(symbol, message)
    }

    pub fn make_overflow_error_caused(&mut self, message: &str, cause: Error) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_OVERFLOW_ERROR
        );

        Error::overflow_error_caused(symbol, message, cause)
    }

    pub fn make_zero_division_error(&mut self, message: &str) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_ZERO_DIVISION_ERROR
        );

        Error::zero_division_error(symbol, message)
    }

    pub fn make_zero_division_error_caused(&mut self, message: &str, cause: Error) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_ZERO_DIVISION_ERROR
        );

        Error::zero_division_error_caused(symbol, message, cause)
    }

    pub fn make_invalid_cons_error(&mut self, message: &str) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_INVALID_CONS_ERROR
        );

        Error::invalid_cons_error(symbol, message)
    }

    pub fn make_invalid_cons_error_caused(&mut self, message: &str, cause: Error) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_INVALID_CONS_ERROR
        );

        Error::invalid_cons_error_caused(symbol, message, cause)
    }

    pub fn make_invalid_argument_error(&mut self, message: &str) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_INVALID_ARGUMENT_ERROR
        );

        Error::invalid_argument_error(symbol, message)
    }

    pub fn make_invalid_argument_error_caused(&mut self, message: &str, cause: Error) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_INVALID_ARGUMENT_ERROR
        );

        Error::invalid_argument_error_caused(symbol, message, cause)
    }


    pub fn make_invalid_argument_count_error(&mut self, message: &str) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR
        );

        Error::invalid_argument_count_error(symbol, message)
    }

    pub fn make_invalid_argument_count_error_caused(&mut self, message: &str, cause: Error) -> Error {
        let symbol = self.intern(
            crate::interpreter::error::SYMBOL_NAME_INVALID_ARGUMENT_COUNT_ERROR
        );

        Error::invalid_argument_count_error_caused(symbol, message, cause)
    }
}

impl Interpreter {
    pub fn deep_equal(&mut self, value1: Value, value2: Value) -> Result<bool, Error> {
        use crate::interpreter::value::Value::*;

        match (value1, value2) {
            (Integer(val1), Integer(val2)) => Ok(val1 == val2),
            (Float(val1), Float(val2)) => Ok(val1 == val2),
            (Boolean(val1), Boolean(val2)) => Ok(val1 == val2),
            (Keyword(val1), Keyword(val2)) => Ok(val1 == val2),
            (Symbol(val1), Symbol(val2)) => Ok(val1 == val2),
            (String(val1), String(val2)) => {
                let string1 = self.get_string(val1)?.clone();
                let string2 = self.get_string(val2)?.clone();

                Ok(string1 == string2)
            },
            (Cons(val1), Cons(val2)) => {
                let car1 = self.get_car(val1)?;
                let car2 = self.get_car(val2)?;

                let cdr1 = self.get_cdr(val1)?;
                let cdr2 = self.get_cdr(val2)?;

                let car_equals = self.deep_equal(car1, car2)?;
                let cdr_equals = self.deep_equal(cdr1, cdr2)?;

                Ok(car_equals && cdr_equals)
            },
            (Object(val1), Object(val2)) => {
                // todo: fix, make it checking objects and not references to them
                Ok(val1 == val2)
            },
            (Function(val1), Function(val2)) => {
                let function_1 = self.get_function(val1)?.clone();
                let function_2 = self.get_function(val2)?.clone();

                Ok(function_1 == function_2)
            },
            _ => Ok(false)
        }
    }

    pub fn print_value(&mut self, value: Value) {
        match value {
            Value::Integer(value) => print!("{}", value),
            Value::Float(value) => print!("{}", value),
            Value::Boolean(value) => print!("{}", if value {"#t"} else {"#f"}),
            Value::Keyword(keyword_id) => {
                print!("{}", self.get_keyword(keyword_id).unwrap().get_name());
            },
            Value::Symbol(symbol_id) => {
                print!("{}", self.get_symbol_name(symbol_id).unwrap());
            },
            Value::String(string_id) => {
                print!("{}", self.get_string(string_id).unwrap().get_string());
            },
            Value::Cons(cons_id) => {
                let mut car = self.get_car(cons_id).unwrap();
                let mut cdr = self.get_cdr(cons_id).unwrap();

                print!("(");

                loop {
                    self.print_value(car);

                    match cdr {
                        Value::Cons(cons_id) => {
                            car = self.get_car(cons_id).unwrap();
                            cdr = self.get_cdr(cons_id).unwrap();
                        },
                        Value::Symbol(symbol_id) => {
                            let symbol = self.get_symbol(symbol_id).unwrap();

                            if !symbol.is_nil() {
                                self.print_value(cdr);
                                print!(" ");
                            } else {
                                print!(")");
                            }

                            break;
                        },
                        value => self.print_value(value)
                    }
                }
            },
            _ => unimplemented!(),
//            Object(ObjectId),
//            Function(FunctionId),
        }

        print!(" ");
    }
}

impl Interpreter {
    pub fn make_string(&mut self, string: String) -> StringId {
        self.string_arena.make_string(string)
    }

    pub fn make_string_value(&mut self, string: String) -> Value {
        Value::String(self.make_string(string))
    }

    pub fn get_string(&mut self, string_id: StringId) -> Result<VString, Error> {
        self.string_arena
            .get_string(string_id)
            .map(|string| string.clone())
            .map_err(|_| self.make_empty_error())
    }

    pub fn intern_string_value(&mut self, string: String) -> Value {
        Value::String(self.string_arena.intern_string(string))
    }
}

impl Interpreter {
    pub fn make_keyword(&mut self, keyword_name: String) -> KeywordId {
        self.keyword_arena.make_keyword(keyword_name)
    }

    pub fn make_keyword_value(&mut self, keyword_name: String) -> Value {
        Value::Keyword(self.make_keyword(keyword_name))
    }

    pub fn get_keyword(&mut self, keyword_id: KeywordId) -> Result<Keyword, Error> {
        self.keyword_arena
            .get_keyword(keyword_id)
            .map(|keyword| keyword.clone())
            .map_err(|_| self.make_empty_error())
    }

    pub fn intern_keyword_value(&mut self, keyword_name: String) -> Value {
        Value::Keyword(self.keyword_arena.intern_keyword(keyword_name))
    }
}

impl Interpreter {
    pub fn get_symbol(&mut self, symbol_id: SymbolId) -> Result<&Symbol, Error> {
        let error = self.make_generic_execution_error(
            ""
        ).into_result();

        self.symbol_arena
            .get_symbol(symbol_id)
            .or(error)
    }
    
    pub fn get_symbol_name(&mut self, symbol_id: SymbolId) -> Result<&String, Error> {
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
}

impl Interpreter {
    pub fn make_cons(&mut self, car: Value, cdr: Value) -> ConsId {
        self.cons_arena.make_cons(car, cdr)
    }

    pub fn make_cons_value(&mut self, car: Value, cdr: Value) -> Value {
        Value::Cons(self.cons_arena.make_cons(car, cdr))
    }

    pub fn get_car(&mut self, cons_id: ConsId) -> Result<Value, Error> {
        self.cons_arena
            .get_car(cons_id)
            .map_err(|_| self.make_empty_error())
    }

    pub fn get_cdr(&mut self, cons_id: ConsId) -> Result<Value, Error> {
        self.cons_arena
            .get_cdr(cons_id)
            .map_err(|_| self.make_empty_error())
    }

    pub fn get_cadr(&mut self, cons_id: ConsId) -> Result<Value, Error> {
        self.cons_arena
            .get_cadr(cons_id)
            .map_err(|_| self.make_empty_error())
    }

    pub fn get_cddr(&mut self, cons_id: ConsId) -> Result<Value, Error> {
        self.cons_arena
            .get_cddr(cons_id)
            .map_err(|_| self.make_empty_error())
    }

    pub fn set_car(&mut self, cons_id: ConsId, value: Value) -> Result<(), Error> {
        self.cons_arena
            .set_car(cons_id, value)
            .map_err(|_| self.make_empty_error())
    }

    pub fn set_cdr(&mut self, cons_id: ConsId, value: Value) -> Result<(), Error> {
        self.cons_arena
            .set_cdr(cons_id, value)
            .map_err(|_| self.make_empty_error())
    }

    pub fn cons_from_vec(&mut self, vector: Vec<Value>) -> Value {
        let nil = self.intern_nil_symbol_value();

        self.cons_arena.cons_from_vec(nil, vector)
    }

    pub fn cons_to_vec(&mut self, cons_id: ConsId) -> Result<Vec<Value>, Error> {
        let mut vector = self.cons_arena
            .cons_to_vec(cons_id)
            .map_err(|_| self.make_empty_error())?;

        // Remove last item of the vector if it's nil. It's necessary, because ConsArena can't say
        // if a SymbolId is the one registered for nil, so it returns all items in the list,
        // including the nil at the cdr of the last cell of the list.
        match vector.last() {
            Some(&val   @ _) => {
                if let Value::Symbol(symbol_id) = val {
                    let symbol = self.get_symbol(symbol_id)?;

                    if symbol.is_nil() {
                        vector.remove(vector.len() - 1);
                    }
                }
            },
            _ => {}
        }

        Ok(vector)
    }
}

impl Interpreter {
    pub fn make_object(&mut self) -> ObjectId {
        self.object_arena.make()
    }

    pub fn make_object_value(&mut self) -> Value {
        Value::Object(self.object_arena.make())
    }

    pub fn make_child_object(&mut self, prototype_id: ObjectId) -> ObjectId {
        self.object_arena.make_child(prototype_id)
    }

    pub fn get_object_item(&mut self, object_id: ObjectId, key: SymbolId) -> Result<Option<Value>, Error> {
        self.object_arena
            .get_item(object_id, key)
            .map_err(|_| self.make_empty_error())
    }

    pub fn set_object_item(&mut self, object_id: ObjectId, key: SymbolId, value: Value) -> Result<(), Error> {
        self.object_arena
            .set_item(object_id, key, value)
            .map_err(|_| self.make_empty_error())
    }

    pub fn get_object_proto(&mut self, object_id: ObjectId) -> Result<Option<ObjectId>, Error> {
        let proto = match self.object_arena.get_object(object_id) {
            Ok(proto) => proto,
            Err(_) => return self.make_empty_error().into_result()
        };

        Ok(proto.get_prototype())
    }

    pub fn set_object_proto(&mut self, object_id: ObjectId, proto_id: ObjectId) -> Result<(), Error>{
        match self.object_arena.get_object_mut(object_id) {
            Ok(object) => object.set_prototype(proto_id),
            Err(_) => return self.make_empty_error().into_result()
        }

        Ok(())
    }
}

impl Interpreter {
    pub fn register_function(&mut self, function: Function) -> FunctionId {
        self.function_arena.register_function(function)
    }

    pub fn get_function(&mut self, function_id: FunctionId) -> Result<&Function, Error> {
        let error = self.make_empty_error()
            .into_result();

        self.function_arena
            .get_function(function_id)
            .or(error)
    }
}

impl Interpreter {
    pub fn get_root_environment(&self) -> EnvironmentId {
        self.root_environment
    }

    pub fn lookup_environment_by_variable(
        &mut self,
        environment_id: EnvironmentId,
        variable_name: SymbolId
    ) -> Result<Option<EnvironmentId>, Error> {
        self.environment_arena.lookup_environment_by_variable(
            environment_id,
            variable_name
        ).map_err(|_| self.make_empty_error())
    }

    pub fn lookup_environment_by_function(
        &mut self,
        environment_id: EnvironmentId,
        function_name: SymbolId
    ) -> Result<Option<EnvironmentId>, Error> {
        self.environment_arena.lookup_environment_by_function(
            environment_id,
            function_name
        ).map_err(|_| self.make_empty_error())
    }

    pub fn has_variable(
        &mut self,
        environment_id: EnvironmentId,
        symbol: SymbolId
    ) -> Result<bool, Error> {
        self.environment_arena.has_variable(
            environment_id,
            symbol
        ).map_err(|_| self.make_empty_error())
    }

    pub fn has_function(
        &mut self,
        environment_id: EnvironmentId,
        symbol: SymbolId
    ) -> Result<bool, Error> {
        self.environment_arena.has_function(
            environment_id,
            symbol
        ).map_err(|_| self.make_empty_error())
    }

    pub fn define_variable(
        &mut self,
        environment_id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        self.environment_arena
            .define_variable(environment_id, symbol_id, value)
            .map_err(|_| self.make_empty_error())
    }

    pub fn define_function(
        &mut self,
        environment_id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        self.environment_arena
            .define_function(environment_id, symbol_id, value)
            .map_err(|_| self.make_empty_error())
    }

    pub fn set_variable(
        &mut self,
        environment_id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        self.environment_arena
            .set_variable(environment_id, symbol_id, value)
            .map_err(|_| self.make_empty_error())
    }

    pub fn set_function(
        &mut self,
        environment_id: EnvironmentId,
        symbol_id: SymbolId,
        value: Value
    ) -> Result<(), Error> {
        self.environment_arena
            .set_function(environment_id, symbol_id, value)
            .map_err(|_| self.make_empty_error())
    }

    pub fn lookup_variable(
        &mut self,
        environment_id: EnvironmentId,
        symbol_id: SymbolId
    ) -> Result<Value, Error> {
        self.environment_arena
            .lookup_variable(environment_id, symbol_id)
            .map_err(|_| self.make_empty_error())?
            .ok_or_else(|| self.make_empty_error())
    }

    pub fn lookup_function(
        &mut self,
        environment_id: EnvironmentId,
        symbol_id: SymbolId
    ) -> Result<Value, Error> {
        self.environment_arena
            .lookup_function(environment_id, symbol_id)
            .map_err(|_| self.make_empty_error())?
            .ok_or_else(|| self.make_empty_error())
    }

    pub fn make_environment(&mut self, parent_environment: EnvironmentId) -> Result<EnvironmentId, Error> {
        self.environment_arena
            .alloc_child(parent_environment)
            .map_err(|_| self.make_empty_error())
    }
}

impl Interpreter {
    fn evaluate_symbol(
        &mut self,
        environment_id: EnvironmentId,
        symbol_id: SymbolId
    ) -> Result<Value, Error> {
        self.lookup_variable(environment_id, symbol_id)
            .map_err(|err| self.make_generic_execution_error_caused("", err))
    }

    fn extract_arguments(&mut self, cons_id: ConsId) -> Result<Vec<Value>, Error> {
        let cons = self.cons_arena
            .get_cdr(cons_id)
            .map_err(|_| self.make_empty_error())?;

        match cons {
            Value::Cons(cons) => self.cons_to_vec(cons),
            Value::Symbol(symbol_id) => {
                let symbol = self.get_symbol(symbol_id)?;

                if symbol.is_nil() {
                    Ok(Vec::new())
                } else {
                    self.make_empty_error()
                        .into_result()
                }
            }
            _ => self.make_empty_error()
                .into_result()
        }
    }

    fn evaluate_arguments(
        &mut self,
        environment_id: EnvironmentId,
        arguments: Vec<Value>
    ) -> Result<Vec<Value>, Error> {
        let mut evaluated_arguments = Vec::new();

        for argument in arguments {
            let evaluated_argument = self
                .evaluate_value(environment_id, argument)
                .map_err(|err| self.make_generic_execution_error_caused("", err))?;

            evaluated_arguments.push(evaluated_argument)
        }

        Ok(evaluated_arguments)
    }

    fn define_environment_variables(
        &mut self,
        execution_environment_id: EnvironmentId,
        variable_names: &Vec<String>,
        variables: &Vec<Value>
    ) -> Result<(), Error> {
        for (i, variable_name) in variable_names.iter().enumerate() {
            let symbol_id = self.intern(variable_name);
            let variable = variables[i];

            self
                .define_variable(execution_environment_id, symbol_id, variable)
                .map_err(|err| self.make_generic_execution_error_caused("", err))?;
        }

        Ok(())
    }

    fn define_environment_functions(
        &mut self,
        execution_environment_id: EnvironmentId,
        function_names: &Vec<String>,
        functions: &Vec<Value>
    ) -> Result<(), Error> {
        for (i, function_name) in function_names.iter().enumerate() {
            let symbol_id = self.intern(function_name);
            let function = functions[i];

            self
                .define_function(execution_environment_id, symbol_id, function)
                .map_err(|err| self.make_generic_execution_error_caused("", err))?;
        }

        Ok(())
    }

    fn execute_code(&mut self, execution_environment:EnvironmentId, code: &Vec<Value>) -> Result<Option<Value>, Error> {
        let mut last_result = None;

        for value in code {
            last_result = self.evaluate_value(execution_environment, *value)
                .map(|v| Some(v))
                .map_err(|err| self.make_generic_execution_error_caused("", err))?;
        }

        Ok(last_result)
    }

    fn evaluate_interpreted_function_invocation(
        &mut self,
        func: &InterpretedFunction,
        evaluated_arguments: Vec<Value>
    ) -> Result<Value, Error> {
        if func.get_argument_names().len() != evaluated_arguments.len() {
            return self.make_empty_error()
                .into_result();
        }

        // 1) make new environment
        let execution_environment_id = self.make_environment(func.get_environment())
            .map_err(|err| self.make_generic_execution_error_caused("", err))?;

        // 2) setup environment variables and functions
        self.define_environment_variables(
            execution_environment_id,
            func.get_argument_names(),
            &evaluated_arguments
        ).map_err(|err| self.make_generic_execution_error_caused("", err))?;

        self.define_environment_functions(
            execution_environment_id,
            func.get_argument_names(),
            &evaluated_arguments
        ).map_err(|err| self.make_generic_execution_error_caused("", err))?;

        // 3) execute code
        let execution_result = self.execute_code(
            execution_environment_id,
            func.get_code()
        ).map_err(|err| self.make_generic_execution_error_caused("", err))?;

        // 4) return result
        execution_result.or_else(|| Some(self.intern_nil_symbol_value()))
            .ok_or_else(|| self.make_empty_error())
    }

    fn evaluate_builtin_function_invocation(
        &mut self,
        builtin_function: &BuiltinFunction,
        environment_id: EnvironmentId,
        evaluated_arguments: Vec<Value>
    ) -> Result<Value, Error> {
        (builtin_function.get_func())(self, environment_id, evaluated_arguments)
    }

    fn evaluate_special_form_invocation(
        &mut self,
        execution_environment: EnvironmentId,
        special_form: &SpecialFormFunction,
        arguments: Vec<Value>
    ) -> Result<Value, Error> {
        (special_form.get_func())(self, execution_environment, arguments)
    }

    fn evaluate_macro_invocation(
        &mut self,
        func: &MacroFunction,
        arguments: Vec<Value>
    ) -> Result<Value, Error> {
        if func.get_argument_names().len() != arguments.len() {
            return self.make_empty_error().into_result();
        }

        // 1) make new environment
        let execution_environment_id = self.make_environment(func.get_environment())
            .map_err(|err| self.make_generic_execution_error_caused("", err))?;

        // 2) set arguments in that environment
        self.define_environment_variables(
            execution_environment_id,
            func.get_argument_names(),
            &arguments
        ).map_err(|err| self.make_generic_execution_error_caused("", err))?;

        self.define_environment_functions(
            execution_environment_id,
            func.get_argument_names(),
            &arguments
        ).map_err(|err| self.make_generic_execution_error_caused("", err))?;

        // 3) execute code
        let execution_result = self.execute_code(execution_environment_id, func.get_code())
            .map_err(|err| self.make_generic_execution_error_caused("", err))?;

        // 4) return result
        execution_result.or_else(|| Some(self.intern_nil_symbol_value()))
            .ok_or_else(|| self.make_empty_error())
    }

    fn evaluate_s_expression_function_invocation(
        &mut self,
        environment_id: EnvironmentId,
        function: FunctionId,
        cons_id: ConsId
    ) -> Result<Value, Error> {
        let function = self.get_function(function)
            .map(|function| function.clone())
            .map_err(|err| self.make_generic_execution_error_caused("", err))?;

        match function {
            Function::Builtin(builtin_function) => {
                // 2) evaluate arguments
                let arguments = self.extract_arguments(cons_id)?;
                let evaluated_arguments = self.evaluate_arguments(environment_id, arguments)?;

                // 3) apply function from step 1 to arguments from step 2
                self.evaluate_builtin_function_invocation(
                    &builtin_function,
                    environment_id,
                    evaluated_arguments
                )
            },
            Function::Interpreted(interpreted_function) => {
                // 2) evaluate arguments
                let arguments = self.extract_arguments(cons_id)?;
                let evaluated_arguments = self.evaluate_arguments(environment_id, arguments)?;

                // 3) apply function from step 1 to arguments from step 2
                self.evaluate_interpreted_function_invocation(
                    &interpreted_function,
                    evaluated_arguments
                )
            },
            Function::SpecialForm(special_form) => {
                let arguments = self.extract_arguments(cons_id)?;

                self.evaluate_special_form_invocation(
                    environment_id,
                    &special_form,
                    arguments
                )
            },
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
        cons_id: ConsId
    ) -> Result<Value, Error> {
        let keyword_name = self.get_keyword(keyword_id)
            .map(|keyword| keyword.get_name().clone())
            .map_err(|err| self.make_generic_execution_error_caused("", err ))?;

        let symbol_id = self.intern(&keyword_name);

        let mut arguments = self.extract_arguments(cons_id)
            .map_err(|err| self.make_generic_execution_error_caused("", err))?;

        if arguments.len() != 1 {
            return self.make_empty_error().into_result();
        }

        let argument = arguments.remove(0);

        let evaluated_argument = self.evaluate_value(
            environment_id,
            argument
        ).map_err(|err| self.make_generic_execution_error_caused("", err))?;

        match evaluated_argument {
            Value::Object(object_id) => {
                self.object_arena.get_item(object_id, symbol_id)
                    .map_err(|_| self.make_empty_error())?
                    .ok_or_else(|| self.make_empty_error())
            },
            _ => return self.make_empty_error().into_result()
        }
    }

    fn evaluate_s_expression(
        &mut self,
        environment_id: EnvironmentId,
        s_expression: ConsId
    ) -> Result<Value, Error> {
        // 1) evaluate first symbol
        let car = self.cons_arena
            .get_car(s_expression)
            .map_err(|_| self.make_empty_error())?;

        match car {
            Value::Symbol(func_name) => {
                let function_value = self.lookup_function(environment_id, func_name)
                    .map_err(|err| self.make_generic_execution_error_caused("", err))?;

                let function_id = match function_value {
                    Value::Function(function_id) => function_id,
                    _ => return self.make_empty_error().into_result(),
                };

                self.evaluate_s_expression_function_invocation(
                    environment_id,
                    function_id,
                    s_expression
                )
            },
            Value::Function(function_id) => self.evaluate_s_expression_function_invocation(
                environment_id,
                function_id,
                s_expression
            ),
            Value::Cons(cons_id) => {
                let function_value = self.evaluate_s_expression(environment_id, cons_id)
                    .map_err(|err| self.make_generic_execution_error_caused("", err))?;

                let function_id = match function_value {
                    Value::Function(function_id) => function_id,
                    _ => return self.make_empty_error().into_result(),
                };

                self.evaluate_s_expression_function_invocation(
                    environment_id,
                    function_id,
                    s_expression
                )
            }
            Value::Keyword(keyword_id) => self.evaluate_s_expression_keyword(
                environment_id,
                keyword_id,
                s_expression
            ),
            _ => self.make_empty_error().into_result()
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

    pub fn execute(&mut self, code: &str) -> Result<Value, Error> {
        // first step: parse code
        let code = parse_code(code)
            .map(|result| result.1)
            .map_err(|_| self.make_empty_error())?;

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
    use crate::interpreter::lib::testing_helpers::make_value_pairs_ifbsyk;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::lib::assertion::assert_deep_equal;

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

        let expected = interpreter.intern_string_value(String::from("tas"));
        let result = interpreter.execute(r#""tas""#).unwrap();

        assertion::assert_deep_equal(
            &mut interpreter,
            expected,
            result
        );
    }

    #[test]
    pub fn executes_symbol_correctly() {
        let mut interpreter = Interpreter::new();
        let name = interpreter.intern("test");

        interpreter.environment_arena.define_variable(
            interpreter.root_environment,
            name,
            Value::Integer(1)
        ).unwrap();

        let result = interpreter.execute("test");

        assert_eq!(Value::Integer(1), result.unwrap());
    }

    #[test]
    pub fn executes_keyword_correctly() {
        assert_execution_result_eq!(Value::Keyword(KeywordId::new(0)), r#":tas"#);
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

        let pairs = make_value_pairs_ifbsyk(&mut interpreter);

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
                result
            );
        }
    }

    #[test]
    fn executes_delimited_symbols_expression_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = make_value_pairs_ifbsyk(&mut interpreter);

        for pair in pairs {
            let code = String::from("(let ((obj {:value ") + &pair.0 + "})) obj:value)";
            let expected = pair.1;
            let result = interpreter.execute(&code).unwrap();

            assertion::assert_deep_equal(
                &mut interpreter,
                expected,
                result
            );
        }
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
            nil
        ));

        let value = Value::Cons(interpreter.make_cons(
            a,
            value
        ));

        let value = Value::Cons(interpreter.make_cons(
            plus,
            value
        ));

        let code = vec!(
            value
        );

        let name = interpreter.intern("test");

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.root_environment,
            vec!("a".to_string(), "b".to_string()),
            code
        ));

        let function_id = interpreter.register_function(function);

        interpreter.environment_arena.define_function(
            interpreter.root_environment,
            name,
            Value::Function(function_id)
        ).unwrap();

        let result = interpreter.execute("(test 3 2)");
        assert_eq!(Value::Integer(5), result.unwrap());
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
                    _ => interpreter.make_empty_error().into_result()
                }
            }
        ));

        let function_id = interpreter.register_function(function);
        let function_value = Value::Function(function_id);

        interpreter.environment_arena.define_function(
            interpreter.root_environment,
            name,
            function_value
        ).unwrap();

        let result = interpreter.execute("(testif #t 1 2)");
        assert_eq!(Value::Integer(1), result.unwrap());

        let result = interpreter.execute("(testif #f 1 2)");
        assert_eq!(Value::Integer(2), result.unwrap());

        let result = interpreter.execute("(testif (testif #t #t #f) 1 2)");
        assert_eq!(Value::Integer(1), result.unwrap());

        let result = interpreter.execute("(testif (testif #f #t #f) 1 2)");
        assert_eq!(Value::Integer(2), result.unwrap());
    }

//    #[test]
//    pub fn macro_invocation_evaluates_correctly() {
//        use crate::interpreter::stdlib::infect_interpreter;
//
//        let mut interpreter = Interpreter::new();
//
//        infect_interpreter(&mut interpreter);
//
//        let macro_name = interpreter.intern_symbol("if");
//        let argument_names = vec!(
//            String::from("condition"),
//            String::from("then-clause"),
//            String::from("else-clause"),
//        );
//
//        interpreter.define_function(
//            interpreter.get_root_environment(),
//            macro_name,
//            Value::Function(Function::Macro(MacroFunction::new(
//                interpreter.root_environment,
//                argument_names,
//
//            )))
//        )
//    }
}
