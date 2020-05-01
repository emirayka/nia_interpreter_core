use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn _continue(
    _interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    _values: Vec<Value>
) -> Result<Value, Error> {
    Error::continue_error().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::ErrorKind;

    #[test]
    fn returns_continue_error() {
        let mut interpreter = Interpreter::new();

        let err = _continue(
            &mut interpreter,
            EnvironmentId::new(0),
            vec!()
        ).err().unwrap();

        assert_eq!(ErrorKind::Continue, err.get_error_kind())
    }
}
