use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::library;

pub fn set_mark(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `object:set!' takes three arguments exactly."
        ).into();
    }

    let mut values = values;
    let object_id = library::read_as_object_id(
        interpreter,
        values.remove(0)
    )?;

    let symbol_id = library::read_keyword_or_symbol_as_symbol_id(
        interpreter,
        values.remove(0)
    )?;

    library::check_if_symbol_assignable(interpreter, symbol_id)?;

    let value = values.remove(0);

    interpreter.set_object_property(
        object_id,
        symbol_id,
        value
    )?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;
    use crate::interpreter::library::testing_helpers::for_special_symbols;

    #[test]
    fn sets_item_to_object() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((obj {:a 1})) (object:set! obj 'a 2) (object:get obj 'a))", Value::Integer(2)),
            ("(let ((obj {:a 1})) (object:set! obj :a 2) (object:get obj :a))", Value::Integer(2)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_value_that_were_set() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((obj {:a 1})) (object:set! obj 'a 2))", Value::Integer(2))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn able_to_set_values_that_were_not_in_the_object_initially() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let ((obj {:a 1})) (object:set! obj 'b 2) (object:get obj 'b))", Value::Integer(2))
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_when_attempt_get_item_by_special_symbol() {
        for_special_symbols(|interpreter, string| {
            let result = interpreter.execute(
                    &(String::from("(let ((obj {:item 1})) (object:set! obj '") + &string +" 2))")
            );
            assertion::assert_invalid_argument_error(&result);
        })
    }

    #[test]
    fn returns_invalid_argument_when_first_argument_is_not_an_object() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj 2)) (object:set! obj 'item 2))",
            "(let ((obj 2.2)) (object:set! obj 'item 2))",
            "(let ((obj #t)) (object:set! obj 'item 2))",
            "(let ((obj #f)) (object:set! obj 'item 2))",
            "(let ((obj \"string\")) (object:set! obj 'item 2))",
            "(let ((obj 'symbol)) (object:set! obj 'item 2))",
            "(let ((obj :keyword)) (object:set! obj 'item 2))",
            "(let ((obj '(list))) (object:set! obj 'item 2))",
            "(let ((obj #())) (object:set! obj 'item 2))",

            "(let ((obj {:a 1})) (object:set! obj 2 2))",
            "(let ((obj {:a 1})) (object:set! obj 2.2 2))",
            "(let ((obj {:a 1})) (object:set! obj #t 2))",
            "(let ((obj {:a 1})) (object:set! obj #f 2))",
            "(let ((obj {:a 1})) (object:set! obj \"string\" 2))",
            "(let ((obj {:a 1})) (object:set! obj '(list) 2))",
            "(let ((obj {:a 1})) (object:set! obj {} 2))",
            "(let ((obj {:a 1})) (object:set! obj #() 2))"
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_argument_count_is_not_correct() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(let ((obj {:item 1})) (object:set!))",
            "(let ((obj {:item 1})) (object:set! obj))",
            "(let ((obj {:item 1})) (object:set! obj 'item))",
            "(let ((obj {:item 1})) (object:set! obj 'item 'sym2 'sym3))"
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
