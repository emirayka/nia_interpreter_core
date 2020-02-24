use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::stdlib::_lib;

pub fn flet_star(
    interpreter: &mut Interpreter,
    special_form_calling_environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() == 0 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `flet*' must have at least one argument."
        ).into_result();
    }

    let mut values = values;

    let definitions = _lib::read_let_definitions(
        interpreter,
        values.remove(0)
    ).map_err(|_| interpreter.make_invalid_argument_error(
        "Special form `flet*' must have a first argument of function definitions"
    ))?;

    let forms = values;
    let function_definition_environment = interpreter.make_environment(
        special_form_calling_environment
    ).map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    super::flet::set_definitions(
        interpreter,
        function_definition_environment,
        function_definition_environment,
        definitions
    )?;

    _lib::execute_forms(
        interpreter,
        function_definition_environment,
        forms
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;
    use crate::interpreter::lib::testing_helpers::{for_constants, for_special_symbols};

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(3), interpreter.execute("(flet* () 3)").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(flet* () 3 2)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(flet* () 3 2 1)").unwrap());
    }

    #[test]
    fn able_to_execute_defined_functions() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(flet* ((test-func () 1)) (test-func))").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(flet* ((test-func (a) a)) (test-func 2))").unwrap()
        );
    }

    #[test]
    fn able_to_define_several_functions() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute(
                "(flet* ((test-func-1 () 1) (test-func-2 () 2) (test-func-3 () 3)) (test-func-1))"
            ).unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute(
                "(flet* ((test-func-1 () 1) (test-func-2 () 2) (test-func-3 () 3)) (test-func-2))"
            ).unwrap()
        );

        assert_eq!(
            Value::Integer(3),
            interpreter.execute(
                "(flet* ((test-func-1 () 1) (test-func-2 () 2) (test-func-3 () 3)) (test-func-3))"
            ).unwrap()
        );
    }

    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(flet* ((a () 1)) (a))").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(flet* ((a () 1)) (flet* ((a () 2) (b () 3)) (a)))").unwrap()
        );

        assert_eq!(
            Value::Integer(3),
            interpreter.execute("(flet* ((a () 1)) (flet* ((a () 2) (b () 3)) (b)))").unwrap()
        );
    }

    #[test]
    fn able_to_use_previously_defined_functions() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(flet* ((sym-1 () 1) (sym-2 () (sym-1))) (sym-2))");

        assert_eq!(Value::Integer(1), result.unwrap());
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(flet* (({} () 2)) {})", constant, constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(flet* (({} () 2)) {})", special_symbol, special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_error_when_first_argument_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let incorrect_strings = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
        );

        for incorrect_string in incorrect_strings {
            let result = interpreter.execute(&format!(
                "(flet* {})",
                incorrect_string
            ));

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_argument_contains_not_a_symbol_nor_cons() {
        let mut interpreter = Interpreter::new();

        let incorrect_strings = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
            "()",
            "nil",
        );

        for incorrect_string in incorrect_strings {
            let result = interpreter.execute(
                &format!("(flet* ({}))", incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_part_of_function_definition_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let incorrect_strings = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
            "(quote symbol)",
        );

        for incorrect_string in incorrect_strings {
            let result = interpreter.execute(
                &format!("(flet* (({} () 2)) {})", incorrect_string, incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_arguments_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let incorrect_strings = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
            "some-symbol",
        );

        for incorrect_string in incorrect_strings {
            let result = interpreter.execute(
                &format!("(flet* ((func {} 2)) (func))", incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_nil() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(flet* ((nil () 2)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(flet* ((sym)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_err_when_attempts_to_redefine_already_defined_function() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(flet* ((sym-1 () 1) (sym-1 () 2)) (sym-1))");

        assertion::assert_error(&result);
    }
}
