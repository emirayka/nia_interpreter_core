use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;
use rand;
use rand::Rng;

pub fn float(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() > 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `rand:float' takes no more than two arguments."
        ).into_result()
    }

    let mut values = values;

    let min = if values.len() > 0 {
        library::read_as_f64(
            interpreter,
            values.remove(0)
        )?
    } else {
        std::f64::MIN
    };

    let max = if values.len() > 0 {
        library::read_as_f64(
            interpreter,
            values.remove(0)
        )?
    } else {
        std::f64::MAX
    };

    if min > max {
        return interpreter.make_invalid_argument_error(
            "Built-in function `rand:float' expects min <= max."
        ).into_result()
    }

    let mut rng = rand::thread_rng();
    let result = rng.gen_range(min, max);

    Ok(Value::Float(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::for_special_symbols;

    #[test]
    fn returns_random_float() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:float? (rand:float))", "#t"),
            ("(>= (rand:float 1.0) 1)", "#t"),
            ("(let ((a (rand:float 1.0 100.0))) (and (>= a 1) (< a 100)))", "#t"),
        );

        for _ in 0..1000 {
            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs.clone()
            );
        }
    }

    #[test]
    fn returns_invalid_argument_error_when_max_lesser_than_min() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(rand:float 10.0 1.0)",
            "(rand:float 100.0 1.0)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_not_floats() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(rand:float 1)",
            "(rand:float #t)",
            "(rand:float #f)",
            "(rand:float \"string\")",
            "(rand:float 'symbol)",
            "(rand:float :keyword)",
            "(rand:float '(1 2))",
            "(rand:float {})",
            "(rand:float #())",

            "(rand:float 1.1 1)",
            "(rand:float 1.1 #t)",
            "(rand:float 1.1 #f)",
            "(rand:float 1.1 \"string\")",
            "(rand:float 1.1 'symbol)",
            "(rand:float 1.1 :keyword)",
            "(rand:float 1.1 '(1 2))",
            "(rand:float 1.1 {})",
            "(rand:float 1.1 #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_was_called_with_invalid_argument_count() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(rand:float 1.1 2.2 3.3)",
            "(rand:float 1.1 2.2 3.3 4.4)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
