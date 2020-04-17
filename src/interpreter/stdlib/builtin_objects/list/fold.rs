use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::library;

pub fn fold(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:fold' takes three arguments exactly."
        ).into_result()
    }

    let mut values = values;

    let function_id = library::read_as_function_id(
        interpreter,
        values.remove(0)
    )?;

    let argument_values = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    let initial_value = values.remove(0);
    let mut acc = initial_value;

    for value in argument_values {
        let arguments = vec!(acc, value);

        acc = library::execute_function(
            interpreter,
            environment_id,
            function_id,
            arguments
        )?;
    }

    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn executes_function() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:fold (function (lambda (acc value) (+ acc value))) '() 0)", "0"),
            ("(list:fold (function (lambda (acc value) (+ acc value))) '(1 2 3 4) 0)", "10"),
            ("(list:fold (function (lambda (acc value) (+ acc value))) '(1 2 3 4 5) 0)", "15"),
            ("(list:fold (function (lambda (acc value) (cons value acc))) '(1 2 3 4 5) nil)", "(list 5 4 3 2 1)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:fold 1 '() nil)",
            "(list:fold 1.1 '() nil)",
            "(list:fold #t '() nil)",
            "(list:fold #f '() nil)",
            "(list:fold \"string\" '() nil)",
            "(list:fold 'symbol '() nil)",
            "(list:fold :keyword '() nil)",
            "(list:fold '(1 2 3) '() nil)",
            "(list:fold {} '() nil)",

            "(list:fold (function (lambda (_1 _2) nil)) 1 nil)",
            "(list:fold (function (lambda (_1 _2) nil)) 1.1 nil)",
            "(list:fold (function (lambda (_1 _2) nil)) #t nil)",
            "(list:fold (function (lambda (_1 _2) nil)) #f nil)",
            "(list:fold (function (lambda (_1 _2) nil)) \"string\" nil)",
            "(list:fold (function (lambda (_1 _2) nil)) 'symbol nil)",
            "(list:fold (function (lambda (_1 _2) nil)) :keyword nil)",
            "(list:fold (function (lambda (_1 _2) nil)) {} nil)",
            "(list:fold (function (lambda (_1 _2) nil)) #() nil)"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:fold)",
            "(list:fold 1)",
            "(list:fold 1 2)",
            "(list:fold 1 2 3 4)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
