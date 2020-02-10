use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::function::interpreted_function::InterpretedFunction;
use crate::interpreter::function::macro_function::MacroFunction;

const ERROR_MESSAGE_INCORRECT_ARGUMENT: &'static str =
    "The first argument of special form `function', must be a list of signature \
 (lambda|macro ([arguments]) form1 form2 ...).";

fn parse_argument_names(values: Vec<Value>) -> Result<Vec<String>, ()> {
    let mut result = Vec::new();

    for value in values {
        match value {
            Value::Symbol(symbol) => result.push(symbol.get_name().clone()),
            _ => return Err(())
        }
    }

    Ok(result)
}

fn construct_interpreted_function(
    environment: EnvironmentId,
    argument_names: Vec<String>,
    code: Vec<Value>
) -> Value {
    Value::Function(Function::Interpreted(InterpretedFunction::new(
        environment,
        argument_names,
        code
    )))
}

fn construct_macro_function(
    environment: EnvironmentId,
    argument_names: Vec<String>,
    code: Vec<Value>
) -> Value {
    Value::Function(Function::Macro(MacroFunction::new(
        environment,
        argument_names,
        code
    )))
}

pub fn function(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    let mut values = values;

    if values.len() != 1 {
        return Err(Error::invalid_argument_count(
            interpreter,
            "Special form `function' must be called with exactly one argument."
        ));
    }

    let mut values = match values.remove(0) {
        Value::Cons(cons) => cons.to_vec(),
        _ => return Err(Error::invalid_argument(
            interpreter,
            ERROR_MESSAGE_INCORRECT_ARGUMENT
        ))
    };

    if values.len() < 3 {
        return Err(Error::invalid_argument(
            interpreter,
            ERROR_MESSAGE_INCORRECT_ARGUMENT
        ));
    }

    let lambda_or_macro_symbol = values.remove(0);
    let arguments = match values.remove(0) {
        Value::Cons(cons) => cons.to_vec(),
        Value::Symbol(symbol) if symbol.is_nil() => Vec::new(),
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The second element of first argument must be a list of symbols that represents argument names"
        ))
    };
    let code = values;

    let argument_names = match parse_argument_names(arguments) {
        Ok(argument_names) => argument_names,
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The second element of the first argument must be a list of symbols that represents argument names"
        ))
    };

    match lambda_or_macro_symbol {
        Value::Symbol(symbol) if symbol.get_name() == "lambda" => {
            Ok(construct_interpreted_function(
                environment,
                argument_names,
                code
            ))
        },
        Value::Symbol(symbol) if symbol.get_name() == "macro" => {
            Ok(construct_macro_function(
                environment,
                argument_names,
                code
            ))
        },
        _ => return Err(Error::invalid_argument(
            interpreter,
            "The first element of the first argument must be a symbol `lambda' or `macro'"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn constructs_simple_function() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Function(Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            vec!(
                String::from("first-arg"),
                String::from("second-arg"),
            ),
            vec!(
                interpreter.intern("first-arg"),
                interpreter.intern("second-arg")
            )
        )));

        let result = interpreter.execute(
            "(function (lambda (first-arg second-arg) first-arg second-arg))"
        );
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn constructs_simple_macro() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Function(Function::Macro(MacroFunction::new(
            interpreter.get_root_environment(),
            vec!(
                String::from("first-arg"),
                String::from("second-arg"),
            ),
            vec!(
                interpreter.intern("first-arg"),
                interpreter.intern("second-arg")
            )
        )));

        let result = interpreter.execute(
            "(function (macro (first-arg second-arg) first-arg second-arg))"
        );
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn returns_correct_function_when_no_argument_was_provided() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Function(Function::Interpreted(InterpretedFunction::new(
            interpreter.get_root_environment(),
            vec!(),
            vec!(
                Value::Integer(1)
            )
        )));

        let result = interpreter.execute(
            "(function (lambda () 1))"
        );
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn returns_correct_macro_when_no_argument_was_provided() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Function(Function::Macro(MacroFunction::new(
            interpreter.get_root_environment(),
            vec!(),
            vec!(
                Value::Integer(1)
            )
        )));

        let result = interpreter.execute(
            "(function (macro () 1))"
        );
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(expected, result);
    }

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

    #[test]
    fn returns_error_when_first_element_is_not_lambda_nor_macro() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(function (special-form () 2))");

        assertion::assert_invalid_argument_error(&result);
    }

    #[test]
    fn returns_error_when_incorrect_amount_of_elements_of_first_argument_were_provided() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(function (lambda))");

        assertion::assert_invalid_argument_error(&result);

        let result = interpreter.execute("(function (lambda ()))");

        assertion::assert_invalid_argument_error(&result);
    }

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
