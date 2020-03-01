use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::environment_arena::EnvironmentId;

pub fn car(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `car' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let car = match values.remove(0) {
        Value::Cons(cons_id) => interpreter.get_car(cons_id)
            .map_err(|err| interpreter.make_generic_execution_error_caused(
                "",
                err
            ))?,
        _ => return interpreter.make_invalid_argument_error(
            ""
        ).into_result()
    };

    Ok(car)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_car_of_cons() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(car (cons 1 1))", Value::Integer(1)),
            ("(car (cons 1.1 1))", Value::Float(1.1)),
            ("(car (cons #t 1))", Value::Boolean(true)),
            ("(car (cons #f 1))", Value::Boolean(false)),
            ("(car (cons \"string\" 1))", interpreter.intern_string_value(String::from("string"))),
            ("(car (cons 'symbol 1))", interpreter.intern_symbol_value("symbol")),
            ("(car (cons :keyword 1))", interpreter.intern_keyword_value(String::from("keyword"))),
            ("(car (cons {} 1))", interpreter.make_object_value()),
            ("(car (cons (cons 1 2) 1))", interpreter.make_cons_value(Value::Integer(1), Value::Integer(2))),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_count_when_called_with_invalid_count_of_arguments() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(car)",
            "(car (cons 1 2) 3)"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_called_with_a_value_that_is_not_cons() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(car 1)",
            "(car 1.1)",
            "(car #t)",
            "(car #f)",
            "(car \"string\")",
            "(car 'symbol)",
            "(car :keyword)",
            "(car {})",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }
}