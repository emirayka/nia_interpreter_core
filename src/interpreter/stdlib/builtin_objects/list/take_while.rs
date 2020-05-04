use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn take_while(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:take-while' takes one argument.",
        )
        .into();
    }

    let mut values = values;

    let function_id = library::read_as_function_id(values.remove(0))?;

    let values = library::read_as_vector(interpreter, values.remove(0))?;

    let mut result = vec![];

    for value in values {
        let need_add =
            library::execute_function(interpreter, _environment, function_id, vec![value])?;

        match need_add {
            Value::Boolean(true) => {
                result.push(value);
            }
            Value::Boolean(false) => {
                break;
            }
            _ => {
                return Error::generic_execution_error("Function returned non-boolean value.")
                    .into()
            }
        }
    }

    Ok(interpreter.vec_to_list(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_correct_heads() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(list:take-while #(eq? (% %1 2) 0) '(1 2 3 4 5))", "'()"),
            ("(list:take-while #(eq? (% %1 2) 0) '(2 3 4 5))", "'(2)"),
            ("(list:take-while #(eq? (% %1 2) 0) '(3 4 5))", "'()"),
            ("(list:take-while #(eq? (% %1 2) 0) '(4 5))", "'(4)"),
            ("(list:take-while #(eq? (% %1 2) 0) '(5))", "'()"),
            (
                "(list:take-while #(eq? (/ %1 5) 0) '(1 2 3 4 5))",
                "'(1 2 3 4)",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
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
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:take-while)",
            "(list:take-while 1)",
            "(list:take-while 1 2 3)",
        ];

        assertion::assert_results_are_invalid_argument_count_errors(&mut interpreter, code_vector);
    }
}
