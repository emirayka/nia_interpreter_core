use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn zip(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 2 {
        return Error::invalid_argument_count_error(
            "Built-in function `list:zip' takes two arguments exactly.",
        )
        .into();
    }

    let mut values = values;

    let vector1 = library::read_as_vector(interpreter, values.remove(0))?;

    let vector2 = library::read_as_vector(interpreter, values.remove(0))?;

    if vector1.len() != vector2.len() {
        return Error::invalid_argument_error(
            "Built-in function `list:zip' takes two lists of equal length.",
        )
        .into();
    }

    let mut result = Vec::new();
    let nil = interpreter.intern_nil_symbol_value();
    let iterator = vector1.iter().zip(vector2.iter());

    for (v1, v2) in iterator {
        let cons = interpreter.make_cons_value(*v2, nil);
        let cons = interpreter.make_cons_value(*v1, cons);

        result.push(cons);
    }

    Ok(interpreter.vec_to_list(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_concatenated_lists() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            ("(list:zip '(1) '(a))", "'((1 a))"),
            ("(list:zip '(1 2) '(a b))", "'((1 a) (2 b))"),
            ("(list:zip '(1 2 3) '(a b c))", "'((1 a) (2 b) (3 c))"),
            (
                "(list:zip '(1 2 3 4) '(a b c d))",
                "'((1 a) (2 b) (3 c) (4 d))",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(list:zip 1 '())",
            "(list:zip 1.1 '())",
            "(list:zip #t '())",
            "(list:zip #f '())",
            "(list:zip \"string\" '())",
            "(list:zip 'symbol '())",
            "(list:zip :keyword '())",
            "(list:zip {} '())",
            "(list:zip #() '())",
            "(list:zip '() 1)",
            "(list:zip '() 1.1)",
            "(list:zip '() #t)",
            "(list:zip '() #f)",
            "(list:zip '() \"string\")",
            "(list:zip '() 'symbol)",
            "(list:zip '() :keyword)",
            "(list:zip '() {})",
            "(list:zip '() #())",
        ];

        utils::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector,
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector =
            vec!["(list:zip)", "(list:zip 1)", "(list:zip 1 2 3)"];

        utils::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
