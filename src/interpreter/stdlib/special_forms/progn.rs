use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn progn(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    super::_lib::execute_forms(
        interpreter,
        environment,
        values
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(3), interpreter.execute("(progn 3)").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(progn 3 2)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(progn 3 2 1)").unwrap());
    }

    #[test]
    fn returns_nil_if_no_form_were_provided() {
        let mut interpreter = Interpreter::new();

        assert_eq!(interpreter.intern_nil(), interpreter.execute("(progn)").unwrap());
    }
}
