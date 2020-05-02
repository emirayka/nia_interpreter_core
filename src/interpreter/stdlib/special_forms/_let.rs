use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::value::SymbolId;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::ConsId;
use crate::interpreter::library;

fn set_variable_via_cons(
    interpreter: &mut Interpreter,
    definition_value_execution_environment: EnvironmentId,
    definition_setting_environment: EnvironmentId,
    cons_id: ConsId,
) -> Result<(), Error> {
    let car = interpreter.get_car(cons_id)
        .map_err(|err| Error::generic_execution_error_caused(
            "",
            err,
        ))?;

    let variable_symbol_id = match car {
        Value::Symbol(symbol_id) => {
            library::check_if_symbol_assignable(interpreter, symbol_id)?;

            symbol_id
        }
        _ => return Error::invalid_argument_error(
            "The first element of lists in the first argument of the special form `let' must be a symbol."
        ).into()
    };

    let cadr = interpreter.get_cadr(cons_id)
        .map_err(|_| Error::invalid_argument_error(
            "The definitions of the special form `let' must have exactly two arguments."
        ))?;

    let value = interpreter.execute_value(definition_value_execution_environment, cadr)
        .map_err(|err| Error::generic_execution_error_caused(
            "The definitions of the special form `let' must have exactly two arguments.",
            err,
        ))?;

    match interpreter.get_cddr(cons_id) {
        Ok(Value::Symbol(symbol_id)) => {
            if interpreter.symbol_is_not_nil(symbol_id)? {
                return Error::invalid_argument_error(
                    "The definitions of the special form `let' must have exactly two arguments."
                ).into();
            }
        }
        _ => return Error::invalid_argument_error(
            "The definitions of the special form `let' must have exactly two arguments."
        ).into()
    };

    interpreter.define_variable(
        definition_setting_environment,
        variable_symbol_id,
        value,
    )
}

fn set_variable_to_nil(
    interpreter: &mut Interpreter,
    definition_setting_environment: EnvironmentId,
    symbol_id: SymbolId,
) -> Result<(), Error> {
    library::check_if_symbol_assignable(interpreter, symbol_id)?;

    let nil = interpreter.intern_nil_symbol_value();

    interpreter.define_variable(definition_setting_environment, symbol_id, nil)
}

fn set_definition(
    interpreter: &mut Interpreter,
    definition_value_execution_environment: EnvironmentId,
    definition_setting_environment: EnvironmentId,
    definition: Value,
) -> Result<(), Error> {
    match definition {
        Value::Cons(cons_id) => set_variable_via_cons(
            interpreter,
            definition_value_execution_environment,
            definition_setting_environment,
            cons_id,
        ),
        Value::Symbol(symbol_id) => {
            if interpreter.symbol_is_nil(symbol_id)? {
                return Error::invalid_argument_error(
                    "It's not possible to redefine `nil' via special form `let'."
                ).into();
            } else {
                set_variable_to_nil(
                    interpreter,
                    definition_setting_environment,
                    symbol_id,
                )
            }
        }
        _ => return Error::invalid_argument_error(
            "Invalid `let*' definitions."
        ).into()
    }
}

pub fn set_definitions(
    interpreter: &mut Interpreter,
    definition_value_execution_environment: EnvironmentId,
    definition_setting_environment: EnvironmentId,
    definitions: Vec<Value>,
) -> Result<(), Error> {
    for definition in definitions {
        set_definition(
            interpreter,
            definition_value_execution_environment,
            definition_setting_environment,
            definition,
        )?;
    }

    Ok(())
}

pub fn _let(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Error::invalid_argument_count_error(
            "Special form let must have at least one argument."
        ).into();
    }

    let mut values = values;

    let definitions = library::read_as_let_definitions(
        interpreter,
        values.remove(0),
    ).map_err(|_| Error::invalid_argument_error(
        "The first argument of special form `let' must be a list of definitions: symbol, or 2-element lists."
    ))?;

    let forms = values;
    let execution_environment = interpreter.make_environment(environment)
        .map_err(|err| Error::generic_execution_error_caused(
            "",
            err,
        ))?;

    set_definitions(
        interpreter,
        environment,
        execution_environment,
        definitions,
    )?;

    library::execute_forms(
        interpreter,
        execution_environment,
        &forms,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::{for_constants, for_special_symbols};

    #[test]
    fn sets_symbol_with_executed_value() {
        let mut interpreter = Interpreter::new();

        let symbol = interpreter.intern_symbol_value("symbol");
        let nil = interpreter.intern_nil_symbol_value();

        let specs = vec!(
            (Value::Integer(1), "1"),
            (Value::Float(1.1), "1.1"),
            (Value::Boolean(true), "#t"),
            (Value::Boolean(false), "#f"),
            (interpreter.intern_symbol_value("symbol"), "'symbol"),
            (interpreter.intern_symbol_value("symbol"), "(quote symbol)"),
            (interpreter.intern_string_value("string"), "\"string\""),
            (interpreter.intern_keyword_value("keyword"), ":keyword"),
            (interpreter.make_cons_value(symbol, nil), "'(symbol)"),
        );

        for (value, string) in specs {
            let result = interpreter.execute(
                &format!("(let ((value {})) value)", string)
            ).unwrap();

            assertion::assert_deep_equal(
                &mut interpreter,
                value,
                result,
            );
        }
    }


    #[test]
    fn sets_symbol_without_value_to_nil() {
        let mut interpreter = Interpreter::new();

        nia_assert_equal(
            interpreter.intern_nil_symbol_value(),
            interpreter.execute("(let (nil-symbol) nil-symbol)").unwrap()
        );
    }

    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            (Value::Integer(1), "(let ((a 1)) a)"),
            (Value::Integer(2), "(let ((a 1)) (let ((a 2) (b 3)) a))"),
            (Value::Integer(3), "(let ((a 1)) (let ((a 2) (b 3)) b))"),
        );

        for (expected, code) in specs {
            let result = interpreter.execute(code).unwrap();

            assertion::assert_deep_equal(
                &mut interpreter,
                expected,
                result
            );
        }
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(let (({} 2)) {})", constant, constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(let (({} 2)) {})", special_symbol, special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_error_when_definition_is_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(let ({}) {})", constant, constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(let ({}) {})", special_symbol, special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_error_when_first_argument_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
        );

        for spec in specs {
            let result = interpreter.execute(&format!(
                "(let {})",
                spec
            ));

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_argument_contains_not_a_symbol_nor_cons() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
            "()",
            "nil",
        );

        for spec in specs {
            let result = interpreter.execute(
                &format!("(let ({}))", spec)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_part_of_definitions_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
            "(quote symbol)",
        );

        for spec in specs {
            let result = interpreter.execute(
                &format!("(let (({} 2)) {})", spec, spec)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_nil() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(let ((nil 2)) nil)"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            "(let ((sym)) nil)",
            "(let ((sym 1 2)) nil)"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn returns_err_when_attempt_to_use_previously_defined_values() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((sym-1 1) (sym-2 sym-1)) sym-2)");

        nia_assert(result.is_err())
    }

    #[test]
    fn returns_err_when_attempt_to_redefine_already_defined_value() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(let ((sym-1 1) (sym-1 2)) sym-1)");

        nia_assert(result.is_err())
    }
}
