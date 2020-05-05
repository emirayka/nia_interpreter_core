use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Function;
use crate::interpreter::value::FunctionArguments;
use crate::interpreter::value::InterpretedFunction;
use crate::interpreter::value::MacroFunction;
use crate::interpreter::value::Value;

const ERROR_MESSAGE_INCORRECT_ARGUMENT: &'static str = "The first argument of special form `function', must be a list of signature \
 (lambda|macro ([arguments]) form1 form2 ...).";

fn construct_interpreted_function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    arguments: FunctionArguments,
    code: Vec<Value>,
) -> Value {
    let function = Function::Interpreted(InterpretedFunction::new(
        environment,
        arguments,
        code,
    ));

    let function_id = interpreter.register_function(function);

    Value::Function(function_id)
}

fn construct_macro_function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    arguments: FunctionArguments,
    code: Vec<Value>,
) -> Value {
    let function =
        Function::Macro(MacroFunction::new(environment, arguments, code));

    let function_id = interpreter.register_function(function);

    Value::Function(function_id)
}

pub fn function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Special form `function' must be called with exactly one argument.",
        )
        .into();
    }

    let mut values = match values.remove(0) {
        Value::Cons(cons_id) => interpreter.list_to_vec(cons_id),
        _ => {
            return Error::invalid_argument_error(
                ERROR_MESSAGE_INCORRECT_ARGUMENT,
            )
            .into();
        },
    }
    .map_err(|err| {
        Error::generic_execution_error_caused(
            "Cannot execute function special form",
            err,
        )
    })?;

    if values.len() < 3 {
        return Error::invalid_argument_error(ERROR_MESSAGE_INCORRECT_ARGUMENT)
            .into();
    }

    let lambda_or_macro_symbol = values.remove(0);

    let arguments = library::read_as_arguments(interpreter, values.remove(0))?;

    let code = values;

    match lambda_or_macro_symbol {
        Value::Symbol(symbol_id) => {
            let name = interpreter.get_symbol_name(symbol_id)?;

            if name == "lambda" {
                Ok(construct_interpreted_function(
                    interpreter,
                    environment,
                    arguments,
                    code,
                ))
            } else if name == "macro" {
                Ok(construct_macro_function(
                    interpreter,
                    environment,
                    arguments,
                    code,
                ))
            } else {
                Error::invalid_argument_error(
                    "The first element of the first argument must be a symbol `lambda' or `macro'",
                )
                .into()
            }
        }
        _ => Error::invalid_argument_error(
            "The first element of the first argument must be a symbol `lambda' or `macro'",
        )
        .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn constructs_simple_function() {
        let mut interpreter = Interpreter::new();
        let mut arguments = FunctionArguments::new();

        arguments
            .add_ordinary_argument(String::from("first-arg"))
            .unwrap();
        arguments
            .add_ordinary_argument(String::from("second-arg"))
            .unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![
                interpreter.intern_symbol_value("first-arg"),
                interpreter.intern_symbol_value("second-arg"),
            ],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute_in_main_environment(
            "(function (lambda (first-arg second-arg) first-arg second-arg))",
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn constructs_simple_macro() {
        let mut interpreter = Interpreter::new();
        let mut arguments = FunctionArguments::new();

        arguments
            .add_ordinary_argument(String::from("first-arg"))
            .unwrap();
        arguments
            .add_ordinary_argument(String::from("second-arg"))
            .unwrap();

        let function = Function::Macro(MacroFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![
                interpreter.intern_symbol_value("first-arg"),
                interpreter.intern_symbol_value("second-arg"),
            ],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute_in_main_environment(
            "(function (macro (first-arg second-arg) first-arg second-arg))",
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn returns_correct_function_when_no_argument_was_provided() {
        let mut interpreter = Interpreter::new();
        let arguments = FunctionArguments::new();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![Value::Integer(1)],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result =
            interpreter.execute_in_main_environment("(function (lambda () 1))");

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn returns_correct_macro_when_no_argument_was_provided() {
        let mut interpreter = Interpreter::new();
        let arguments = FunctionArguments::new();

        let function = Function::Macro(MacroFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![Value::Integer(1)],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result =
            interpreter.execute_in_main_environment("(function (macro () 1))");

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn able_to_construct_functions_with_optional_arguments() {
        let mut interpreter = Interpreter::new();
        let mut arguments = FunctionArguments::new();

        arguments
            .add_optional_argument(String::from("a"), None, None)
            .unwrap();
        arguments
            .add_optional_argument(
                String::from("b"),
                Some(Value::Integer(1)),
                None,
            )
            .unwrap();
        arguments
            .add_optional_argument(
                String::from("c"),
                Some(Value::Integer(1)),
                Some(String::from("c-provided?")),
            )
            .unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![Value::Integer(1)],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute_in_main_environment(
            "(function (lambda (#opt a (b 1) (c 1 c-provided?)) 1))",
        );

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn able_to_construct_functions_with_rest_arguments() {
        let mut interpreter = Interpreter::new();
        let mut arguments = FunctionArguments::new();

        arguments.add_rest_argument(String::from("a")).unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![Value::Integer(1)],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result = interpreter
            .execute_in_main_environment("(function (lambda (#rest a) 1))");

        let result = result.unwrap();
        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn able_to_construct_functions_with_key_arguments() {
        let mut interpreter = Interpreter::new();
        let mut arguments = FunctionArguments::new();

        arguments
            .add_key_argument(String::from("a"), None, None)
            .unwrap();
        arguments
            .add_key_argument(String::from("b"), Some(Value::Integer(1)), None)
            .unwrap();
        arguments
            .add_key_argument(
                String::from("c"),
                Some(Value::Integer(1)),
                Some(String::from("c-provided?")),
            )
            .unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![Value::Integer(1)],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute_in_main_environment(
            "(function (lambda (#keys a (b 1) (c 1 c-provided?)) 1))",
        );

        let result = result.unwrap();

        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn able_to_construct_functions_with_rest_arguments_after_optional_arguments(
    ) {
        let mut interpreter = Interpreter::new();
        let mut arguments = FunctionArguments::new();

        arguments
            .add_optional_argument(String::from("a"), None, None)
            .unwrap();

        arguments
            .add_optional_argument(
                String::from("b"),
                Some(Value::Integer(1)),
                None,
            )
            .unwrap();

        arguments.add_rest_argument(String::from("c")).unwrap();

        let function = Function::Interpreted(InterpretedFunction::new(
            interpreter.get_main_environment_id(),
            arguments,
            vec![Value::Integer(1)],
        ));

        let function_id = interpreter.register_function(function);

        let expected = Value::Function(function_id);
        let result = interpreter.execute_in_main_environment(
            "(function (lambda (#opt a (b 1) #rest c) 1))",
        );

        let result = result.unwrap();

        assertion::assert_deep_equal(&mut interpreter, expected, result);
    }

    #[test]
    fn returns_error_when_incorrect_amount_of_arguments_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(function)", "(function 1 2)", "(function 1 2 3)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_error_when_not_a_cons_cell_were_provided() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(function 1)",
            "(function 1.1)",
            "(function #t)",
            "(function #f)",
            "(function symbol)",
            "(function \"string\")",
            "(function :keyword)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_error_when_first_element_is_not_lambda_nor_macro() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(function (special-form () 2))"];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_error_when_incorrect_amount_of_elements_of_first_argument_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(function (lambda))", "(function (lambda ()))"];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_error_when_incorrect_constructed_function_arguments_were_provided(
    ) {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(function (lambda 1 1))",
            "(function (lambda 1.1 1))",
            "(function (lambda #t 1))",
            "(function (lambda #f 1))",
            "(function (lambda \"string\" 1))",
            "(function (lambda :keyword 1))",
            "(function (lambda {} 1))",
            "(function (lambda (1) 1))",
            "(function (lambda (1.1) 1))",
            "(function (lambda (#t) 1))",
            "(function (lambda (#f) 1))",
            "(function (lambda (\"string\") 1))",
            "(function (lambda (:keyword) 1))",
            "(function (lambda ({}) 1))",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }
}
