use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::function::Function;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::{library, InterpretedFunction};
use crate::interpreter::function::function_arena::FunctionId;
use crate::interpreter::function::arguments::Arguments;

pub fn always(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `func:always' takes one argument exactly."
        ).into_result();
    }

    let mut values = values;

    let nil = interpreter.intern_nil_symbol_value();
    let quote = interpreter.intern_symbol_value("quote");
    let value = values.remove(0);

    let cons = interpreter.make_cons_value(value, nil);
    let cons = interpreter.make_cons_value(quote, cons);

    let code = vec!(cons);
    let arguments = Arguments::new();

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
