use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn has_value_question(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `alist:has-value?' takes two arguments exactly.",
        )
        .into();
    }

    library::check_value_is_list(interpreter, values[0])?;

    let alist_value = values[0];
    let value = values[1];

    let result = library::alist_contains_value_question(
        interpreter,
        value,
        alist_value,
    )?;

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
            ("(alist:has-value? '() 0)", "#f"),
            ("(alist:has-value? '() 1)", "#f"),
            ("(alist:has-value? '() 2)", "#f"),

            ("(alist:has-value? (list:new (cons:new 0 10)) 10)", "#t"),
            ("(alist:has-value? (list:new (cons:new 0 10)) 11)", "#f"),
            ("(alist:has-value? (list:new (cons:new 0 10)) 12)", "#f"),

            ("(alist:has-value? (list:new (cons:new 1 11) (cons:new 0 10)) 10)", "#t"),
            ("(alist:has-value? (list:new (cons:new 1 11) (cons:new 0 10)) 11)", "#t"),
            ("(alist:has-value? (list:new (cons:new 1 11) (cons:new 0 10)) 12)", "#f"),

            ("(alist:has-value? (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 10)", "#t"),
            ("(alist:has-value? (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 11)", "#t"),
            ("(alist:has-value? (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 12)", "#t"),
        );

        utils::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(alist:has-value? 1 :key)",
            "(alist:has-value? 1.1 :key)",
            "(alist:has-value? #t :key)",
            "(alist:has-value? #f :key)",
            "(alist:has-value? \"string\" :key)",
            "(alist:has-value? :keyword :key)",
            "(alist:has-value? 'symbol :key)",
            "(alist:has-value? {} :key)",
            "(alist:has-value? #() :key)",
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
            "(alist:has-value?)",
            "(alist:has-value? nil)",
            "(alist:has-value? nil \"at\" 3)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
