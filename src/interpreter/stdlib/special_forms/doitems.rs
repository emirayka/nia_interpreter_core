use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::{Error, ErrorKind};
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;
use crate::interpreter::value::SymbolId;

fn make_doitems_environment(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    key_symbol_id: SymbolId,
    value_symbol_id: SymbolId,
) -> Result<EnvironmentId, Error> {
    let doitems_environment_id =
        interpreter.make_environment(environment_id)?;

    let break_symbol_id = interpreter.intern_symbol_id("break");
    let break_function_id = interpreter.get_internal_function("break")?;

    let continue_symbol_id = interpreter.intern_symbol_id("continue");
    let continue_function_id = interpreter.get_internal_function("continue")?;

    interpreter.define_function(
        doitems_environment_id,
        break_symbol_id,
        Value::Function(break_function_id),
    )?;

    interpreter.define_function(
        doitems_environment_id,
        continue_symbol_id,
        Value::Function(continue_function_id),
    )?;

    let nil = interpreter.intern_nil_symbol_value();

    interpreter.define_variable(doitems_environment_id, key_symbol_id, nil)?;
    interpreter.define_variable(
        doitems_environment_id,
        value_symbol_id,
        nil,
    )?;

    Ok(doitems_environment_id)
}

pub fn doitems(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 1 {
        return Error::invalid_argument_count_error(
            "Special form `doitems' takes one argument at least.",
        )
        .into();
    }

    let mut values = values;
    let binding = library::read_as_vector(interpreter, values.remove(0))?;

    if binding.len() != 3 {
        return Error::invalid_argument_error(
            "Special form `doitems' takes 3 item list as its first argument.",
        )
        .into();
    }

    let binding_key_symbol_id = library::read_as_symbol_id(binding[0])?;
    let binding_value_symbol_id = library::read_as_symbol_id(binding[1])?;

    let evaluated_value =
        interpreter.execute_value(environment_id, binding[2])?;

    let object_id = library::read_as_object_id(evaluated_value)?;
    let object_enumerable_keys =
        interpreter.get_object_enumerable_keys(object_id)?;

    let doitems_environment_id = make_doitems_environment(
        interpreter,
        environment_id,
        binding_key_symbol_id,
        binding_value_symbol_id,
    )?;

    let code = values;

    for enumerable_key in object_enumerable_keys {
        let value = interpreter.get_object_property(
            object_id,
            enumerable_key
        )?.ok_or_else(|| Error::generic_execution_error(
            "Somehow interpreter returned not internable property as internable."
        ))?;

        interpreter.set_environment_variable(
            doitems_environment_id,
            binding_key_symbol_id,
            enumerable_key.into(),
        )?;

        interpreter.set_environment_variable(
            doitems_environment_id,
            binding_value_symbol_id,
            value,
        )?;

        match library::evaluate_forms_return_last(
            interpreter,
            doitems_environment_id,
            &code,
        ) {
            Ok(_) => {}
            Err(error) => match error.get_error_kind() {
                ErrorKind::Break => {
                    break;
                }
                ErrorKind::Continue => {}
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
    fn loops_correctly() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(doitems (k v {:a 1 :b 2 :c 3}) 1)", "nil"),
            (
                r#"
                (let ((obj {})
                      (acc 0))
                  (doitems (key value obj)
                    (set! acc (+ acc value (object:get obj key))))
                  acc)
                "#,
                "0",
            ),
            (
                r#"
                (let ((obj {:key-1 1})
                      (acc 0))
                  (doitems (key value obj)
                    (set! acc (+ acc value (object:get obj key))))
                  acc)
                "#,
                "2",
            ),
            (
                r#"
                (let ((obj {:key-1 1 :key-2 2})
                      (acc 0))
                  (doitems (key value obj)
                    (set! acc (+ acc value (object:get obj key))))
                  acc)
                "#,
                "6",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn ignores_not_enumerable_and_not_internable_properties() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (
                r#"
                (let ((obj {:key-1 1 :key-2 2})
                      (acc 0))
                  (object:set-internable! obj :key-1 #f)
                  (doitems (key value obj)
                    (set! acc (+ acc value (object:get obj key))))
                  acc)
                "#,
                "4",
            ),
            (
                r#"
                (let ((obj {:key-1 1 :key-2 2})
                      (acc 0))
                  (object:set-enumerable! obj :key-1 #f)
                  (doitems (key value obj)
                    (set! acc (+ acc value (object:get obj key))))
                  acc)
                "#,
                "4",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn able_to_be_broken_and_continued_inside_of_loop() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (
                r#"
                (let ((obj {:key-1 1 :key-2 2})
                      (acc 0)
                      (count 0))
                  (doitems (key value obj)
                    (set! count (inc count))
                    (break)
                    (set! acc (+ acc value (object:get obj key))))
                  (list:new acc count))
                "#,
                "'(0 1)",
            ),
            (
                r#"
                (let ((obj {:key-1 1 :key-2 2})
                      (acc 0)
                      (count 0))
                  (doitems (key value obj)
                    (set! count (inc count))
                    (continue)
                    (set! acc (+ acc value (object:get obj key))))
                  (list:new acc count))
                "#,
                "'(0 2)",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs)
    }

    #[test]
    fn returns_invalid_argument_error_when_binding_is_not_a_two_item_list() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(doitems 1)",
            "(doitems 1.1)",
            "(doitems #t)",
            "(doitems #f)",
            "(doitems \"string\")",
            "(doitems symbol)",
            "(doitems :keyword)",
            "(doitems ())",
            "(doitems (1))",
            "(doitems (1 2 3))",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }

    #[test]
    fn returns_invalid_argument_errors_when_bindings_are_not_symbols() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(doitems (1 value {}))",
            "(doitems (1.1 value {}))",
            "(doitems (#t value {}))",
            "(doitems (#f value {}))",
            "(doitems (\"string\" value {}))",
            "(doitems ('(1 2) value {}))",
            "(doitems (:keyword value {}))",
            "(doitems ({} value {}))",
            // "(doitems (#() value {})", // todo: find out why there is a panic
            "(doitems (key 1 {}))",
            "(doitems (key 1.1 {}))",
            "(doitems (key #t {}))",
            "(doitems (key #f {}))",
            "(doitems (key \"string\" {}))",
            "(doitems (key '(1 2) {}))",
            "(doitems (key :keyword {}))",
            "(doitems (key {} {}))",
            // "(doitems (key #() {})", // todo: find out why there is a panic
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        )
    }

    #[test]
    fn returns_invalid_argument_errors_when_object_value_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(doitems (i 1))",
            "(doitems (i 1.1))",
            "(doitems (i #t))",
            "(doitems (i #f))",
            "(doitems (i \"string\"))",
            "(doitems (i 'symbol))",
            "(doitems (i '(list)))",
            "(doitems (i :keyword))",
            "(doitems (i #()))",
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

        let code_vector = vec!["(doitems)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        )
    }
}
