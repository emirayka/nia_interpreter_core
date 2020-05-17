use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn with_this(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Error::invalid_argument_count_error(
            "Special form `with-this' takes one argument at least.",
        )
        .into();
    }

    let mut values = values;

    let first_value = values.remove(0);
    let first_value_evaluated =
        interpreter.execute_value(environment_id, first_value)?;

    let object_id = library::read_as_object_id(first_value_evaluated)?;

    let code = values;
    let previous_this = interpreter.get_this_object();

    interpreter.set_this_object(object_id);

    let result =
        library::evaluate_forms_return_last(interpreter, environment_id, &code);

    match previous_this {
        Some(previous_this_object_id) => {
            interpreter.set_this_object(previous_this_object_id);
        },
        None => {
            interpreter.clear_this_object();
        },
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
    fn sets_this_object_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("(with-this {:a 1} (list:new this:a))", "'(1)"),
            ("(with-this {:a 1 :b 2} (list:new this:a this:b))", "'(1 2)"),
            ("(with-this {:f1 (fn () 1)} (this:f1))", "1"),
            (
                "(with-this {:f1 (fn () 1) :f2 (fn () (+ (this:f1) (this:f1)))} (this:f2))",
                "2",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn returns_err_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(with-this)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_incorrect_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(with-this 1 1)",
            "(with-this 1.1 1)",
            "(with-this #t 1)",
            "(with-this #f 1)",
            "(with-this \"string\" 1)",
            "(with-this :keyword 1)",
            "(with-this 'symbol 1)",
            "(with-this '(list:new) 1)",
            "(with-this '() 1)",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }
}
