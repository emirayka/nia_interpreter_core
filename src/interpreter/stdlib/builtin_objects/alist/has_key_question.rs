use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn has_key_question(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `alist:has-key?' takes two arguments exactly.",
        )
        .into();
    }

    library::check_value_is_list(interpreter, values[0])?;

    let alist_value = values[0];
    let key = values[1];

    let result =
        library::alist_contains_key_question(interpreter, key, alist_value)?;

    Ok(Value::Boolean(result))
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

        let specs = vec!(
            ("(alist:has-key? '() 0)", "#f"),
            ("(alist:has-key? '() 1)", "#f"),
            ("(alist:has-key? '() 2)", "#f"),

            ("(alist:has-key? (list:new (cons:new 0 10)) 0)", "#t"),
            ("(alist:has-key? (list:new (cons:new 0 10)) 1)", "#f"),
            ("(alist:has-key? (list:new (cons:new 0 10)) 2)", "#f"),

            ("(alist:has-key? (list:new (cons:new 1 11) (cons:new 0 10)) 0)", "#t"),
            ("(alist:has-key? (list:new (cons:new 1 11) (cons:new 0 10)) 1)", "#t"),
            ("(alist:has-key? (list:new (cons:new 1 11) (cons:new 0 10)) 2)", "#f"),

            ("(alist:has-key? (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 0)", "#t"),
            ("(alist:has-key? (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 1)", "#t"),
            ("(alist:has-key? (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 2)", "#t"),
        );

        utils::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(alist:has-key? 1 :key)",
            "(alist:has-key? 1.1 :key)",
            "(alist:has-key? #t :key)",
            "(alist:has-key? #f :key)",
            "(alist:has-key? \"string\" :key)",
            "(alist:has-key? :keyword :key)",
            "(alist:has-key? 'symbol :key)",
            "(alist:has-key? {} :key)",
            "(alist:has-key? #() :key)",
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
            "(alist:has-key?)",
            "(alist:has-key? nil)",
            "(alist:has-key? nil \"at\" 2 3)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
