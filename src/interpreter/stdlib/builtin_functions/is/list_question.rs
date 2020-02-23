use crate::interpreter::error::Error;
use crate::interpreter::value::Value;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;

pub fn list_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 1 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `list?' must take exactly one argument."
        ).into_result();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Cons(cons_id) => {
            let mut current_cons = cons_id;
            let mut is_list_correct = true;

            loop {
                let cdr = interpreter.get_cdr(current_cons)
                    .map_err(|err| interpreter.make_generic_execution_error_caused(
                        "",
                        err
                    ))?;

                match cdr {
                    Value::Symbol(symbol_id) => {
                        let symbol = match interpreter.get_symbol(symbol_id) {
                            Ok(value) => value,
                            Err(error) => return interpreter.make_generic_execution_error_caused(
                                "",
                                error
                            ).into_result()
                        };

                        is_list_correct = symbol.is_nil();

                        break;
                    },
                    Value::Cons(next_cons_id) => {
                        current_cons = next_cons_id;
                    },
                    _ => {
                        is_list_correct = false;
                        break;
                    }
                }
            }

            is_list_correct
        },
        Value::Symbol(symbol_id) => {
            let symbol = match interpreter.get_symbol(symbol_id) {
                Ok(value) => value,
                Err(error) => return interpreter.make_generic_execution_error_caused(
                    "",
                    error
                ).into_result()
            };

            symbol.is_nil()
        },
        _ => false
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::testing_helpers::for_value_pairs_evaluated_ifbsykcou;
    use crate::interpreter::lib::assertion;

    #[test]
    fn returns_true_when_nil_was_provided() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Boolean(true);
        let result = interpreter.execute("(is:list? ())").unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn returns_true_when_one_item_list_was_provided() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Boolean(true);
        let result = interpreter.execute("(is:list? (cons 1 nil))").unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn returns_true_when_two_item_list_was_provided() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Boolean(true);
        let result = interpreter.execute("(is:list? (cons 1 (cons 2 nil)))").unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn returns_false_when_a_cons_cell_with_not_a_nil_at_the_last_cdr_was_provided() {
        let mut interpreter = Interpreter::new();

        let expected = Value::Boolean(false);
        let result = interpreter.execute("(is:list? (cons 1 2))").unwrap();

        assert_eq!(expected, result);

        let expected = Value::Boolean(false);
        let result = interpreter.execute("(is:list? (cons 1 (cons 2 3)))").unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn returns_false_when_not_an_list_was_passed() {
        for_value_pairs_evaluated_ifbsykcou(
            |interpreter, code, value| {
                if let Value::Cons(_) = value {
                    return;
                }

                let code = format!("(is:list? {})", code);
                let result = interpreter.execute(&code).unwrap();
                let expected = Value::Boolean(false);

                assert_eq!(expected, result);
            }
        )
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let result = interpreter.execute("(is:list?)");
        assertion::assert_invalid_argument_count_error(&result);

        let result = interpreter.execute("(is:list? 1 2)");
        assertion::assert_invalid_argument_count_error(&result);
    }
}
