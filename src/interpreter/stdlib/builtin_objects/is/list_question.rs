use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn list_question(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() != 1 {
        return Error::invalid_argument_count_error(
            "Built-in function `list?' must take exactly one argument.",
        )
        .into();
    }

    let mut values = values;

    let result = match values.remove(0) {
        Value::Cons(cons_id) => {
            let mut current_cons = cons_id;
            let mut is_list_correct = true;

            loop {
                let cdr = interpreter.get_cdr(current_cons).map_err(|err| {
                    Error::generic_execution_error_caused("", err)
                })?;

                match cdr {
                    Value::Symbol(symbol_id) => {
                        is_list_correct =
                            interpreter.symbol_is_nil(symbol_id)?;

                        break;
                    },
                    Value::Cons(next_cons_id) => {
                        current_cons = next_cons_id;
                    },
                    _ => {
                        is_list_correct = false;
                        break;
                    },
                }
            }

            is_list_correct
        },
        Value::Symbol(symbol_id) => interpreter.symbol_is_nil(symbol_id)?,
        _ => false,
    };

    Ok(Value::Boolean(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_true_when_list_was_provided() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:list? ())", Value::Boolean(true)),
            ("(is:list? (cons 1 nil))", Value::Boolean(true)),
            ("(is:list? (cons 1 (cons 2 nil)))", Value::Boolean(true)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_false_when_not_a_list_was_provided() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(is:list? 1)", Value::Boolean(false)),
            ("(is:list? 1.1)", Value::Boolean(false)),
            ("(is:list? #t)", Value::Boolean(false)),
            ("(is:list? #f)", Value::Boolean(false)),
            ("(is:list? \"string\")", Value::Boolean(false)),
            ("(is:list? 'symbol)", Value::Boolean(false)),
            ("(is:list? :keyword)", Value::Boolean(false)),
            ("(is:list? {})", Value::Boolean(false)),
            ("(is:list? #())", Value::Boolean(false)),
            ("(is:list? (cons 1 2))", Value::Boolean(false)),
            ("(is:list? (cons 1 (cons 2 3)))", Value::Boolean(false)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed(
    ) {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(is:list?)", "(is:list? 1 2)"];

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector,
        );
    }
}
