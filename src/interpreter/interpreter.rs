use crate::interpreter::environment_arena::EnvironmentArena;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::parser::parse_code;
use crate::interpreter::pre_reader::preread_elements;
use crate::interpreter::cons::Cons;
use crate::interpreter::function::{Function};
use crate::interpreter::function::interpreted_function::InterpretedFunction;
use crate::interpreter::function::builtin_function::BuiltinFunction;
use crate::interpreter::function::special_form_function::SpecialFormFunction;
use crate::interpreter::symbol::{Symbol, SymbolArena};
use crate::interpreter::function::macro_function::MacroFunction;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::infect_interpreter;

pub struct Interpreter {
    environment_arena: EnvironmentArena,
    symbol_arena: SymbolArena,
    root_environment: EnvironmentId,
    call_stack: (),
}

impl Interpreter {
    pub fn raw() -> Interpreter {
        let mut environment_arena = EnvironmentArena::new();
        let root_env_id = environment_arena.alloc();

        let symbol_arena = SymbolArena::new();

        Interpreter {
            environment_arena,
            symbol_arena,
            root_environment: root_env_id,
            call_stack: (),
        }
    }

    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter::raw();

        match infect_interpreter(&mut interpreter) {
            Ok(()) => interpreter,
            Err(_error) => panic!("Cannot construct interpreter")
        }
    }
}

impl Interpreter {
    pub fn define_variable(&mut self, environment: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        self.environment_arena.define_variable(environment, symbol, value)
    }

    pub fn define_function(&mut self, environment: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        self.environment_arena.define_function(environment, symbol, value)
    }

    pub fn set_variable(&mut self, environment: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        self.environment_arena.set_variable(environment, symbol, value)
    }

    pub fn set_function(&mut self, environment: EnvironmentId, symbol: &Symbol, value: Value) -> Result<(), Error> {
        self.environment_arena.set_function(environment, symbol, value)
    }

    pub fn lookup_variable(&self, environment: EnvironmentId, symbol: &Symbol) -> Result<&Value, Error> {
        match self.environment_arena.lookup_variable(environment, symbol) {
            Some(value) => Ok(value),
            None => Err(Error::empty())
        }
    }

    pub fn lookup_function(
        &self,
        environment: EnvironmentId,
        symbol: &Symbol
    ) -> Result<&Value, Error> {
        match self.environment_arena.lookup_function(environment, symbol) {
            Some(value) => Ok(value),
            None => Err(Error::empty())
        }
    }

    pub fn make_environment(&mut self, parent_environment: EnvironmentId) -> EnvironmentId {
        self.environment_arena.alloc_child(parent_environment)
    }

    pub fn intern_nil(&mut self) -> Value {
        Value::Symbol(self.symbol_arena.intern("nil"))
    }

    pub fn intern(&mut self, symbol_name: &str) -> Value {
        Value::Symbol(self.symbol_arena.intern(symbol_name))
    }

    pub fn gensym(&mut self, symbol_name: &str) -> Symbol {
        self.symbol_arena.gensym(symbol_name)
    }

    pub fn intern_symbol(&mut self, symbol_name: &str) -> Symbol {
        self.symbol_arena.intern(symbol_name)
    }

    pub fn intern_symbol_nil(&mut self, symbol_name: &str) -> Symbol {
        self.symbol_arena.intern(symbol_name)
    }

    pub fn get_root_environment(&self) -> EnvironmentId {
        self.root_environment
    }

    pub fn lookup_environment_by_variable(
        &self,
        environment: EnvironmentId,
        variable_name: &Symbol
    ) -> Option<EnvironmentId> {
        self.environment_arena.lookup_environment_by_variable(environment, variable_name)
    }
}

impl Interpreter {
    fn evaluate_symbol(&mut self, environment: EnvironmentId, symbol: &Symbol) -> Result<Value, Error> {
        let result = self.lookup_variable(environment, symbol);

        match result {
            Ok(value) => Ok(value.clone()),
            Err(error) => Err(error)
        }
    }

    fn extract_arguments(&mut self, cons: &Cons) -> Result<Vec<Value>, Error> {
        let mut extracted_arguments = Vec::new();
        let mut current_cdr = match cons.get_cdr() {
            Value::Cons(cons) => cons,
            Value::Symbol(symbol) => {
                if symbol.is_nil() {
                    return Ok(extracted_arguments);
                } else {
                    return Err(Error::empty());
                }
            }
            _ => return Err(Error::empty())
        };

        loop {
            extracted_arguments.push(current_cdr.get_car().clone());

            current_cdr = match current_cdr.get_cdr() {
                Value::Cons(cons) => cons,
                Value::Symbol(symbol) => {
                    if symbol.is_nil() {
                        break;
                    } else {
                        return Err(Error::empty());
                    }
                }
                _ => return Err(Error::empty())
            };
        }

        Ok(extracted_arguments)
    }

    fn evaluate_arguments(&mut self, environment: EnvironmentId, arguments: Vec<Value>)  -> Result<Vec<Value>, Error> {
        let mut evaluated_arguments = Vec::new();

        for argument in arguments {
            match self.evaluate_value(environment, &argument) {
                Ok(evaluated_argument) => evaluated_arguments.push(evaluated_argument),
                Err(_) => return Err(Error::empty())
            }
        }

        Ok(evaluated_arguments)
    }

    fn define_environment_variables(
        &mut self,
        execution_environment: EnvironmentId,
        names: &Vec<String>,
        variables: Vec<Value>
    ) -> Result<(), Error> {
        let len = names.len();

        for i in 0..len {
            let name = &names[i];
            let name = self.intern_symbol(name);
            let variable = &variables[i];

            match self.define_variable(execution_environment, &name, variable.clone()) {
                Ok(()) => (),
                Err(error) => return Err(error)
            };
        }

        Ok(())
    }

    fn execute_code(&mut self, execution_environment:EnvironmentId, code: &Vec<Value>) -> Result<Option<Value>, Error> {
        let mut last_result = None;

        for value in code {
            last_result = match self.evaluate_value(execution_environment, value) {
                Ok(value) => Some(value),
                _ => return Err(Error::empty())
            };
        }

        Ok(last_result)
    }

    fn evaluate_interpreted_function_invocation(
        &mut self,
        func: &InterpretedFunction,
        evaluated_arguments: Vec<Value>
    ) -> Result<Value, Error> {
        if func.get_argument_names().len() != evaluated_arguments.len() {
            return Err(Error::empty());
        }

        // 1) make new environment
        let execution_environment = self.make_environment(func.get_environment());

        // 2) set arguments in that environment
        match self.define_environment_variables(
            execution_environment,
            func.get_argument_names(),
            evaluated_arguments
        ) {
            Ok(()) => (),
            Err(error) => return Err(error)
        };

        // 3) execute code
        let execution_result = match self.execute_code(execution_environment, func.get_code()) {
            Ok(value) => value,
            Err(error) => return Err(error)
        };

        // 4) return result
        let value_to_return = match execution_result {
            Some(value) => value,
            None => self.intern_nil()
        };

        Ok(value_to_return)
    }

    fn evaluate_builtin_function_invocation(
        &mut self,
        builtin_function: &BuiltinFunction,
        evaluated_arguments: Vec<Value>
    ) -> Result<Value, Error> {
        (builtin_function.get_func())(self, evaluated_arguments)
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
            return Err(Error::empty());
        }

        // 1) make new environment
        let execution_environment = self.make_environment(func.get_environment());

        // 2) set arguments in that environment
        match self.define_environment_variables(
            execution_environment,
            func.get_argument_names(),
            arguments
        ) {
            Ok(()) => (),
            Err(error) => return Err(error)
        };

        // 3) execute code
        let execution_result = match self.execute_code(execution_environment, func.get_code()) {
            Ok(value) => value,
            Err(error) => return Err(error)
        };

        // 4) return result
        let value_to_return = match execution_result {
            Some(value) => value,
            None => self.intern_nil()
        };

        Ok(value_to_return)
    }

    fn evaluate_s_expression(&mut self, environment: EnvironmentId, cons: &Cons) -> Result<Value, Error> {
        // 1) evaluate first symbol
        let car_value = cons.get_car();

        let function = match car_value {
            Value::Symbol(func_name) => match self.lookup_function(environment, func_name) {
                Ok(Value::Function(func)) => func.clone(),
                Ok(_) => return Err(Error::empty()),
                Err(error) => return Err(error)
            },
            Value::Function(func) => func.clone(),
            Value::Cons(cons) => match self.evaluate_s_expression(environment, cons) {
                Ok(Value::Function(func)) => func,
                Ok(_) => return Err(Error::empty()),
                Err(error) => return Err(error)
            },
            _ => return Err(Error::empty())
        };

        // todo: add caused errors
        match &function {
            Function::Builtin(builtin_function) => {
                // 2) evaluate arguments
                let arguments = match self.extract_arguments(cons) {
                    Ok(arguments) => arguments,
                    Err(error) => return Err(error)
                };

                let evaluated_arguments = match self.evaluate_arguments(environment, arguments) {
                    Ok(arguments) => arguments,
                    Err(error) => return Err(error)
                };

                // 3) apply function from step 1 to arguments from step 2
                self.evaluate_builtin_function_invocation(builtin_function, evaluated_arguments)
            },
            Function::Interpreted(interpreted_function) => {
                // 2) evaluate arguments
                let arguments = match self.extract_arguments(cons) {
                    Ok(arguments) => arguments,
                    Err(error) => return Err(error)
                };

                let evaluated_arguments = match self.evaluate_arguments(environment, arguments) {
                    Ok(arguments) => arguments,
                    Err(error) => return Err(error)
                };

                // 3) apply function from step 1 to arguments from step 2
                self.evaluate_interpreted_function_invocation(interpreted_function, evaluated_arguments)
            },
            Function::SpecialForm(special_form) => {
                let arguments = match self.extract_arguments(cons) {
                    Ok(arguments) => arguments,
                    Err(error) => return Err(error)
                };

                self.evaluate_special_form_invocation(
                    environment,
                    special_form,
                    arguments
                )
            },
            Function::Macro(macro_function) => {
                let arguments = match self.extract_arguments(cons) {
                    Ok(arguments) => arguments,
                    Err(error) => return Err(error)
                };

                match self.evaluate_macro_invocation(macro_function, arguments) {
                    Ok(value) => self.evaluate_value(environment, &value),
                    Err(error) => return Err(error)
                }
            }
        }
    }

    pub fn evaluate_value(&mut self, environment: EnvironmentId, value: &Value) -> Result<Value, Error> {
        match value {
            Value::Symbol(symbol_name) => self.evaluate_symbol(environment, symbol_name),
            Value::Cons(cons) => self.evaluate_s_expression(environment, cons),
            _ => Ok(value.clone())
        }
    }
}

impl Interpreter {
    pub fn execute_value(&mut self, environment: EnvironmentId, value: &Value) -> Result<Value, Error> {
        self.evaluate_value(environment, value)
    }

    pub fn execute(&mut self, code: &str) -> Result<Value, Error> {
        // first step: parse code
        let parsed = parse_code(code);

        if parsed.is_err() {
            return Err(Error::empty());
        }

        // second step: read forms
        let values = if let Ok((_, code)) = parsed {
            preread_elements(self, code.get_elements())
        } else {
            return Err(Error::empty());
        };

        // third step: evaluate
        let mut results: Vec<Value> = Vec::new();

        for value in values {
            match self.execute_value(self.root_environment, &value) {
                Ok(result) => results.push(result),
                Err(error) => return Err(error)
            }
        }

        Ok(results.last().unwrap().clone())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_execution_result_eq {
        ($expected:expr, $code:expr) => {
            let mut interpreter = Interpreter::new();
            let result = interpreter.execute($code);

            assert_eq!($expected, result.unwrap())
        }
    }

    macro_rules! define_sum_function {
        ($interpreter_id:ident) => {
            let name = $interpreter_id.intern_symbol("+");
            $interpreter_id.environment_arena.define_function(
                $interpreter_id.root_environment,
                &name,
                Value::Function(Function::Builtin(BuiltinFunction::new(
                    |interpreter: &mut Interpreter, values: Vec<Value>| -> Result<Value, Error> {
                        let first = &values[0];
                        let second= &values[1];

                        let value = match (first, second) {
                            (Value::Integer(int1), Value::Integer(int2)) => Value::Integer(int1 + int2),
                            (Value::Float(float1), Value::Float(float2)) => Value::Float(float1 + float2),
                            _ => interpreter.intern_nil()
                        };

                        Ok(value)
                    }
                )))
            ).unwrap();
        }
    }

    #[test]
    pub fn test_executes_integer_correctly() {
        assert_execution_result_eq!(Value::Integer(1), "1");
    }

    #[test]
    pub fn test_executes_float_correctly() {
        assert_execution_result_eq!(Value::Float(1.2), "1.2");
    }

    #[test]
    pub fn test_executes_boolean_correctly() {
        assert_execution_result_eq!(Value::Boolean(true), "#t");
        assert_execution_result_eq!(Value::Boolean(false), "#f");
    }

    #[test]
    pub fn test_executes_string_correctly() {
        assert_execution_result_eq!(Value::String(String::from("tas")), r#""tas""#);
    }

    #[test]
    pub fn test_executes_symbol_correctly() {
        let mut interpreter = Interpreter::raw();
        let name = interpreter.intern_symbol("test");

        interpreter.environment_arena.define_variable(
            interpreter.root_environment,
            &name,
            Value::Integer(1)
        ).unwrap();

        let result = interpreter.execute("test");

        assert_eq!(Value::Integer(1), result.unwrap());
    }

    #[test]
    pub fn test_executes_keyword_correctly() {
        assert_execution_result_eq!(Value::Keyword(String::from("tas")), r#":tas"#);
    }

    #[test]
    pub fn test_builtin_function_works_correctly() {
        let mut interpreter = Interpreter::raw();

        define_sum_function!(interpreter);

        let result = interpreter.execute("(+ 1 2)");
        assert_eq!(Value::Integer(3), result.unwrap());

        let result = interpreter.execute("(+ 1 2.2)");
        assert_eq!(interpreter.intern_nil(), result.unwrap());

        let result = interpreter.execute("(+ 1.1 2.4)");
        assert_eq!(Value::Float(3.5), result.unwrap());

        let result = interpreter.execute("(+ (+ (+ 1 2) 3) 4)");
        assert_eq!(Value::Integer(10), result.unwrap());
    }

    #[test]
    pub fn test_interpreted_function_works_correctly() {
        let mut interpreter = Interpreter::raw();

        define_sum_function!(interpreter);
        let code = vec!(
            Value::Cons(Cons::new(
                interpreter.intern("+"),
                Value::Cons(Cons::new(
                    interpreter.intern("a"),
                    Value::Cons(Cons::new(
                        interpreter.intern("b"),
                        interpreter.intern("nil")
                    ))
                )),
            ))
        );

        let name = interpreter.intern_symbol("test");

        interpreter.environment_arena.define_function(
            interpreter.root_environment,
            &name,
            Value::Function(Function::Interpreted(InterpretedFunction::new(
                interpreter.root_environment,
                vec!("a".to_string(), "b".to_string()),
                code
            )))
        ).unwrap();

        let result = interpreter.execute("(test 3 2)");
        assert_eq!(Value::Integer(5), result.unwrap());
    }

    #[test]
    pub fn test_special_form_invocation_evaluates_correctly() {
        let mut interpreter = Interpreter::raw();
        let name = interpreter.intern_symbol("testif");

        interpreter.environment_arena.define_function(
            interpreter.root_environment,
            &name,
            Value::Function(Function::SpecialForm(SpecialFormFunction::new(
                |interpreter: &mut Interpreter, environment: EnvironmentId, values: Vec<Value>| -> Result<Value, Error> {
                    let condition = &values[0];
                    let then_clause = &values[1];
                    let else_clause = &values[2];

                    let evaluated_condition = interpreter.evaluate_value(environment, condition);

                    match evaluated_condition {
                        Ok(Value::Boolean(true)) => interpreter.evaluate_value(environment, then_clause),
                        Ok(Value::Boolean(false)) => interpreter.evaluate_value(environment, else_clause),
                        _ => Err(Error::empty())
                    }
                }
            )))
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

    #[test]
    pub fn test_macro_invocation_evaluates_correctly() {
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
    }
}
