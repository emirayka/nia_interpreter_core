use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::library;

pub fn let_star(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() == 0 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `let*' must have at least one argument."
        ).into_result();
    }

    let mut values = values;

    let definitions = library::read_let_definitions(
        interpreter,
        values.remove(0)
    ).map_err(|_| interpreter.make_invalid_argument_error(
        "The first argument of special form `let*' must be a list of definitions: symbol, or 2-element lists."
    ))?;

    let forms = values;

    let execution_environment = interpreter.make_environment(environment)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    super::_let::set_definitions(
        interpreter,
        execution_environment,
        execution_environment,
        definitions
    )?;

    library::execute_forms(
        interpreter,
        execution_environment,
        forms
    )
}

// todo: simplify tests somehow by using tests of `let'
#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::{for_constants, for_special_symbols};

    // todo: ensure this test is fine
    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(1), interpreter.execute("(let* () 3 2 1)").unwrap());
    }

    // todo: ensure this test is fine
    #[test]
    fn sets_symbol_with_executed_value() {
        let mut interpreter = Interpreter::new();

        let symbol = interpreter.intern_symbol_value("symbol");
        let nil = interpreter.intern_nil_symbol_value();

        let cons = interpreter.make_cons_value(
            symbol,
            nil
        );

        let definitions = vec!(
            (Value::Integer(1), "1"),
            (Value::Float(1.1), "1.1"),
            (Value::Boolean(true), "#t"),
            (Value::Boolean(false), "#f"),
            (interpreter.intern_symbol_value("symbol"), "'symbol"),
            (interpreter.intern_symbol_value("symbol"), "(quote symbol)"),
            (interpreter.intern_string_value(String::from("string")), "\"string\""),
            (interpreter.intern_keyword_value(String::from("keyword")), ":keyword"),
            (cons, "'(symbol)"),
        );

        for (value, code_representation) in definitions {
            let result = interpreter.execute(
                &format!("(let* ((value {})) value)", code_representation)
            ).unwrap();

            assertion::assert_deep_equal(
                &mut interpreter,
                value,
                result
            );
        }
    }


    // todo: ensure this test is fine
    #[test]
    fn sets_symbol_without_value_to_nil() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            interpreter.intern_nil_symbol_value(),
            interpreter.execute("(let* (nil-symbol) nil-symbol)").unwrap()
        );
    }

    // todo: ensure this test is fine
    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(let* ((a 1)) a)").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(let* ((a 1)) (let* ((a 2) (b 3)) a))").unwrap()
        );

        assert_eq!(
            Value::Integer(3),
            interpreter.execute("(let* ((a 1)) (let* ((a 2) (b 3)) b))").unwrap()
        );
    }

    // the only difference between `let' `let*'
    // todo: ensure this test is fine
    #[test]
    fn able_to_use_previously_defined_values() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let* ((sym-1 1) (sym-2 sym-1)) sym-2)").unwrap();

        assert_eq!(Value::Integer(1), result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(let* (({} 2)) {})", constant, constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(let* (({} 2)) {})", special_symbol, special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_definition_is_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(let* ({}) {})", constant, constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(let* ({}) {})", special_symbol, special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    // todo: ensure this test is fine
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
                "(let* {})",
                incorrect_string
            ));

            assertion::assert_invalid_argument_error(&result);
        }
    }

    // todo: ensure this test is fine
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
                &format!("(let* ({}))", incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_first_part_of_definitions_is_not_a_symbol() {
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
                &format!("(let* (({} 2)) {})", incorrect_string, incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_nil() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let* ((nil 2)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let* ((sym)) nil)");

        assertion::assert_invalid_argument_error(&result);

        let result = interpreter.execute("(let* ((sym 1 2)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_err_when_attempt_to_redefine_already_defined_value() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let* ((sym-1 1) (sym-1 2)) sym-1)");

        assert!(result.is_err())
    }
}
