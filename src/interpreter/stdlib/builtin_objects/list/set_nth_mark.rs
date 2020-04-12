use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

pub fn set_nth_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:set-nth!' takes three arguments exactly."
        ).into_result();
    }

    let mut values = values;

    let setting_value = values.remove(0);

    let index = library::read_as_i64(
        interpreter,
        values.remove(0),
    )? as usize;

    let mut cons_id = library::read_as_cons_id(
        interpreter,
        values.remove(0),
    )?;

    for _ in 0..index {
        let next_cdr = interpreter.get_cdr(cons_id)?;

        cons_id = library::read_as_cons_id(
            interpreter,
            next_cdr
        )?;
    }

    interpreter.set_car(cons_id, setting_value);

    Ok(setting_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn sets_values_at_correct_positions() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((b (list 1 2 3 4 5))) (list:set-nth! 0 0 b) b)", "'(0 2 3 4 5)"),
            ("(let ((b (list 1 2 3 4 5))) (list:set-nth! 0 1 b) b)", "'(1 0 3 4 5)"),
            ("(let ((b (list 1 2 3 4 5))) (list:set-nth! 0 2 b) b)", "'(1 2 0 4 5)"),
            ("(let ((b (list 1 2 3 4 5))) (list:set-nth! 0 3 b) b)", "'(1 2 3 0 5)"),
            ("(let ((b (list 1 2 3 4 5))) (list:set-nth! 0 4 b) b)", "'(1 2 3 4 0)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_list_has_zero_length() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:set-nth! 1 0 '())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:set-nth! 1 1.1 '(1 2 3))",
            "(list:set-nth! 1 #t '(1 2 3))",
            "(list:set-nth! 1 #f '(1 2 3))",
            "(list:set-nth! 1 \"string\" '(1 2 3))",
            "(list:set-nth! 1 'symbol '(1 2 3))",
            "(list:set-nth! 1 :keyword '(1 2 3))",
            "(list:set-nth! 1 '(1 2 3) '(1 2 3))",
            "(list:set-nth! 1 {} '(1 2 3))",
            "(list:set-nth! 1 #() '(1 2 3))",

            "(list:set-nth! 1 0 1)",
            "(list:set-nth! 1 0 1.1)",
            "(list:set-nth! 1 0 #t)",
            "(list:set-nth! 1 0 #f)",
            "(list:set-nth! 1 0 \"string\")",
            "(list:set-nth! 1 0 'symbol)",
            "(list:set-nth! 1 0 :keyword)",
            "(list:set-nth! 1 0 {})",
            "(list:set-nth! 1 0 #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:set-nth!)",
            "(list:set-nth! 1)",
            "(list:set-nth! 1 0)",
            "(list:set-nth! 1 0 '(1 2 3 4) 5)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
