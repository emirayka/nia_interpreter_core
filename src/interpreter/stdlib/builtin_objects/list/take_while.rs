use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::lib;

pub fn take_while(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list:take-while' takes one argument."
        ).into_result();
    }

    let mut values = values;

    let function_id = lib::read_as_function_id(
        interpreter,
        values.remove(0)
    )?;

    let mut values = lib::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    let mut result = vec!();

    for value in values {
        let need_add = lib::execute_function(
            interpreter,
            _environment,
            function_id,
            vec!(value)
        )?;

        match need_add {
            Value::Boolean(true) => {
                result.push(value);
            },
            Value::Boolean(false) => {
                break;
            },
            _ => return interpreter.make_generic_execution_error(
                "Function returned non-boolean value."
            ).into_result()
        }
    }

    Ok(interpreter.vec_to_list(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn test_returns_correct_heads() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(list:take-while #(eq? (% %1 2) 0) '(1 2 3 4 5))", "'()"),
            ("(list:take-while #(eq? (% %1 2) 0) '(2 3 4 5))", "'(2)"),
            ("(list:take-while #(eq? (% %1 2) 0) '(3 4 5))", "'()"),
            ("(list:take-while #(eq? (% %1 2) 0) '(4 5))", "'(4)"),
            ("(list:take-while #(eq? (% %1 2) 0) '(5))", "'()"),

            ("(list:take-while #(eq? (/ %1 5) 0) '(1 2 3 4 5))", "'(1 2 3 4)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(list:take-while 1 '())",
            "(list:take-while 1.1 '())",
            "(list:take-while #t '())",
            "(list:take-while #f '())",
            "(list:take-while \"string\" '())",
            "(list:take-while 'symbol '())",
            "(list:take-while :keyword '())",
            "(list:take-while '(1 2 3) '())",
            "(list:take-while {} '())",

            "(list:take-while #() 1)",
            "(list:take-while #() 1.1)",
            "(list:take-while #() #t)",
            "(list:take-while #() #f)",
            "(list:take-while #() \"string\")",
            "(list:take-while #() 'symbol)",
            "(list:take-while #() :keyword)",
            "(list:take-while #() {})",
            "(list:take-while #() #())",
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
            "(list:take-while)",
            "(list:take-while 1)",
            "(list:take-while 1 2 3)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
