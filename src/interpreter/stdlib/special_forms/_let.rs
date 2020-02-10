use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::stdlib::special_forms::_lib::infect_special_form;
use crate::interpreter::cons::Cons;
use crate::interpreter::symbol::Symbol;

fn set_variable_via_cons(
    interpreter: &mut Interpreter,
    definition_value_execution_environment: EnvironmentId,
    definition_setting_environment: EnvironmentId,
    cons: &Cons
) -> Result<(), Error> {
    let car = cons.get_car();
    let name = match car {
        Value::Symbol(symbol) if symbol.is_nil() => return Err(Error::invalid_argument(
            interpreter,
            "It's not possible to redefine `nil' via special form `let'."
        )),
        Value::Symbol(symbol) => {
            symbol
        },
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first element of lists in the first argument of the special form `let' must be a symbol."
        ))
    };

    let cadr = cons.get_cadr();
    let value = match cadr {
        Ok(value) => value,
        Err(_) => return Err(Error::invalid_argument(
            interpreter,
            "The definitions of the special form `let' must have exactly two arguments."
        ))
    };

    let value = match interpreter.execute_value(definition_value_execution_environment, value) {
        Ok(value) => value,
        Err(error) => return Err(Error::generic_execution_error_caused(
            interpreter,
            "The definitions of the special form `let' must have exactly two arguments.",
            error
        ))
    };

    match cons.get_cddr() {
        Ok(Value::Symbol(symbol)) if symbol.is_nil() => {},
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The definitions of the special form `let' must have exactly two arguments."
        ))
    };

    interpreter.define_variable(
        definition_setting_environment,
        name,
        value
    )
}

fn set_variable_to_nil(
    interpreter: &mut Interpreter,
    definition_setting_environment: EnvironmentId,
    symbol: &Symbol
) -> Result<(), Error> {
    let nil = interpreter.intern_nil();

    interpreter.define_variable(definition_setting_environment, symbol, nil)
}

fn set_definition(
    interpreter: &mut Interpreter,
    definition_value_execution_environment: EnvironmentId,
    definition_setting_environment: EnvironmentId,
    definition: &Value
) -> Result<(), Error> {
    match definition {
        Value::Cons(cons) => set_variable_via_cons(
            interpreter,
            definition_value_execution_environment,
            definition_setting_environment,
            &cons
        ),
        Value::Symbol(symbol) if symbol.is_nil() => return Err(Error::invalid_argument(
            interpreter,
            "It's not possible to redefine `nil' via special form `let'."
        )),
        Value::Symbol(symbol) => set_variable_to_nil(
            interpreter,
            definition_setting_environment,
            &symbol
        ),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first argument of special form `let' must be a list of symbols, or lists."
        ))
    }
}

pub fn set_definitions(
    interpreter: &mut Interpreter,
    definition_value_execution_environment: EnvironmentId,
    definition_setting_environment: EnvironmentId,
    definitions: Vec<Value>
) -> Result<(), Error> {
    for definition in definitions {
        set_definition(
            interpreter,
            definition_value_execution_environment,
            definition_setting_environment,
            &definition
        )?;
    }

    Ok(())
}

fn _let(
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

    let definitions = match super::_lib::read_let_definitions(
        interpreter,
        values.remove(0)
    ) {
        Ok(definitions) => definitions,
        Err(_) => return Err(Error::invalid_argument(
            interpreter,
            "The first argument of special form `let' must be a list of definitions: symbol, or 2-element lists."
        ))
    };
    let forms = values;
    let execution_environment = interpreter.make_environment(environment);

    set_definitions(
        interpreter,
        environment,
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
    infect_special_form(interpreter, "let", _let)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn sets_symbol_with_executed_value() {
        let mut interpreter = Interpreter::new();

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
                    &format!("(let ((value {})) value)", code_representation)
                ).unwrap()
            );
        }
    }


    #[test]
    fn sets_symbol_without_value_to_nil() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            interpreter.intern_nil(),
            interpreter.execute("(let (nil-symbol) nil-symbol)").unwrap()
        );
    }

    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(let ((a 1)) a)").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(let ((a 1)) (let ((a 2) (b 3)) a))").unwrap()
        );

        assert_eq!(
            Value::Integer(3),
            interpreter.execute("(let ((a 1)) (let ((a 2) (b 3)) b))").unwrap()
        );
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
                "(let {})",
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
                &format!("(let ({}))", incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

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
                &format!("(let (({} 2)) {})", incorrect_string, incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_nil() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((nil 2)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((sym)) nil)");

        assertion::assert_invalid_argument_error(&result);

        let result = interpreter.execute("(let ((sym 1 2)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_err_when_attempt_to_use_previously_defined_values() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((sym-1 1) (sym-2 sym-1)) sym-2)");

        assert!(result.is_err())
    }

    #[test]
    fn returns_err_when_attempt_to_redefine_already_defined_value() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((sym-1 1) (sym-1 2)) sym-1)");

        assert!(result.is_err())
    }
}
