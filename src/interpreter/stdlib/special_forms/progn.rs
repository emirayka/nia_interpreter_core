use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::stdlib::special_forms::_lib::infect_special_form;

fn progn(
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

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect_special_form(interpreter, "progn", progn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::cons::Cons;
    use crate::interpreter::error::assertion;

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Integer(3), interpreter.execute("(progn 3)").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(progn 3 2)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(progn 3 2 1)").unwrap());
    }

    #[test]
    fn returns_nil_if_no_form_were_provided() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(interpreter.intern_nil(), interpreter.execute("(progn)").unwrap());
    }
}
