use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::function::InterpretedFunction;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::cons::ConsId;
use crate::interpreter::library;

fn set_function_via_cons(
    interpreter: &mut Interpreter,
    function_parent_environment: EnvironmentId,
    function_definition_environment: EnvironmentId,
    cons_id: ConsId
) -> Result<(), Error> {
    let car = interpreter.get_car(cons_id)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    let function_symbol_id = match car {
        Value::Symbol(symbol_id)  => {
            let symbol = interpreter.get_symbol(symbol_id)?;

            if symbol.is_nil() {
                return interpreter.make_invalid_argument_error(
                    "It's not possible to redefine `nil' via special form `flet'."
                ).into_result()
            } else {
                symbol_id
            }
        },
        _ => return interpreter.make_invalid_argument_error(
            "The first element of lists in the first argument of the special form `flet' must be a symbol that represents function name."
        ).into_result()
    };

    library::check_if_symbol_assignable(interpreter, function_symbol_id)?;

    let cadr = interpreter.get_cadr(cons_id)
        .map_err(|_| interpreter.make_invalid_argument_error(
            "The function definitions of the special form `flet' must have at least two items."
        ))?;

    let arguments = library::parse_arguments_from_value(interpreter, cadr)?;

    let cddr = interpreter.get_cddr(cons_id)
        .map_err(|err| interpreter.make_generic_execution_error_caused(
            "",
            err
        ))?;

    let code = match cddr {
        Value::Cons(cons_id) => interpreter.list_to_vec(cons_id),
        Value::Symbol(symbol_id) => {
            let symbol = interpreter.get_symbol(symbol_id)?;

            if symbol.is_nil() {
                Ok(Vec::new())
            } else {
                return interpreter.make_invalid_argument_error(
                    "The function definitions of the special form `flet' must have at least two items.",
                ).into_result()
            }
        },
        _ => return interpreter.make_invalid_argument_error(
            "The function definitions of the special form `flet' must have at least two items.",
        ).into_result()
    };

    let code = code.map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    let function = Function::Interpreted(InterpretedFunction::new(
        function_parent_environment,
        arguments,
        code
    ));

    let function_id = interpreter.register_function(function);
    let function_value = Value::Function(function_id);

    interpreter.define_function(
        function_definition_environment,
        function_symbol_id,
        function_value
    )
}

fn set_definition(
    interpreter: &mut Interpreter,
    function_parent_environment: EnvironmentId,
    function_definition_environment: EnvironmentId,
    definition: Value
) -> Result<(), Error> {
    match definition {
        Value::Cons(cons_id) => set_function_via_cons(
            interpreter,
            function_parent_environment,
            function_definition_environment,
            cons_id
        ),
        _ => interpreter.make_invalid_argument_error(
            "The first argument of special form `flet' must be a list of lists that represent functions."
        ).into_result()
    }
}

pub fn set_definitions(
    interpreter: &mut Interpreter,
    special_form_calling_environment: EnvironmentId,
    function_definition_environment: EnvironmentId,
    definitions: Vec<Value>
) -> Result<(), Error> {
    for definition in definitions {
        set_definition(
            interpreter,
            special_form_calling_environment,
            function_definition_environment,
            definition
        )?;
    }

    Ok(())
}

pub fn flet(
    interpreter: &mut Interpreter,
    special_form_calling_environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() == 0 {
        return interpreter.make_invalid_argument_count_error(
            "Special form flet must have at least one argument."
        ).into_result();
    }

    let mut values = values;

    let definitions = library::read_let_definitions(
        interpreter,
        values.remove(0)
    ).map_err(|_| interpreter.make_invalid_argument_error(
        "Special form `flet' must have a first argument of function definitions"
    ))?;

    let forms = values;
    let function_definition_environment = interpreter.make_environment(
        special_form_calling_environment
    ).map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    set_definitions(
        interpreter,
        special_form_calling_environment,
        function_definition_environment,
        definitions
    )?;

    library::execute_forms(
        interpreter,
        function_definition_environment,
        forms
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
            ("(flet () 3)", Value::Integer(3)),
            ("(flet () 3 2)", Value::Integer(2)),
            ("(flet () 3 2 1)", Value::Integer(1)),
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
            ("(flet ((test-macro () 1)) (test-macro))", Value::Integer(1)),
            ("(flet ((test-macro (a) a)) (test-macro 2))", Value::Integer(2))
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
            ("(flet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-1))", Value::Integer(1)),
            ("(flet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-2))", Value::Integer(2)),
            ("(flet ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-3))", Value::Integer(3)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn possible_to_use_rest_opt_key_arguments() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(flet ((a (#rest a) a)) (a))",                          "(list)"),
            ("(flet ((a (#rest a) a)) (a 1))",                        "(list 1)"),
            ("(flet ((a (#rest a) a)) (a 1 2))",                      "(list 1 2)"),

            ("(flet ((a (#opt a) (list a))) (a))",                    "(list nil)"),
            ("(flet ((a (#opt (a 1)) (list a))) (a))",                "(list 1)"),
            ("(flet ((a (#opt (a 1 a?)) (list a a?))) (a))",          "(list 1 #f)"),

            ("(flet ((a (#opt a) (list a))) (a 2))",                  "(list 2)"),
            ("(flet ((a (#opt (a 1)) (list a))) (a 2))",              "(list 2)"),
            ("(flet ((a (#opt (a 1 a?)) (list a a?))) (a 2))",        "(list 2 #t)"),

            ("(flet ((a (#keys a) (list a))) (a))",                   "(list nil)"),
            ("(flet ((a (#keys (a 1)) (list a))) (a))",               "(list 1)"),
            ("(flet ((a (#keys (a 1 a?)) (list a a?))) (a))",         "(list 1 #f)"),

            ("(flet ((a (#keys a) (list a))) (a :a 2))",              "(list 2)"),
            ("(flet ((a (#keys (a 1)) (list a))) (a :a 2))",          "(list 2)"),
            ("(flet ((a (#keys (a 1 a?)) (list a a?))) (a :a 2))",    "(list 2 #t)"),
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
            ("(flet ((a () 1)) (a))", Value::Integer(1)),
            ("(flet ((a () 1)) (flet ((a () 2) (b () 3)) (a)))", Value::Integer(2)),
            ("(flet ((a () 1)) (flet ((a () 2) (b () 3)) (b)))", Value::Integer(3)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_constant_or_special_symbol() {
        for_constants(|interpreter, constant| {
            let code = &format!("(flet (({} () 2)) {})", constant, constant);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });

        for_special_symbols(|interpreter, special_symbol| {
            let code = &format!("(flet (({} () 2)) {})", special_symbol, special_symbol);
            let result = interpreter.execute(code);

            assertion::assert_invalid_argument_error(&result);
        });
    }

    #[test]
    fn returns_error_when_first_argument_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let incorrect_strings = vec!(
            "(flet 1)",
            "(flet 1.1)",
            "(flet #t)",
            "(flet #f)",
            "(flet 'symbol)",
            "(flet \"string\")",
            "(flet :keyword)",
            "(flet {})",
            "(flet #(+ %1 %2))",
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
            "(flet (1))",
            "(flet (1.1))",
            "(flet (#t))",
            "(flet (#f))",
            "(flet (\"string\"))",
            "(flet (:keyword))",
            "(flet ({}))",
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
            "(flet ((1 () 2)) 1)",
            "(flet ((1.1 () 2)) 1.1)",
            "(flet ((#t () 2)) #t)",
            "(flet ((#f () 2)) #f)",
            "(flet ((\"string\" () 2)) \"string\")",
            "(flet ((:keyword () 2)) :keyword)",
            "(flet (((quote symbol) () 2)) (quote symbol))",
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
            "(flet ((func 1 2)) (func))",
            "(flet ((func 1.1 2)) (func))",
            "(flet ((func #t 2)) (func))",
            "(flet ((func #f 2)) (func))",
            "(flet ((func \"string\" 2)) (func))",
            "(flet ((func :keyword 2)) (func))",
            "(flet ((func some-symbol 2)) (func))",
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
            "(flet ((sym)) nil)",
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
            "(flet ((sym-1 () 1) (sym-2 () (sym-1))) (sym-2))"
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
            "(flet ((sym-1 () 1) (sym-1 () 2)) (sym-1))"
        );

        assertion::assert_results_are_just_errors(
            &mut interpreter,
            code_vector
        );
    }
}
