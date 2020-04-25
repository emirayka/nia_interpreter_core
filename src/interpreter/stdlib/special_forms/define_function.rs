use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::library;

pub fn define_function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() < 1 || values.len() > 3 {
        return Error::invalid_argument_count_error(
            "Special form `define-function' must be used with one or two or three forms."
        ).into_result();
    }

    let first_argument = values.remove(0);
    let second_argument = if values.len() > 0 {
        Some(values.remove(0))
    } else {
        None
    };

    let need_to_be_const = if values.len() > 0 {
        let result = library::read_as_keyword(
            interpreter,
            values.remove(0),
        )?.is_const();

        if !result {
            return Error::invalid_argument_error(
                "Third argument of special form `define-function' must be a keyword `:const'."
            ).into_result();
        }

        result
    } else {
        false
    };

    let function_symbol_id = match first_argument {
        Value::Symbol(symbol) => symbol,
        _ => return Error::invalid_argument_error(
            "First form of `define-function' must be a symbol."
        ).into_result()
    };

    library::check_if_symbol_assignable(interpreter, function_symbol_id)?;

    let evaluated_value = match second_argument {
        Some(value) => interpreter.evaluate_value(environment, value),
        None => Ok(interpreter.intern_nil_symbol_value())
    }.map_err(|err| Error::generic_execution_error_caused(
        "Cannot evaluate the second form of define-function.",
        err,
    ))?;

    let result = if need_to_be_const {
        interpreter.define_const_function(
            interpreter.get_root_environment(),
            function_symbol_id,
            evaluated_value,
        )
    } else {
        interpreter.define_function(
            interpreter.get_root_environment(),
            function_symbol_id,
            evaluated_value,
        )
    };

    result.map_err(|err| {
        let symbol_name = match interpreter.get_symbol_name(function_symbol_id) {
            Ok(symbol_name) => symbol_name,
            _ => return Error::generic_execution_error("")
        };

        let message = &format!(
            "Cannot define function: {}.",
            symbol_name
        );
        Error::generic_execution_error_caused(
            message,
            err,
        )
    })?;

    Ok(Value::Boolean(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::{for_constants, for_special_symbols};

    #[test]
    fn defines_function_with_evaluation_result_of_the_second_form_when_two_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test 2)").unwrap();
        let name = interpreter.intern("test");

        assert!(interpreter.has_function(
            interpreter.get_root_environment(),
            name,
        ).unwrap());

        assert_eq!(
            Value::Integer(2),
            interpreter.lookup_function(
                interpreter.get_root_environment(),
                name,
            ).unwrap()
        );
    }

    #[test]
    fn defines_function_with_nil_when_one_form_were_provided() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test)").unwrap();
        let name = interpreter.intern("test");

        assert!(interpreter.has_function(
            interpreter.get_root_environment(),
            name)
            .unwrap());
        assert_eq!(
            interpreter.intern_nil_symbol_value(),
            interpreter.lookup_function(
                interpreter.get_root_environment(),
                name,
            ).unwrap()
        );
    }

    #[test]
    fn defines_function_that_can_be_executed() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test (function (lambda (a b) b)))").unwrap();
        let result = interpreter.execute("(test 2 3)");

        assert_eq!(Value::Integer(3), result.unwrap());
    }

    #[test]
    fn able_to_define_const_function() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test (function (lambda (a b) b)) :const)").unwrap();

        let result = interpreter.execute("(test 2 3)");
        assert_eq!(Value::Integer(3), result.unwrap());

        let result = interpreter.execute("(fset! test 2)");
        assertion::assert_is_err(result)
    }

    #[test]
    fn possible_to_make_a_closure() {
        let mut interpreter = Interpreter::new();

        interpreter.execute("(define-function test (function (lambda (a) (function (lambda () a)))))").unwrap();
        interpreter.execute("(define-function test2 (test 2))").unwrap();

        assert_eq!(Value::Integer(2), interpreter.execute("(test2)").unwrap());
    }

    #[test]
    fn returns_error_when_attempts_to_define_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(define-function {} #(% 2 3))", constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(define-function {} #(% 2 3))", special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_err_when_incorrect_count_of_forms_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(define-function)",
            "(define-function test 2 :const 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_an_incorrect_form_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(define-function 3 2)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }
}
