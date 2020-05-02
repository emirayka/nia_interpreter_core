use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::value::{
    FunctionArguments,
    Function,
    InterpretedFunction,
};

pub fn always(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `func:always' takes one argument exactly."
        ).into();
    }

    let mut values = values;

    let nil = interpreter.intern_nil_symbol_value();
    let quote = interpreter.intern_symbol_value("quote");
    let value = values.remove(0);

    let cons = interpreter.make_cons_value(value, nil);
    let cons = interpreter.make_cons_value(quote, cons);

    let code = vec!(cons);
    let arguments = FunctionArguments::new();

    let interpreted_function = InterpretedFunction::new(
        environment_id,
        arguments,
        code
    );

    let function_id = interpreter.register_function(
        Function::Interpreted(interpreted_function)
    );

    Ok(Value::Function(function_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn returns_true_when_an_atom_was_passed() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("((func:always 1))", "1"),
            ("((func:always 1.1))", "1.1"),
            ("((func:always #t))", "#t"),
            ("((func:always #f))", "#f"),
            ("((func:always \"string\"))", "\"string\""),
            ("((func:always 'symbol))", "'symbol"),
            ("((func:always :keyword))", ":keyword"),
            ("((func:always '(1 2 3)))", "'(1 2 3)"),
            ("((func:always {:a 1}))", "{:a 1}"),
            ("((func:always #()))", "#()"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(func:always)",
            "(func:always 1 2)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
