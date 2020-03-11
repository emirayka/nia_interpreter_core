use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::function::MacroFunction;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::cons::ConsId;
use crate::interpreter::library;

fn set_macro_via_cons(
    interpreter: &mut Interpreter,
    macro_parent_environment: EnvironmentId,
    macro_definition_environment: EnvironmentId,
    cons_id: ConsId
) -> Result<(), Error> {
    let car = interpreter.get_car(cons_id)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    let function_symbol_id = match car {
        Value::Symbol(symbol_id) => {
            let symbol = interpreter.get_symbol(symbol_id)?;

            if symbol.is_nil() {
                return interpreter.make_invalid_argument_error(
                    "It's not possible to redefine `nil' via special form `mlet'."
                ).into_result()
            } else {
                symbol_id
            }
        },
        _ => return interpreter.make_invalid_argument_error(
            "The first element of lists in the first argument of the special form `let' must be a symbol that represents macro name."
        ).into_result()
    };

    library::check_if_symbol_assignable(interpreter, function_symbol_id)?;

    let cadr = interpreter.get_cadr(cons_id)
        .map_err(|_| interpreter.make_invalid_argument_error(
            "The macro definitions of the special form `mlet' must have at least two items."
        ))?;

    let arguments = library::parse_arguments_from_value(interpreter, cadr)?;

    let code = match interpreter.get_cddr(cons_id) {
        Ok(Value::Cons(cons_id)) => interpreter.list_to_vec(cons_id),
        Ok(Value::Symbol(symbol_id)) => {
            let symbol = interpreter.get_symbol(symbol_id)?;

            if symbol.is_nil() {
                Ok(Vec::new())
            } else {
                return interpreter.make_invalid_argument_error(
                    "The macro definitions of the special form `mlet' must have at least two items.",
                ).into_result()
            }
        },
        _ => return interpreter.make_invalid_argument_error(
            "The macro definitions of the special form `mlet' must have at least two items.",
        ).into_result()
    };

    let code = code.map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    let function = Function::Macro(MacroFunction::new(
        macro_parent_environment,
        arguments,
        code
    ));

    let function_id = interpreter.register_function(function);
    let function_value = Value::Function(function_id);

    interpreter.define_function(
        macro_definition_environment,
        function_symbol_id,
        function_value
    )
}

fn set_definition(
    interpreter: &mut Interpreter,
    macro_parent_environment: EnvironmentId,
    macro_definition_environment: EnvironmentId,
    definition: Value
) -> Result<(), Error> {
    match definition {
        Value::Cons(cons) => set_macro_via_cons(
            interpreter,
            macro_parent_environment,
            macro_definition_environment,
            cons
        ),
        _ => return interpreter.make_invalid_argument_error(
            "The first argument of special form `mlet' must be a list of lists that represent macros."
        ).into_result()
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
            definition
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
        return interpreter.make_invalid_argument_count_error(
            "Special form mlet must have at least one argument."
        ).into_result();
    }

    let mut values = values;

    let definitions = library::read_let_definitions(
        interpreter,
        values.remove(0)
    ).map_err(|_| interpreter.make_invalid_argument_error(
        "Special form `mlet' must have a first argument of macro definitions"
    ))?;

    let forms = values;
    let macro_definition_environment = interpreter.make_environment(
        special_form_calling_environment
    ).map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    set_definitions(
        interpreter,
        special_form_calling_environment,
        macro_definition_environment,
        definitions
    )?;

    library::execute_forms(
        interpreter,
        macro_definition_environment,
        &forms
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::{
        for_constants,
        for_special_symbols
    };

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(mlet () 3)", Value::Integer(3)),
            ("(mlet () 3 2)", Value::Integer(2)),
            ("(mlet () 3 2 1)", Value::Integer(1)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn able_to_execute_defined_macros() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(mlet ((test-macro () 1)) (test-macro))", Value::Integer(1)),
            ("(mlet ((test-macro (a) a)) (test-macro 2))", Value::Integer(2))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn able_to_define_several_macros() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(mlet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-1))", Value::Integer(1)),
            ("(mlet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-2))", Value::Integer(2)),
            ("(mlet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-3))", Value::Integer(3)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn makes_a_correct_macro() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(mlet ((test-macro-1 () (quote (quote test)))) (test-macro-1))", "'test"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn possible_to_use_rest_opt_key_arguments() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(mlet ((a (#rest a) (list 'quote a))) (a))",                  "(list)"),
            ("(mlet ((a (#rest a) (list 'quote a))) (a 1))",                "(list 1)"),
            ("(mlet ((a (#rest a) (list 'quote a))) (a 1 2))",              "(list 1 2)"),

            ("(mlet ((a (#opt a) (list 'list a))) (a))",                    "(list nil)"),
            ("(mlet ((a (#opt (a 1)) (list 'list a))) (a))",                "(list 1)"),
            ("(mlet ((a (#opt (a 1 a?)) (list 'list a a?))) (a))",          "(list 1 #f)"),

            ("(mlet ((a (#opt a) (list 'list a))) (a 2))",                  "(list 2)"),
            ("(mlet ((a (#opt (a 1)) (list 'list a))) (a 2))",              "(list 2)"),
            ("(mlet ((a (#opt (a 1 a?)) (list 'list a a?))) (a 2))",        "(list 2 #t)"),

            ("(mlet ((a (#keys a) (list 'list a))) (a))",                   "(list nil)"),
            ("(mlet ((a (#keys (a 1)) (list 'list a))) (a))",               "(list 1)"),
            ("(mlet ((a (#keys (a 1 a?)) (list 'list a a?))) (a))",         "(list 1 #f)"),

            ("(mlet ((a (#keys a) (list 'list a))) (a :a 2))",              "(list 2)"),
            ("(mlet ((a (#keys (a 1)) (list 'list a))) (a :a 2))",          "(list 2)"),
            ("(mlet ((a (#keys (a 1 a?)) (list 'list a a?))) (a :a 2))",    "(list 2 #t)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(mlet ((a () 1)) (a))", Value::Integer(1)),
            ("(mlet ((a () 1)) (mlet ((a () 2) (b () 3)) (a)))", Value::Integer(2)),
            ("(mlet ((a () 1)) (mlet ((a () 2) (b () 3)) (b)))", Value::Integer(3)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(mlet (({} () 2)) {})", constant, constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(mlet (({} () 2)) {})", special_symbol, special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_error_when_first_argument_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let incorrect_strings = vec!(
            "(mlet 1)",
            "(mlet 1.1)",
            "(mlet #t)",
            "(mlet #f)",
            "(mlet 'symbol)",
            "(mlet \"string\")",
            "(mlet :keyword)",
            "(mlet {})",
            "(mlet #(+ %1 %2))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            incorrect_strings
        );
    }

    #[test]
    fn returns_error_when_first_argument_contains_not_a_symbol_nor_cons() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(mlet (1))",
            "(mlet (1.1))",
            "(mlet (#t))",
            "(mlet (#f))",
            "(mlet (\"string\"))",
            "(mlet (:keyword))",
            "(mlet ({}))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_error_when_first_part_of_macro_definition_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(mlet ((1 () 2)) 1)",
            "(mlet ((1.1 () 2)) 1.1)",
            "(mlet ((#t () 2)) #t)",
            "(mlet ((#f () 2)) #f)",
            "(mlet ((\"string\" () 2)) \"string\")",
            "(mlet ((:keyword () 2)) :keyword)",
            "(mlet (((quote symbol) () 2)) (quote symbol))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_error_when_arguments_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(mlet ((func 1 2)) (func))",
            "(mlet ((func 1.1 2)) (func))",
            "(mlet ((func #t 2)) (func))",
            "(mlet ((func #f 2)) (func))",
            "(mlet ((func \"string\" 2)) (func))",
            "(mlet ((func :keyword 2)) (func))",
            "(mlet ((func some-symbol 2)) (func))",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(mlet ((sym)) nil)",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_err_when_attempts_to_use_previously_defined_macros() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(mlet ((sym-1 () 1) (sym-2 () (sym-1))) (sym-2))"
        );

        assertion::assert_results_are_just_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_err_when_attempts_to_redefine_already_defined_macro() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(mlet ((sym-1 () 1) (sym-1 () 2)) (sym-1))"
        );

        assertion::assert_results_are_just_errors(
            &mut interpreter,
            code_vector
        );
    }
}
