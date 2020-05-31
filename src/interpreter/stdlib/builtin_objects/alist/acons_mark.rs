use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn acons_mark(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `alist:acons!' takes three arguments exactly.",
        )
        .into();
    }

    let alist_symbol_name = library::read_as_symbol_id(values[0])?;
    let defined_variable_environment_id = interpreter
        .lookup_environment_by_variable(environment_id, alist_symbol_name)?
        .ok_or_else(|| {
            Error::generic_execution_error(format!(
                "Cannot find variable with name."
            ))
        })?;

    let alist_value = interpreter
        .lookup_variable(defined_variable_environment_id, alist_symbol_name)?
        .ok_or_else(|| {
            Error::generic_execution_error(format!(
                "Cannot find variable with name."
            ))
        })?;

    library::check_value_is_list(interpreter, alist_value)?;

    let key = values[1];
    let value = values[2];

    let new_key_pair = interpreter.make_cons_value(key, value);
    let new_alist = interpreter.make_cons_value(new_key_pair, alist_value);

    interpreter.set_variable(
        defined_variable_environment_id,
        alist_symbol_name,
        new_alist,
    )?;

    return Ok(new_alist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn makes_new_acons_cells() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("(defv test nil) test", "'()"),
            ("(alist:acons! 'test 1 2) test", "(list:new (cons:new 1 2))"),
            (
                "(alist:acons! 'test 3 4) test",
                "(list:new (cons:new 3 4) (cons:new 1 2))",
            ),
            (
                "(alist:acons! 'test 5 6) test",
                "(list:new (cons:new 5 6) (cons:new 3 4) (cons:new 1 2))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(alist:acons! 1 :key 'value)",
            "(alist:acons! 1.1 :key 'value)",
            "(alist:acons! #t :key 'value)",
            "(alist:acons! #f :key 'value)",
            "(alist:acons! \"string\" :key 'value)",
            "(alist:acons! :keyword :key 'value)",
            "(alist:acons! '(list) :key 'value)",
            "(alist:acons! {} :key 'value)",
            "(alist:acons! #() :key 'value)",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(alist:acons!)",
            "(alist:acons! nil)",
            "(alist:acons! nil \"at\")",
            "(alist:acons! nil \"at\" 3 4)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
