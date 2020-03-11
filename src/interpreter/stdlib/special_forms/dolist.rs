use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::{Error, ErrorKind};
use crate::interpreter::environment::EnvironmentId;

use crate::interpreter::library;
use crate::interpreter::symbol::SymbolId;

fn make_dolist_environment(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    symbol_id: SymbolId
) -> Result<EnvironmentId, Error> {
    let dolist_environment_id = interpreter.make_environment(
        environment_id
    )?;

    let break_symbol_id = interpreter.intern("break");
    let break_function_id = interpreter.get_internal_function("break")?;

    let continue_symbol_id = interpreter.intern("continue");
    let continue_function_id = interpreter.get_internal_function("continue")?;

    interpreter.define_function(
        dolist_environment_id,
        break_symbol_id,
        Value::Function(break_function_id)
    )?;

    interpreter.define_function(
        dolist_environment_id,
        continue_symbol_id,
        Value::Function(continue_function_id)
    )?;

    let nil = interpreter.intern_nil_symbol_value();
    interpreter.define_variable(
        dolist_environment_id,
        symbol_id,
        nil
    )?;

    Ok(dolist_environment_id)
}

pub fn dolist(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 1 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `dolist' takes one argument at least."
        ).into_result();
    }

    let mut values = values;
    let mut binding = library::read_as_vector(
        interpreter,
        values.remove(0)
    )?;

    if binding.len() != 2 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `dolist' takes 2 item list as its first argument."
        ).into_result()
    }

    let binding_symbol_id = library::read_as_symbol_id(
        interpreter,
        binding.remove(0)
    )?;

    let evaluated_list = interpreter.evaluate_value(
        environment_id,
        binding.remove(0)
    )?;

    let vector = library::read_as_vector(
        interpreter,
        evaluated_list
    )?;

    let dolist_environment_id = make_dolist_environment(
        interpreter,
        environment_id,
        binding_symbol_id
    )?;

    let code = values;

    for value in vector {
        interpreter.set_environment_variable(
            dolist_environment_id,
            binding_symbol_id,
            value
        );

        match library::execute_forms(
            interpreter,
            dolist_environment_id,
            &code
        ) {
            Ok(_) => {},
            Err(error) => {
                match error.get_error_kind() {
                    ErrorKind::Break => {
                        break;
                    },
                    ErrorKind::Continue => {},
                    _ => return Err(error)
                }
            }
        };
    }

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;

    #[test]
    fn loops() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(dolist (i '(1 2)))", "nil"),
            ("(dolist (i '(1 2 3)))", "nil"),
            ("(defv a (list)) (dolist (i '(1 2 3)) (set! a (cons i a))) a", "'(3 2 1)"),
            ("(defv b (list)) (dolist (i '(1 2 3)) (break) (set! b (cons i b))) b", "'()"),
            ("(defv c (list)) (dolist (i '(1 2 3)) (set! c (cons i c)) (set! c (cons i c))) c", "'(3 3 2 2 1 1)"),
            ("(defv d (list)) (dolist (i '(1 2 3)) (set! d (cons i d)) (continue) (set! d (cons i d))) d", "'(3 2 1)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_invalid_argument_count_when_was_called_with_invalid_argument_count() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(dolist)",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        )
    }
}
