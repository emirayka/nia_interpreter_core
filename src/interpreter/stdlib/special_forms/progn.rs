use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn progn(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    library::evaluate_forms_return_last(interpreter, environment, &values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("nil", "(progn)"),
            ("3", "(progn 3)"),
            ("2", "(progn 3 2)"),
            ("1", "(progn 3 2 1)"),
        ];

        utils::assert_results_are_equal(&mut interpreter, specs);
    }
}
