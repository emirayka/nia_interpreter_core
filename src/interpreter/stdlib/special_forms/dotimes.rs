use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::{Error, ErrorKind};
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::interpreter::value::SymbolId;

fn make_dotimes_environment(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    symbol_id: SymbolId,
) -> Result<EnvironmentId, Error> {
    let dotimes_environment_id =
        interpreter.make_environment(environment_id)?;

    let break_symbol_id = interpreter.intern_symbol_id("break");
    let break_function_id = interpreter.get_internal_function("break")?;

    let continue_symbol_id = interpreter.intern_symbol_id("continue");
    let continue_function_id = interpreter.get_internal_function("continue")?;

    interpreter.define_function(
        dotimes_environment_id,
        break_symbol_id,
        Value::Function(break_function_id),
    )?;

    interpreter.define_function(
        dotimes_environment_id,
        continue_symbol_id,
        Value::Function(continue_function_id),
    )?;

    let nil = interpreter.intern_nil_symbol_value();
    interpreter.define_variable(dotimes_environment_id, symbol_id, nil)?;

    Ok(dotimes_environment_id)
}

pub fn dotimes(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 1 {
        return Error::invalid_argument_count_error(
            "Special form dotimes`' takes one argument at least.",
        )
        .into();
    }

    let mut values = values;
    let mut binding = library::read_as_vector(interpreter, values.remove(0))?;

    if binding.len() != 2 {
        return Error::invalid_argument_error(
            "Special form `dotimes' takes 2 item list as its first argument.",
        )
        .into();
    }

    let binding_symbol_id = library::read_as_symbol_id(binding.remove(0))?;

    let evaluated_count =
        interpreter.execute_value(environment_id, binding.remove(0))?;

    let count = library::read_as_i64(evaluated_count)?;

    if count < 0 {
        return Error::invalid_argument_error(
            "Special form `dotimes' takes positive count.",
        )
        .into();
    }

    let dotimes_environment_id = make_dotimes_environment(
        interpreter,
        environment_id,
        binding_symbol_id,
    )?;

    let code = values;

    for index in 0..count {
        interpreter.set_environment_variable(
            dotimes_environment_id,
            binding_symbol_id,
            Value::Integer(index),
        )?;

        match library::evaluate_forms_return_last(
            interpreter,
            dotimes_environment_id,
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
            ("(dotimes (i 1))", "nil"),
            ("(defv n 1) (dotimes (i n))", "nil"),
            (
                "(defv a (list:new)) (dotimes (i 3) (set! a (cons:new i a))) a",
                "'(2 1 0)",
            ),
            (
                "(defv b (list:new)) (dotimes (i 3) (break) (set! b (cons:new i b))) b",
                "'()",
            ),
            (
                "(defv c (list:new)) (dotimes (i 3) (set! c (cons:new i c)) (set! c (cons:new i c))) c",
                "'(2 2 1 1 0 0)",
            ),
            (
                "(defv d (list:new)) (dotimes (i 3) (set! d (cons:new i d)) (continue) (set! d (cons:new i d))) d",
                "'(2 1 0)",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_when_binding_is_not_a_two_item_list() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(dotimes 1)",
            "(dotimes 1.1)",
            "(dotimes #t)",
            "(dotimes #f)",
            "(dotimes \"string\")",
            "(dotimes symbol)",
            "(dotimes :keyword)",
            "(dotimes ())",
            "(dotimes (1))",
            "(dotimes (1 2 3))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }

    #[test]
    fn returns_invalid_argument_error_when_binding_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(dotimes (1 5))",
            "(dotimes (1.1 5))",
            "(dotimes (#t 5))",
            "(dotimes (#f 5))",
            "(dotimes (:keyword 5))",
            "(dotimes (\"string\" 5))",
            "(dotimes ('(1 2) 5))",
            "(dotimes ({} 5))",
            "(dotimes (#() 5))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }

    #[test]
    fn returns_invalid_argument_error_when_count_did_not_evaluate_to_integer() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(dotimes (n 1.1))",
            "(dotimes (n #t))",
            "(dotimes (n #f))",
            "(dotimes (n \"string\"))",
            "(dotimes (n 'symbol))",
            "(defv not-int 'sym) (dotimes (n not-int))",
            "(dotimes (n :keyword))",
            "(dotimes (n '(1 2)))",
            "(dotimes (n {}))",
            "(dotimes (n #()))",
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

        let code_vector = vec!["(dotimes)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        )
    }
}
