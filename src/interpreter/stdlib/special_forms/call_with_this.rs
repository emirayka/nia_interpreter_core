use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn call_with_this(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 2 {
        return Error::invalid_argument_count_error(
            "Special form `call-with-this' takes two arguments at least.",
        )
        .into();
    }

    let mut values = values;

    let context_evaluated =
        interpreter.execute_value(environment_id, values.remove(0))?;
    let context_object_id = library::read_as_object_id(context_evaluated)?;

    let function_evaluated =
        interpreter.execute_value(environment_id, values.remove(0))?;
    let function_id = library::read_as_function_id(function_evaluated)?;

    let arguments = values;
    let evaluated_arguments =
        library::evaluate_forms(interpreter, environment_id, arguments)?;

    let previous_this = interpreter.get_this_object();

    interpreter.set_this_object(context_object_id);

    let result = interpreter.execute_function_with_evaluated_arguments(
        function_id,
        environment_id,
        evaluated_arguments,
    );

    match previous_this {
        Some(previous_this_object_id) => {
            interpreter.set_this_object(previous_this_object_id);
        }
        None => {
            interpreter.clear_this_object();
        }
    }

    let result = result?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn invokes_function_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("(call-with-this {:a 1} list:new)", "nil"),
            ("(call-with-this {:a 1} list:new 1)", "'(1)"),
            ("(call-with-this {:a 1} list:new 1 2)", "'(1 2)"),
            ("(call-with-this {} (let ((a 1) (b 2)) #(+ a b)))", "3"),
            ("(call-with-this {} (let ((a 1) (b 2)) #(+ a b %1)) 3)", "6"),
            (
                "(call-with-this {} (let ((a 1) (b 2)) #(+ a b %1 %2)) 3 4)",
                "10",
            ),

            ("(flet ((f () (+ this:a this:b))) (call-with-this {:a 1 :b 2} (flookup 'f)))", "3"),

            ("(with-this {:a 1 :b 2} (call-with-this list (:new list) this:a this:b))", "'(1 2)")
        ];

        utils::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_err_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(call-with-this)", "(call-with-this {})"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_incorrect_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(call-with-this 1 1)",
            "(call-with-this 1.1 1)",
            "(call-with-this #t 1)",
            "(call-with-this #f 1)",
            "(call-with-this \"string\" 1)",
            "(call-with-this :keyword 1)",
            "(call-with-this 'symbol 1)",
            "(call-with-this '(list:new) 1)",
            "(call-with-this #() 1)",
            "(call-with-this {} 1)",
            "(call-with-this {} 1.1)",
            "(call-with-this {} #t)",
            "(call-with-this {} #f)",
            "(call-with-this {} \"string\")",
            "(call-with-this {} :keyword)",
            "(call-with-this {} 'symbol)",
            "(call-with-this {} '(list))",
            "(call-with-this {} {})",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }
}
