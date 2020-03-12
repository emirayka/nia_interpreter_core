use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;
use rand;
use rand::Rng;

pub fn int(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() > 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `rand:int' takes no more than two arguments."
        ).into_result()
    }

    let mut values = values;

    let min = if values.len() > 0 {
        library::read_as_i64(
            interpreter,
            values.remove(0)
        )?
    } else {
        std::i64::MIN
    };

    let max = if values.len() > 0 {
        library::read_as_i64(
            interpreter,
            values.remove(0)
        )?
    } else {
        std::i64::MAX
    };

    if min > max {
        return interpreter.make_invalid_argument_error(
            "Built-in function `rand:int' expects min <= max."
        ).into_result()
    }

    let mut rng = rand::thread_rng();
    let result = rng.gen_range(min, max);

    Ok(Value::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::for_special_symbols;

    #[test]
    fn returns_random_int() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(is:int? (rand:int))", "#t"),
            ("(>= (rand:int 1) 1)", "#t"),
            ("(let ((a (rand:int 1 100))) (and (>= a 1) (< a 100)))", "#t"),
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
            "(rand:int 10 1)",
            "(rand:int 100 1)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_was_called_with_not_integers() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(rand:int 1.1)",
            "(rand:int #t)",
            "(rand:int #f)",
            "(rand:int \"string\")",
            "(rand:int 'symbol)",
            "(rand:int :keyword)",
            "(rand:int '(1 2))",
            "(rand:int {})",
            "(rand:int #())",

            "(rand:int 1 1.1)",
            "(rand:int 1 #t)",
            "(rand:int 1 #f)",
            "(rand:int 1 \"string\")",
            "(rand:int 1 'symbol)",
            "(rand:int 1 :keyword)",
            "(rand:int 1 '(1 2))",
            "(rand:int 1 {})",
            "(rand:int 1 #())",
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
            "(rand:int 1 2 3)",
            "(rand:int 1 2 3 4)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
