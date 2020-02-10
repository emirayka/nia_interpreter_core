use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::function::macro_function::MacroFunction;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::cons::cons_arena::ConsId;

fn set_macro_via_cons(
    interpreter: &mut Interpreter,
    macro_parent_environment: EnvironmentId,
    macro_definition_environment: EnvironmentId,
    cons_id: &ConsId
) -> Result<(), Error> {
    let car = interpreter.get_car(cons_id).clone();
    let name = match car {
        Value::Symbol(symbol) if symbol.is_nil() => return Err(Error::invalid_argument(
            interpreter,
            "It's not possible to redefine `nil' via special form `mlet'."
        )),
        Value::Symbol(symbol) => {
            symbol
        },
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first element of lists in the first argument of the special form `let' must be a symbol that represents macro name."
        ))
    };

    let cadr = interpreter.get_cadr(cons_id);
    let arguments = match cadr {
        Ok(value) => value,
        Err(_) => return Err(Error::invalid_argument(
            interpreter,
            "The macro definitions of the special form `mlet' must have at least two items."
        ))
    };

    let arguments = match arguments {
        Value::Cons(cons_id) => interpreter.cons_to_vec(&cons_id),
        Value::Symbol(symbol) if symbol.is_nil() => Vec::new(),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The macro definitions of the special form `mlet' must have at least two items."
        ))
    };

    let argument_names = match super::_lib::convert_vector_of_values_to_vector_of_symbol_names(
        arguments
    ) {
        Ok(names) => names,
        Err(_) => return Err(Error::invalid_argument(
            interpreter,
            "The second item of a macro definition must be a list of symbols."
        ))
    };

    let code = match interpreter.get_cddr(cons_id) {
        Ok(Value::Cons(cons_id)) => interpreter.cons_to_vec(&cons_id),
        Ok(Value::Symbol(symbol)) if symbol.is_nil() => Vec::new(),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The macro definitions of the special form `mlet' must have at least two items.",
        ))
    };

    interpreter.define_function(
        macro_definition_environment,
        &name,
        Value::Function(Function::Macro(MacroFunction::new(
            macro_parent_environment,
            argument_names,
            code
        )))
    )
}

fn set_definition(
    interpreter: &mut Interpreter,
    macro_parent_environment: EnvironmentId,
    macro_definition_environment: EnvironmentId,
    definition: &Value
) -> Result<(), Error> {
    match definition {
        Value::Cons(cons) => set_macro_via_cons(
            interpreter,
            macro_parent_environment,
            macro_definition_environment,
            &cons
        ),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first argument of special form `mlet' must be a list of lists that represent macros."
        ))
    }
}

pub fn set_definitions(
    interpreter: &mut Interpreter,
    special_form_calling_environment: EnvironmentId,
    macro_definition_environment: EnvironmentId,
    definitions: Vec<Value>
) -> Result<(), Error> {
    for definition in definitions {
        set_definition(
            interpreter,
            special_form_calling_environment,
            macro_definition_environment,
            &definition
        )?;
    }

    Ok(())
}

pub fn mlet(
    interpreter: &mut Interpreter,
    special_form_calling_environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Special form mlet must have at least one argument."
        ));
    }

    let mut values = values;

    let definitions = match super::_lib::read_let_definitions(
        interpreter,
        values.remove(0)
    ) {
        Ok(values) => values,
        Err(_) => return Err(Error::invalid_argument(
            interpreter,
            "Special form `mlet' must have a first argument of macro definitions"
        ))
    };
    let forms = values;
    let macro_definition_environment = interpreter.make_environment(
        special_form_calling_environment
    );

    set_definitions(
        interpreter,
        special_form_calling_environment,
        macro_definition_environment,
        definitions
    )?;

    super::_lib::execute_forms(
        interpreter,
        macro_definition_environment,
        forms
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        assert_eq!(Value::Integer(3), interpreter.execute("(mlet () 3)").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(mlet () 3 2)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(mlet () 3 2 1)").unwrap());
    }

    #[test]
    fn able_to_execute_defined_macros() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(mlet ((test-macro () 1)) (test-macro))").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(mlet ((test-macro (a) a)) (test-macro 2))").unwrap()
        );
    }

    #[test]
    fn able_to_define_several_macros() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute(
                "(mlet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-1))"
            ).unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute(
                "(mlet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-2))"
            ).unwrap()
        );

        assert_eq!(
            Value::Integer(3),
            interpreter.execute(
                "(mlet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-3))"
            ).unwrap()
        );
    }

    #[test]
    fn makes_a_correct_macro() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute(
            "(mlet ((test-macro-1 () (quote (quote test)))) (test-macro-1))"
        );

        assert_eq!(interpreter.intern("test"), result.unwrap());
    }

    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(mlet ((a () 1)) (a))").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(mlet ((a () 1)) (mlet ((a () 2) (b () 3)) (a)))").unwrap()
        );

        assert_eq!(
            Value::Integer(3),
            interpreter.execute("(mlet ((a () 1)) (mlet ((a () 2) (b () 3)) (b)))").unwrap()
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
                "(mlet {})",
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
                &format!("(mlet ({}))", incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_part_of_macro_definition_is_not_a_symbol() {
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
                &format!("(mlet (({} () 2)) {})", incorrect_string, incorrect_string)
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
                &format!("(mlet ((func {} 2)) (func))", incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_nil() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(mlet ((nil () 2)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(mlet ((sym)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_err_when_attempts_to_use_previously_defined_macros() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(mlet ((sym-1 () 1) (sym-2 () (sym-1))) (sym-2))");

        assertion::assert_error(&result);
    }

    #[test]
    fn returns_err_when_attempts_to_redefine_already_defined_macro() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(mlet ((sym-1 () 1) (sym-1 () 2)) (sym-1))");

        assertion::assert_error(&result);
    }
}
