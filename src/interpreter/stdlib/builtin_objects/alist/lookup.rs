use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn lookup(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `alist:lookup' takes two arguments exactly.",
        )
        .into();
    }

    // todo: check if alist
    library::check_value_is_list(interpreter, values[0])?;

    let alist_value = values[0];
    let key = values[1];
    let value = match library::alist_get(interpreter, key, alist_value)? {
        Some(value) => value,
        None => interpreter.intern_nil_symbol_value(),
    };

    return Ok(value);
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
            ("(alist:lookup (list:new) 0)", "nil"),
            ("(alist:lookup (list:new (cons:new 0 10)) 0)", "10"),
            ("(alist:lookup (list:new (cons:new 1 11) (cons:new 0 10)) 0)", "10"),
            ("(alist:lookup (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 0)", "10"),

            ("(alist:lookup (list:new) 1)", "nil"),
            ("(alist:lookup (list:new (cons:new 0 10)) 1)", "nil"),
            ("(alist:lookup (list:new (cons:new 1 11) (cons:new 0 10)) 1)", "11"),
            ("(alist:lookup (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 1)", "11"),

            ("(alist:lookup (list:new) 2)", "nil"),
            ("(alist:lookup (list:new (cons:new 0 10)) 2)", "nil"),
            ("(alist:lookup (list:new (cons:new 1 11) (cons:new 0 10)) 2)", "nil"),
            ("(alist:lookup (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 2)", "12"),

            ("(alist:lookup (list:new) 3)", "nil"),
            ("(alist:lookup (list:new (cons:new 0 10)) 3)", "nil"),
            ("(alist:lookup (list:new (cons:new 1 11) (cons:new 0 10)) 3)", "nil"),
            ("(alist:lookup (list:new (cons:new 2 12) (cons:new 1 11) (cons:new 0 10)) 3)", "nil"),
        );

        utils::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_invalid_argument_error_when_incorrect_value_was_provided() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(alist:lookup 1 :key)",
            "(alist:lookup 1.1 :key)",
            "(alist:lookup #t :key)",
            "(alist:lookup #f :key)",
            "(alist:lookup \"string\" :key)",
            "(alist:lookup :keyword :key)",
            "(alist:lookup 'symbol :key)",
            "(alist:lookup {} :key)",
            "(alist:lookup #() :key)",
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
            "(alist:lookup)",
            "(alist:lookup nil)",
            "(alist:lookup nil \"at\" 3)",
        ];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
