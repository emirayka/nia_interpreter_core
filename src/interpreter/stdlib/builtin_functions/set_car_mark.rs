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
            "Built-in function `set-car!' must take exactly one argument.",
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
    use crate::utils::assertion;

    #[test]
    fn sets_car() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(let ((c (cons 1 2))) (car c))", "1"),
            ("(let ((c (cons 1 2))) (set-car! c 3) (car c))", "3"),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(set-car!)",
            "(set-car! (cons 1 2))",
            "(set-car! (cons 1 2) 3 4)",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_called_with_a_value_that_is_not_cons(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(set-car! 1 1)",
            "(set-car! 1.1 1)",
            "(set-car! #t 1)",
            "(set-car! #f 1)",
            "(set-car! \"string\" 1)",
            "(set-car! 'symbol 1)",
            "(set-car! :keyword 1)",
            "(set-car! {} 1)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
