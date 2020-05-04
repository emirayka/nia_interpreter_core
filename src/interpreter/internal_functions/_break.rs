use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn _break(
    _interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    _values: Vec<Value>,
) -> Result<Value, Error> {
    Error::break_error().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    use crate::interpreter::error::ErrorKind;

    #[test]
    fn returns_break_error() {
        let mut interpreter = Interpreter::new();

        let err = _break(&mut interpreter, EnvironmentId::new(0), vec![])
            .err()
            .unwrap();

        nia_assert_equal(ErrorKind::Break, err.get_error_kind())
    }
}
