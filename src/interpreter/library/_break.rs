use crate::interpreter::symbol::SymbolId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn _break(
    _interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    _values: Vec<Value>
) -> Result<Value, Error> {
    Error::break_error().into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::error::ErrorKind;

    #[test]
    fn returns_break_error() {
        let mut interpreter = Interpreter::new();

        let err = _break(
            &mut interpreter,
            EnvironmentId::new(0),
            vec!()
        ).err().unwrap();

        assert_eq!(ErrorKind::Break, err.get_error_kind())
    }
}
