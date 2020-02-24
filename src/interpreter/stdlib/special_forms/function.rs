use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::function::interpreted_function::InterpretedFunction;
use crate::interpreter::function::macro_function::MacroFunction;
use crate::interpreter::environment::environment_arena::EnvironmentId;

const ERROR_MESSAGE_INCORRECT_ARGUMENT: &'static str =
    "The first argument of special form `function', must be a list of signature \
 (lambda|macro ([arguments]) form1 form2 ...).";

fn parse_argument_names(interpreter: &mut Interpreter, values: Vec<Value>) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();

    for value in values {
        match value {
            Value::Symbol(symbol_id) => {
                let symbol = match interpreter.get_symbol(symbol_id) {
                    Ok(symbol) => symbol,
                    Err(error) => return interpreter.make_generic_execution_error_caused(
                        "",
                        error
                    ).into_result()
                };

                result.push(symbol.get_name().clone())
            }
            _ => return interpreter.make_generic_execution_error("").into_result()
        }
    }

    Ok(result)
}

fn construct_interpreted_function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    argument_names: Vec<String>,
    code: Vec<Value>
) -> Value {
    let function = Function::Interpreted(InterpretedFunction::new(
        environment,
        argument_names,
        code
    ));

    let function_id = interpreter.register_function(function);

    Value::Function(function_id)
}

fn construct_macro_function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    argument_names: Vec<String>,
    code: Vec<Value>
) -> Value {
    let function = Function::Macro(MacroFunction::new(
        environment,
        argument_names,
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
    let arguments = match values.remove(0) {
        Value::Cons(cons_id) => interpreter.cons_to_vec(cons_id),
        Value::Symbol(symbol_id) => {
            let symbol = match interpreter.get_symbol(symbol_id) {
                Ok(symbol) => symbol,
                Err(error) => return interpreter.make_generic_execution_error_caused(
                    "",
                    error
                ).into_result()
            };

            if symbol.is_nil() {
                Ok(Vec::new())
            } else {
                return interpreter.make_invalid_argument_error(
                    "The second element of first argument must be a list of symbols that represents argument names"
                ).into_result()
            }
        },
        _ => return interpreter.make_invalid_argument_error(
            "The second element of first argument must be a list of symbols that represents argument names"
        ).into_result()
    };

    let arguments = arguments.map_err(|err| interpreter.make_generic_execution_error_caused(
        "",
        err
    ))?;

    let code = values;

    let argument_names = parse_argument_names(interpreter, arguments)
        .map_err(|_| interpreter.make_invalid_argument_error(
            "The second element of the first argument must be a list of symbols that represents argument names"
        ))?;

    match lambda_or_macro_symbol {
        Value::Symbol(symbol_id) => {
            let name = interpreter.get_symbol_name(symbol_id)?;

            if name == "lambda" {
                Ok(construct_interpreted_function(
                    interpreter,
                    environment,
                    argument_names,
                    code
                ))
            } else if name == "macro" {
                Ok(construct_macro_function(
                    interpreter,
                    environment,
                    argument_names,
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

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            vec!(
                String::from("first-arg"),
                String::from("second-arg"),
            ),
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
        assert!(result.is_ok());

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    // todo: ensure this test is fine
    #[test]
    fn constructs_simple_macro() {
        let mut interpreter = Interpreter::new();

        let function = Function::Macro(MacroFunction::new(
            interpreter.get_root_environment(),
            vec!(
                String::from("first-arg"),
                String::from("second-arg"),
            ),
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
        assert!(result.is_ok());

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_correct_function_when_no_argument_was_provided() {
        let mut interpreter = Interpreter::new();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            vec!(),
            vec!(
                Value::Integer(1)
            )
        ));

        let function_id = interpreter.register_function( function);

        let expected = Value::Function(function_id);

        let result = interpreter.execute(
            "(function (lambda () 1))"
        );
        assert!(result.is_ok());

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    // todo: ensure this test is fine
    #[test]
    fn returns_correct_macro_when_no_argument_was_provided() {
        let mut interpreter = Interpreter::new();

        let function = Function::Macro(MacroFunction::new(
            interpreter.get_root_environment(),
            vec!(),
            vec!(
                Value::Integer(1)
            )
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);

        let result = interpreter.execute(
            "(function (macro () 1))"
        );
        assert!(result.is_ok());

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

        let not_valid_first_arguments = vec!(
            "1",
            "1.1",
            "#t",
            "#f",
            "symbol",
            "\"string\"",
            ":keyword",
        );

        for not_valid_first_argument in not_valid_first_arguments {
            let result = interpreter.execute(
                &format!("(function {})", not_valid_first_argument)
            );

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
