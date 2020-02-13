use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::function::interpreted_function::InterpretedFunction;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::cons::cons_arena::ConsId;

fn set_function_via_cons(
    interpreter: &mut Interpreter,
    function_parent_environment: EnvironmentId,
    function_definition_environment: EnvironmentId,
    cons_id: &ConsId
) -> Result<(), Error> {
    let car = match interpreter.get_car(cons_id){
        Ok(value) => value.clone(),
        Err(error) => return interpreter.make_generic_execution_error_caused(
            "",
            error
        )
    };

    let name = match car {
        Value::Symbol(symbol) if symbol.is_nil() => return interpreter.make_invalid_argument_error(
            "It's not possible to redefine `nil' via special form `flet'."
        ),
        Value::Symbol(symbol) => {
            symbol
        },
        _ => return interpreter.make_invalid_argument_error(
            "The first element of lists in the first argument of the special form `flet' must be a symbol that represents function name."
        )
    };

    let cadr = interpreter.get_cadr(cons_id);
    let arguments = match cadr {
        Ok(value) => value.clone(),
        Err(_) => return interpreter.make_invalid_argument_error(
            "The function definitions of the special form `flet' must have at least two items."
        )
    };

    let arguments = match arguments {
        Value::Cons(cons_id) => interpreter.cons_to_vec(cons_id),
        Value::Symbol(symbol) if symbol.is_nil() => Ok(Vec::new()),
        _ => return interpreter.make_invalid_argument_error(
            "The function definitions of the special form `flet' must have at least two items."
        )
    };

    let arguments = match arguments {
        Ok(arguments) => arguments,
        Err(error) => return interpreter.make_generic_execution_error_caused(
            "",
            error
        )
    };

    let argument_names = match super::_lib::convert_vector_of_values_to_vector_of_symbol_names(
        arguments
    ) {
        Ok(names) => names,
        Err(_) => return interpreter.make_invalid_argument_error(
            "The second item of a function definition must be a list of symbols."
        )
    };

    let cddr = interpreter.get_cddr(cons_id);
    let code = match cddr {
        Ok(Value::Cons(cons_id)) => interpreter.cons_to_vec(cons_id),
        Ok(Value::Symbol(symbol)) if symbol.is_nil() => Ok(Vec::new()),
        _ => return interpreter.make_invalid_argument_error(
            "The function definitions of the special form `flet' must have at least two items.",
        )
    };

    let code = match code {
        Ok(code) => code,
        Err(error) => return interpreter.make_generic_execution_error_caused(
            "",
            error
        )
    };

    interpreter.define_function(
        function_definition_environment,
        &name,
        Value::Function(Function::Interpreted(InterpretedFunction::new(
            function_parent_environment,
            argument_names,
            code
        )))
    )
}

fn set_definition(
    interpreter: &mut Interpreter,
    function_parent_environment: EnvironmentId,
    function_definition_environment: EnvironmentId,
    definition: &Value
) -> Result<(), Error> {
    match definition {
        Value::Cons(cons) => set_function_via_cons(
            interpreter,
            function_parent_environment,
            function_definition_environment,
            &cons
        ),
        _ => return interpreter.make_invalid_argument_error(
            "The first argument of special form `flet' must be a list of lists that represent functions."
        )
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
            &definition
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
        );
    }

    let mut values = values;

    let definitions = match super::_lib::read_let_definitions(
        interpreter,
        values.remove(0)
    ) {
        Ok(values) => values,
        Err(_) => return interpreter.make_invalid_argument_error(
            "Special form `flet' must have a first argument of function definitions"
        )
    };
    let forms = values;
    let function_definition_environment = interpreter.make_environment(
        special_form_calling_environment
    );

    set_definitions(
        interpreter,
        special_form_calling_environment,
        function_definition_environment,
        definitions
    )?;

    super::_lib::execute_forms(
        interpreter,
        function_definition_environment,
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

        assert_eq!(Value::Integer(3), interpreter.execute("(flet () 3)").unwrap());
        assert_eq!(Value::Integer(2), interpreter.execute("(flet () 3 2)").unwrap());
        assert_eq!(Value::Integer(1), interpreter.execute("(flet () 3 2 1)").unwrap());
    }

    #[test]
    fn able_to_execute_defined_functions() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(flet ((test-func () 1)) (test-func))").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(flet ((test-func (a) a)) (test-func 2))").unwrap()
        );
    }

    #[test]
    fn able_to_define_several_functions() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute(
                 "(flet ((test-func-1 () 1) (test-func-2 () 2) (test-func-3 () 3)) (test-func-1))"
            ).unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute(
                "(flet ((test-func-1 () 1) (test-func-2 () 2) (test-func-3 () 3)) (test-func-2))"
            ).unwrap()
        );

        assert_eq!(
            Value::Integer(3),
            interpreter.execute(
                "(flet ((test-func-1 () 1) (test-func-2 () 2) (test-func-3 () 3)) (test-func-3))"
            ).unwrap()
        );
    }

    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        assert_eq!(
            Value::Integer(1),
            interpreter.execute("(flet ((a () 1)) (a))").unwrap()
        );

        assert_eq!(
            Value::Integer(2),
            interpreter.execute("(flet ((a () 1)) (flet ((a () 2) (b () 3)) (a)))").unwrap()
        );

        assert_eq!(
            Value::Integer(3),
            interpreter.execute("(flet ((a () 1)) (flet ((a () 2) (b () 3)) (b)))").unwrap()
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
                "(flet {})",
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
                &format!("(flet ({}))", incorrect_string)
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
                &format!("(flet (({} () 2)) {})", incorrect_string, incorrect_string)
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
                &format!("(flet ((func {} 2)) (func))", incorrect_string)
            );

            assertion::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_nil() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(flet ((nil () 2)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(flet ((sym)) nil)");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_err_when_attempts_to_use_previously_defined_functions() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(flet ((sym-1 () 1) (sym-2 () (sym-1))) (sym-2))");

        assertion::assert_error(&result);
    }

    #[test]
    fn returns_err_when_attempts_to_redefine_already_defined_function() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(flet ((sym-1 () 1) (sym-1 () 2)) (sym-1))");

        assertion::assert_error(&result);
    }
}
