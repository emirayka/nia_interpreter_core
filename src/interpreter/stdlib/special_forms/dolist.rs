use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::{Error, ErrorKind};
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::interpreter::value::SymbolId;

fn make_dolist_environment(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    symbol_id: SymbolId,
) -> Result<EnvironmentId, Error> {
    let dolist_environment_id = interpreter.make_environment(environment_id)?;

    let break_symbol_id = interpreter.intern_symbol_id("break");
    let break_function_id = interpreter.get_internal_function("break")?;

    let continue_symbol_id = interpreter.intern_symbol_id("continue");
    let continue_function_id = interpreter.get_internal_function("continue")?;

    interpreter.define_function(
        dolist_environment_id,
        break_symbol_id,
        Value::Function(break_function_id),
    )?;

    interpreter.define_function(
        dolist_environment_id,
        continue_symbol_id,
        Value::Function(continue_function_id),
    )?;

    let nil = interpreter.intern_nil_symbol_value();
    interpreter.define_variable(dolist_environment_id, symbol_id, nil)?;

    Ok(dolist_environment_id)
}

pub fn dolist(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 1 {
        return Error::invalid_argument_count_error(
            "Special form `dolist' takes one argument at least.",
        )
        .into();
    }

    let mut values = values;
    let mut binding = library::read_as_vector(interpreter, values.remove(0))?;

    if binding.len() != 2 {
        return Error::invalid_argument_error(
            "Special form `dolist' takes 2 item list as its first argument.",
        )
        .into();
    }

    let binding_symbol_id = library::read_as_symbol_id(binding.remove(0))?;

    let evaluated_list =
        interpreter.execute_value(environment_id, binding.remove(0))?;

    let vector = library::read_as_vector(interpreter, evaluated_list)?;

    let dolist_environment_id = make_dolist_environment(
        interpreter,
        environment_id,
        binding_symbol_id,
    )?;

    let code = values;

    for value in vector {
        interpreter.set_environment_variable(
            dolist_environment_id,
            binding_symbol_id,
            value,
        )?;

        match library::evaluate_forms_return_last(
            interpreter,
            dolist_environment_id,
            &code,
        ) {
            Ok(_) => {},
            Err(error) => match error.get_error_kind() {
                ErrorKind::Break => {
                    break;
                },
                ErrorKind::Continue => {},
                _ => return Err(error),
            },
        };
    }

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn loops() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(dolist (i '(1 2)))", "nil"),
            ("(dolist (i '(1 2 3)))", "nil"),
            ("(defv lst (list 1 2 3)) (dolist (i lst))", "nil"),
            (
                "(defv a (list)) (dolist (i '(1 2 3)) (set! a (cons:new i a))) a",
                "'(3 2 1)",
            ),
            (
                "(defv b (list)) (dolist (i '(1 2 3)) (break) (set! b (cons:new i b))) b",
                "'()",
            ),
            (
                "(defv c (list)) (dolist (i '(1 2 3)) (set! c (cons:new i c)) (set! c (cons:new i c))) c",
                "'(3 3 2 2 1 1)",
            ),
            (
                "(defv d (list)) (dolist (i '(1 2 3)) (set! d (cons:new i d)) (continue) (set! d (cons:new i d))) d",
                "'(3 2 1)",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_when_binding_is_not_a_two_item_list() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(dolist 1)",
            "(dolist 1.1)",
            "(dolist #t)",
            "(dolist #f)",
            "(dolist \"string\")",
            "(dolist symbol)",
            "(dolist :keyword)",
            "(dolist ())",
            "(dolist (1))",
            "(dolist (1 2 3))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }

    #[test]
    fn returns_invalid_argument_errors_when_binding_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(dolist (1 '(1 2)))",
            "(dolist (1.1 '(1 2)))",
            "(dolist (#t '(1 2)))",
            "(dolist (#f '(1 2)))",
            "(dolist (\"string\" '(1 2)))",
            "(dolist ('(1 2) '(1 2)))",
            "(dolist (:keyword) '(1 2))",
            "(dolist ({} '(1 2)))",
            "(dolist (#() '(1 2)))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }

    #[test]
    fn returns_invalid_argument_errors_when_list_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(dolist (i 1))",
            "(dolist (i 1.1))",
            "(dolist (i #t))",
            "(dolist (i #f))",
            "(dolist (i \"string\"))",
            "(dolist (i 'symbol))",
            "(defv a 1) (dolist (i a))",
            "(dolist (i :keyword))",
            "(dolist (i {}))",
            "(dolist (i #()))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }

    #[test]
    fn returns_invalid_argument_count_when_was_called_with_invalid_argument_count(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(dolist)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        )
    }
}
