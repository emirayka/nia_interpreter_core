use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn set_car_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `cons:set-car!' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let cons_id = library::read_as_cons_id(values.remove(0))?;

    let value = values.remove(0);

    interpreter.set_car(cons_id, value)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn sets_car() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(let ((c (cons:new 1 2))) (cons:car c))", "1"),
            (
                "(let ((c (cons:new 1 2))) (cons:set-car! c 3) (cons:car c))",
                "3",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(cons:set-car!)",
            "(cons:set-car! (cons:new 1 2))",
            "(cons:set-car! (cons:new 1 2) 3 4)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_called_with_a_value_that_is_not_cons(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(cons:set-car! 1 1)",
            "(cons:set-car! 1.1 1)",
            "(cons:set-car! #t 1)",
            "(cons:set-car! #f 1)",
            "(cons:set-car! \"string\" 1)",
            "(cons:set-car! 'symbol 1)",
            "(cons:set-car! :keyword 1)",
            "(cons:set-car! {} 1)",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
