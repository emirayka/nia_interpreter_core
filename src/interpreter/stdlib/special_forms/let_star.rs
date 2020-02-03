use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::cons::Cons;
use crate::interpreter::symbol::Symbol;

fn let_star(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Special form let must have at least one argument."
        ));
    }

    let mut values = values;

    let definitions = match values.remove(0) {
        Value::Cons(cons) => cons.to_vec(),
        Value::Symbol(symbol) if symbol.is_nil() => Vec::new(),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first argument of special form let must be a list of variable definitions."
        ))
    };

    let forms = values;

    let execution_environment = interpreter.make_environment(environment);

    super::_let::set_definitions(
        interpreter,
        execution_environment,
        execution_environment,
        definitions
    )?;

    super::_lib::execute_forms(
        interpreter,
        execution_environment,
        forms
    )
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    super::_lib::infect_special_form(interpreter, "let*", let_star)
}

// todo: simplify tests somehow by using tests of `let'
#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::error::assertion;
    use crate::interpreter::stdlib::special_forms;

    #[test]
    fn test_returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(Value::Integer(1), interpreter.execute("(let* () 3 2 1)").unwrap());
    }

    #[test]
    fn test_sets_symbol_with_executed_value() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();
        special_forms::quote::infect(&mut interpreter).unwrap();

        let definitions = vec!(
            (Value::Integer(1), "1"),
            (Value::Float(1.1), "1.1"),
            (Value::Boolean(true), "#t"),
            (Value::Boolean(false), "#f"),
            (interpreter.intern("symbol"), "'symbol"),
            (interpreter.intern("symbol"), "(quote symbol)"),
            (Value::String(String::from("string")), "\"string\""),
            (Value::Keyword(String::from("keyword")), ":keyword"),
            (Value::Cons(
                Cons::new(interpreter.intern("symbol"),
                          interpreter.intern_nil())),
             "'(symbol)"),
        );

        for (value, code_representation) in definitions {
            assert_eq!(
                value,
                interpreter.execute(
                    &format!("(let* ((value {})) value)", code_representation)
                ).unwrap()
            );
        }
    }


    #[test]
    fn test_sets_symbol_without_value_to_nil() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        assert_eq!(
            interpreter.intern_nil(),
            interpreter.execute("(let* (nil-symbol) nil-symbol)").unwrap()
        );
    }

    // the only difference between `let' `let*'
    #[test]
    fn test_able_to_use_previously_defined_values() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(let* ((sym-1 1) (sym-2 sym-1)) sym-2)").unwrap();

        assert_eq!(Value::Integer(1), result);
    }

    #[test]
    fn test_returns_error_when_first_argument_is_not_a_list() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(let* test)");

        assertion::assert_argument_error(&result);
        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn test_returns_error_when_first_argument_contains_not_a_symbol_nor_cons() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();


        let incorrect_strings = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword"
        );

        for incorrect_string in incorrect_strings {
            let result = interpreter.execute(
                &format!("(let* ({}))", incorrect_string)
            );

            assertion::assert_argument_error(&result);
            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn test_returns_error_when_first_part_of_definitions_is_not_a_symbol() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();


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

            assertion::assert_argument_error(&result);
            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn test_returns_error_when_first_symbol_of_a_definition_is_nil() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(let* ((nil 2)) nil)");

        assertion::assert_argument_error(&result);
        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn test_returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(let* ((sym)) nil)");

        assertion::assert_argument_error(&result);
        assertion::assert_invalid_argument_error(&result);

        let result = interpreter.execute("(let* ((sym 1 2)) nil)");

        assertion::assert_argument_error(&result);
        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn test_returns_err_when_attempt_to_redefine_already_defined_value() {
        let mut interpreter = Interpreter::raw();

        infect(&mut interpreter).unwrap();

        let result = interpreter.execute("(let* ((sym-1 1) (sym-1 2)) sym-1)");

        assert!(result.is_err())
    }
}