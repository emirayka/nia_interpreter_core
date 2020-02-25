use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::function::interpreted_function::InterpretedFunction;
use crate::interpreter::function::macro_function::MacroFunction;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::function::arguments::Arguments;
use crate::interpreter::stdlib::_lib;

const ERROR_MESSAGE_INCORRECT_ARGUMENT: &'static str =
    "The first argument of special form `function', must be a list of signature \
 (lambda|macro ([arguments]) form1 form2 ...).";


fn construct_interpreted_function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    arguments: Arguments,
    code: Vec<Value>
) -> Value {
    let function = Function::Interpreted(InterpretedFunction::new(
        environment,
        arguments,
        code
    ));

    let function_id = interpreter.register_function(function);

    Value::Function(function_id)
}

fn construct_macro_function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    arguments: Arguments,
    code: Vec<Value>
) -> Value {
    let function = Function::Macro(MacroFunction::new(
        environment,
        arguments,
        code
    ));

    let function_id = interpreter.register_function(function);

    Value::Function(function_id)
}

pub fn function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Special form `function' must be called with exactly one argument."
        ).into_result();
    }

    let mut values = match values.remove(0) {
        Value::Cons(cons_id) => interpreter.cons_to_vec(cons_id),
        _ => return interpreter.make_invalid_argument_error(
            ERROR_MESSAGE_INCORRECT_ARGUMENT
        ).into_result()
    }.map_err(|err| interpreter.make_generic_execution_error_caused(
        "Cannot execute function special form",
        err
    ))?;

    if values.len() < 3 {
        return interpreter.make_invalid_argument_error(
            ERROR_MESSAGE_INCORRECT_ARGUMENT
        ).into_result();
    }

    let lambda_or_macro_symbol = values.remove(0);

    let arguments = _lib::parse_arguments_from_value(
        interpreter,
        values.remove(0)
    )?;

    let code = values;

    match lambda_or_macro_symbol {
        Value::Symbol(symbol_id) => {
            let name = interpreter.get_symbol_name(symbol_id)?;

            if name == "lambda" {
                Ok(construct_interpreted_function(
                    interpreter,
                    environment,
                    arguments,
                    code
                ))
            } else if name == "macro" {
                Ok(construct_macro_function(
                    interpreter,
                    environment,
                    arguments,
                    code
                ))
            } else {
                interpreter.make_invalid_argument_error(
                    "The first element of the first argument must be a symbol `lambda' or `macro'"
                ).into_result()
            }
        },
        _ => interpreter.make_invalid_argument_error(
            "The first element of the first argument must be a symbol `lambda' or `macro'"
        ).into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    // todo: ensure this test is fine
    #[test]
    fn constructs_simple_function() {
        let mut interpreter = Interpreter::new();
        let mut arguments = Arguments::new();

        arguments.add_ordinary_argument(String::from("first-arg")).unwrap();
        arguments.add_ordinary_argument(String::from("second-arg")).unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            arguments,
            vec!(
                interpreter.intern_symbol_value("first-arg"),
                interpreter.intern_symbol_value("second-arg")
            )
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);

        let result = interpreter.execute(
            "(function (lambda (first-arg second-arg) first-arg second-arg))"
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    // todo: ensure this test is fine
    #[test]
    fn constructs_simple_macro() {
        let mut interpreter = Interpreter::new();
        let mut arguments = Arguments::new();

        arguments.add_ordinary_argument(String::from("first-arg")).unwrap();
        arguments.add_ordinary_argument(String::from("second-arg")).unwrap();

        let function = Function::Macro(MacroFunction::new(
            interpreter.get_root_environment(),
            arguments,
            vec!(
                interpreter.intern_symbol_value("first-arg"),
                interpreter.intern_symbol_value("second-arg")
            )
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);

        let result = interpreter.execute(
            "(function (macro (first-arg second-arg) first-arg second-arg))"
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_correct_function_when_no_argument_was_provided() {
        let mut interpreter = Interpreter::new();
        let mut arguments = Arguments::new();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            arguments,
            vec!(
                Value::Integer(1)
            )
        ));

        let function_id = interpreter.register_function( function);

        let expected = Value::Function(function_id);

        let result = interpreter.execute(
            "(function (lambda () 1))"
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_correct_macro_when_no_argument_was_provided() {
        let mut interpreter = Interpreter::new();
        let mut arguments = Arguments::new();

        let function = Function::Macro(MacroFunction::new(
            interpreter.get_root_environment(),
            arguments,
            vec!(
                Value::Integer(1)
            )
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);

        let result = interpreter.execute(
            "(function (macro () 1))"
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn able_to_construct_functions_with_optional_arguments() {
        let mut interpreter = Interpreter::new();
        let mut arguments = Arguments::new();

        arguments.add_optional_argument(
            String::from("a"),
            None,
            None
        ).unwrap();
        arguments.add_optional_argument(
            String::from("b"),
            Some(Value::Integer(1)),
            None
        ).unwrap();
        arguments.add_optional_argument(
            String::from("c"),
            Some(Value::Integer(1)),
            Some(String::from("c-provided?")
            )).unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            arguments,
            vec!(
                Value::Integer(1)
            )
        ));

        let function_id = interpreter.register_function( function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute(
            "(function (lambda (#opt a (b 1) (c 1 c-provided?)) 1))"
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn able_to_construct_functions_with_rest_arguments() {
        let mut interpreter = Interpreter::new();
        let mut arguments = Arguments::new();

        arguments.add_rest_argument(
            String::from("a"),
        ).unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            arguments,
            vec!(
                Value::Integer(1)
            )
        ));

        let function_id = interpreter.register_function( function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute(
            "(function (lambda (#rest a) 1))"
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn able_to_construct_functions_with_key_arguments() {
        let mut interpreter = Interpreter::new();
        let mut arguments = Arguments::new();

        arguments.add_key_argument(
            String::from("a"),
            None,
            None
        ).unwrap();
        arguments.add_key_argument(
            String::from("b"),
            Some(Value::Integer(1)),
            None
        ).unwrap();
        arguments.add_key_argument(
            String::from("c"),
            Some(Value::Integer(1)),
            Some(String::from("c-provided?")
            )).unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            arguments,
            vec!(
                Value::Integer(1)
            )
        ));

        let function_id = interpreter.register_function( function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute(
            "(function (lambda (#keys a (b 1) (c 1 c-provided?)) 1))"
        );

        let result = result.unwrap();

        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn able_to_construct_functions_with_rest_arguments_after_optional_arguments() {
        let mut interpreter = Interpreter::new();
        let mut arguments = Arguments::new();

        arguments.add_optional_argument(
            String::from("a"),
            None,
            None
        ).unwrap();

        arguments.add_optional_argument(
            String::from("b"),
            Some(Value::Integer(1)),
            None
        ).unwrap();

        arguments.add_rest_argument(
            String::from("c"),
        ).unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            arguments,
            vec!(
                Value::Integer(1)
            )
        ));

        let function_id = interpreter.register_function( function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute(
            "(function (lambda (#opt a (b 1) #rest c) 1))"
        );

        let result = result.unwrap();

        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_incorrect_amount_of_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(function)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(function 1 2)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(function 1 2 3)");
        assertion::assert_invalid_argument_count_error(&result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_not_a_cons_cell_were_provided() {
        let mut interpreter = Interpreter::new();

        let not_valid_code = vec!(
            "(function 1)",
            "(function 1.1)",
            "(function #t)",
            "(function #f)",
            "(function symbol)",
            "(function \"string\")",
            "(function :keyword)",
        );

        for code in not_valid_code {
            let result = interpreter.execute(&code);

            assertion::assert_invalid_argument_error(&result);
        }
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_first_element_is_not_lambda_nor_macro() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(function (special-form () 2))");
        assertion::assert_invalid_argument_error(&result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_incorrect_amount_of_elements_of_first_argument_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(function (lambda))");
        assertion::assert_invalid_argument_error(&result);

        let result = interpreter.execute("(function (lambda ()))");
        assertion::assert_invalid_argument_error(&result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_error_when_incorrect_constructed_function_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let not_valid_arguments = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
        );

        for not_valid_argument in &not_valid_arguments {
            let result = interpreter.execute(&format!(
                "(function (lambda {} 1))",
                not_valid_argument
            ));

            assertion::assert_invalid_argument_error(&result);
        }

        for not_valid_argument in &not_valid_arguments {
            let result = interpreter.execute(&format!(
                "(function (lambda ({}) 1))",
                not_valid_argument
            ));

            assertion::assert_invalid_argument_error(&result);
        }

        for not_valid_argument_1 in &not_valid_arguments {
            for not_valid_argument_2 in &not_valid_arguments {
                let result = interpreter.execute(&format!(
                    "(function (lambda ({} {}) 1))",
                    not_valid_argument_1,
                    not_valid_argument_2
                ));

                assertion::assert_invalid_argument_error(&result);
            }
        }
    }


}
