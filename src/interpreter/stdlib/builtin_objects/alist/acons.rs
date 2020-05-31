use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn acons(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `alist:acons' takes three arguments exactly.",
        )
        .into();
    }

    // todo: check if alist
    library::check_value_is_list(interpreter, values[0])?;

    let alist_value = values[0];
    let key = values[1];
    let value = values[2];

    let new_key_pair = interpreter.make_cons_value(key, value);
    let new_alist = interpreter.make_cons_value(new_key_pair, alist_value);

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
            ("(alist:acons nil 1 2)", "(list:new (cons:new 1 2))"),
            (
                "(alist:acons (alist:acons nil 1 2) 3 4)",
                "(list:new (cons:new 3 4) (cons:new 1 2))",
            ),
            (
                "(alist:acons (alist:acons (alist:acons nil 1 2) 3 4) 5 6)",
                "(list:new (cons:new 5 6) (cons:new 3 4) (cons:new 1 2))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(alist:acons 1 :key 'value)",
            "(alist:acons 1.1 :key 'value)",
            "(alist:acons #t :key 'value)",
            "(alist:acons #f :key 'value)",
            "(alist:acons \"string\" :key 'value)",
            "(alist:acons :keyword :key 'value)",
            "(alist:acons 'symbol :key 'value)",
            "(alist:acons {} :key 'value)",
            "(alist:acons #() :key 'value)",
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
            "(alist:acons)",
            "(alist:acons nil)",
            "(alist:acons nil \"at\")",
            "(alist:acons nil \"at\" 3 4)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
