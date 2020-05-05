use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::value::{Function, InterpretedFunction};

use crate::interpreter::value::FunctionArguments;

use crate::interpreter::library;

pub fn combine(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `func:combine' takes one argument at least.",
        )
        .into();
    }

    let mut values = values;
    let nil = interpreter.intern_nil_symbol_value();

    let function_id = library::read_as_function_id(values.remove(0))?;

    let mut arguments = Vec::new();

    for value in values {
        let argument_function_id = library::read_as_function_id(value)?;

        let cons = interpreter
            .make_cons_value(Value::Function(argument_function_id), nil);
        arguments.push(cons)
    }

    let cons = interpreter.vec_to_list(arguments);

    let result =
        interpreter.make_cons_value(Value::Function(function_id), cons);

    let interpreted_function = InterpretedFunction::new(
        environment_id,
        FunctionArguments::new(),
        vec![result],
    );

    let constructed_function_id = interpreter
        .register_function(Function::Interpreted(interpreted_function));

    Ok(Value::Function(constructed_function_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn combines_functions() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("((func:combine #()))", "'()"),
            ("((func:combine #(+ %1 %1) (func:always 1)))", "2"),
            (
                "((func:combine #(+ %1 %2) (func:always 1) (func:always 2)))",
                "3",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(func:combine 1)",
            "(func:combine 1.1)",
            "(func:combine #t)",
            "(func:combine #f)",
            "(func:combine \"string\")",
            "(func:combine 'symbol)",
            "(func:combine :keyword)",
            "(func:combine '(1 2 3))",
            "(func:combine {})",
            "(func:combine #(+ %1 %1) 1)",
            "(func:combine #(+ %1 %1) 1.1)",
            "(func:combine #(+ %1 %1) #t)",
            "(func:combine #(+ %1 %1) #f)",
            "(func:combine #(+ %1 %1) \"string\")",
            "(func:combine #(+ %1 %1) 'symbol)",
            "(func:combine #(+ %1 %1) :keyword)",
            "(func:combine #(+ %1 %1) '(1 2 3))",
            "(func:combine #(+ %1 %1) {})",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(func:combine)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
