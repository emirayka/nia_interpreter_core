use crate::interpreter::environment_arena::Arena;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::parser::parse_code;
use crate::interpreter::pre_reader::preread_elements;
use crate::interpreter::cons::Cons;
use crate::interpreter::function::{InterpretedFunction, BuiltInFunction};

pub struct Interpreter {
    arena: Arena,
    root_env_id: EnvironmentId,
    current_env_id: EnvironmentId,
    call_stack: (),
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut arena = Arena::new();
        let root_env_id = arena.alloc();

        Interpreter {
            arena,
            root_env_id,
            current_env_id: root_env_id,
            call_stack: (),
        }
    }
}

impl Interpreter {
    fn execute_symbol(&mut self, environment: EnvironmentId, symbol_name: &str) -> Result<Value, ()> {
        let result = self.arena.lookup_variable(environment, symbol_name);

        match result {
            Some(value) => Ok(value.clone()),
            None => Err(())
        }
    }

    pub fn execute_value(&mut self, environment: EnvironmentId, value: &Value) -> Result<Value, ()> {
        match value {
            Value::Symbol(symbol_name) => self.execute_symbol(environment, symbol_name),
//            Value::Cons(cons) => self.execute_sexp(environment, cons),
            _ => Ok(value.clone())
        }
    }

    pub fn execute(&mut self, code: &str) -> Result<Value, ()> {
        // first step: parse code
        let parsed = parse_code(code);

        if parsed.is_err() {
            return Err(());
        }

        // second step: read forms
        let values = if let Ok((_, code)) = parsed {
            preread_elements(code.get_elements())
        } else {
            return Err(());
        };

        // third step: evaluate
        let mut results: Vec<Value> = Vec::new();

        for value in values {
            match self.execute_value(self.root_env_id, &value) {
                Ok(result) => results.push(result),
                Err(_) => return Err(())
            }
        }

        Ok(results.last().unwrap().clone())
    }

}

impl Interpreter {
    pub fn define_variable(&mut self, environment: EnvironmentId, name: &str, value: Value) {
        self.arena.define_variable(environment, name, value);
    }

    pub fn define_function(&mut self, environment: EnvironmentId, name: &str, value: Value) {
        self.arena.define_function(environment, name, value);
    }

    pub fn set_variable(&mut self, environment: EnvironmentId, name: &str, value: Value) {
        self.arena.set_variable(environment, name, value);
    }

    pub fn set_function(&mut self, environment: EnvironmentId, name: &str, value: Value) {
        self.arena.set_function(environment, name, value);
    }

    pub fn lookup_variable(&self, environment: EnvironmentId, name: &str) -> Option<&Value> {
        self.arena.lookup_variable(environment, name)
    }

    pub fn lookup_function(&self, environment: EnvironmentId, name: &str) -> Option<&Value> {
        self.arena.lookup_function(environment, name)
    }

    pub fn make_environment(&mut self, parent_environment: EnvironmentId) -> EnvironmentId {
        self.arena.alloc_child(parent_environment)
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
        let mut interpreter = Interpreter::new();

        interpreter.arena.define_variable(interpreter.root_env_id, "test", Value::Integer(1));

        let result = interpreter.execute("test");

        assert_eq!(Value::Integer(1), result.unwrap());
    }

    #[test]
    pub fn test_executes_keyword_correctly() {
        assert_execution_result_eq!(Value::Keyword(String::from("tas")), r#":tas"#);
    }

    #[test]
    pub fn test_executes_sexp_correctly() {
//        let mut interpreter = Interpreter::new();
//
//        interpreter.arena.define_function(
//            interpreter.root_env_id,
//            "test",
//            Value::BuiltinFunction(BuiltInFunction::new(
//                |values: Vec<Value>| -> Result<Value, ()> {
//                    let first = &values[0];
//                    let second= &values[1];
//
//                    let value = match (first, second) {
//                        (Value::Integer(int1), Value::Integer(int2)) => Value::Integer(int1 + int2),
//                        _ => Value::Symbol(String::from("nil"))
//                    };
//
//                    Ok(value)
//                }
//            ))
//        );
//
//        let result = interpreter.execute("(test 1 2)");
//        assert_eq!(Value::Integer(3), result.unwrap());
//
//        let result = interpreter.execute("(test 1 2.2)");
//        assert_eq!(Value::Symbol(String::from("nil")), result.unwrap());
    }
}
