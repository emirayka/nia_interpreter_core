use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn let_star(
    interpreter: &mut Interpreter,
    environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Error::invalid_argument_count_error(
            "Special form `let*' must have at least one argument.",
        )
        .into();
    }

    let mut values = values;

    let definitions =
        library::read_as_let_definitions(interpreter, values.remove(0))
            .map_err(|_| {
                Error::invalid_argument_error("Invalid `let*' definitions.")
            })?;

    let forms = values;

    let execution_environment = interpreter
        .make_environment(environment)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    super::_let::set_definitions(
        interpreter,
        execution_environment,
        execution_environment,
        definitions,
    )?;

    library::evaluate_forms_return_last(
        interpreter,
        execution_environment,
        &forms,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils::assertion;

    #[test]
    fn returns_the_result_of_execution_of_the_last_form() {
        let mut interpreter = Interpreter::new();

        let specs = vec![("(let* () 3 2 1)", Value::Integer(1))];

        assertion::assert_results_are_correct(&mut interpreter, specs);
    }

    #[test]
    fn sets_symbol_with_executed_value() {
        let mut interpreter = Interpreter::new();

        let symbol = interpreter.intern_symbol_value("symbol");
        let nil = interpreter.intern_nil_symbol_value();

        let specs = vec![
            ("(let* ((value 1)) value)", Value::Integer(1)),
            ("(let* ((value 1.1)) value)", Value::Float(1.1)),
            ("(let* ((value #t)) value)", Value::Boolean(true)),
            ("(let* ((value #f)) value)", Value::Boolean(false)),
            (
                "(let* ((value 'symbol)) value)",
                interpreter.intern_symbol_value("symbol"),
            ),
            (
                "(let* ((value (quote symbol))) value)",
                interpreter.intern_symbol_value("symbol"),
            ),
            (
                "(let* ((value \"string\")) value)",
                interpreter.intern_string_value("string"),
            ),
            (
                "(let* ((value :keyword)) value)",
                interpreter.intern_keyword_value("keyword"),
            ),
            (
                "(let* ((value '(symbol))) value)",
                interpreter.make_cons_value(symbol, nil),
            ),
        ];

        assertion::assert_results_are_correct(&mut interpreter, specs);
    }

    #[test]
    fn sets_symbol_without_value_to_nil() {
        let mut interpreter = Interpreter::new();

        let specs = vec![("nil", "(let* (nil-symbol) nil-symbol)")];

        assertion::assert_results_are_equal(&mut interpreter, specs);
    }

    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            ("(let* ((a 1)) a)", Value::Integer(1)),
            ("(let* ((a 1)) (let* ((a 2) (b 3)) a))", Value::Integer(2)),
            ("(let* ((a 1)) (let* ((a 2) (b 3)) b))", Value::Integer(3)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, specs);
    }

    // the only difference between `let' `let*'
    #[test]
    fn able_to_use_previously_defined_values() {
        let mut interpreter = Interpreter::new();

        let specs =
            vec![("(let* ((sym-1 1) (sym-2 sym-1)) sym-2)", Value::Integer(1))];

        assertion::assert_results_are_correct(&mut interpreter, specs);
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_constant_or_special_symbol(
    ) {
        let mut interpreter = Interpreter::new();

        let mut specs = vec![
            // todo: when new constants will be, add them here
            "(let* ((nil 2)) nil)",
            // todo: when new special symbols will be, add them here
            "(let* ((#opt 2)) #opt)",
            "(let* ((#rest 2)) #rest)",
            "(let* ((#keys 2)) #keys)",
            // todo: remainder, when new special variable will be introduced, add them here
            "(let* ((this 2)) nil)",
            "(let* ((super 2)) nil)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_error_when_definition_is_constant_or_special_symbol() {
        let mut interpreter = Interpreter::new();

        let mut specs = vec![
            // todo: remainder, when new constants will be introduced, add them here
            "(let* (nil 2) nil)",
            // todo: remainder, when new special symbols will be introduced, add them here
            "(let* (#opt 2) #opt)",
            "(let* (#rest 2) #rest)",
            "(let* (#keys 2) #keys)",
            // todo: remainder, when new special variable will be introduced, add them here
            "(let* (this 2) nil)",
            "(let* (super 2) nil)",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_error_when_first_argument_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let incorrect_strings = vec![
            "(let* 1)",
            "(let* 1.1)",
            "(let* #t)",
            "(let* #f)",
            "(let* \"string\")",
            "(let* :keyword)",
            "(let* {})",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            incorrect_strings,
        );
    }

    #[test]
    fn returns_error_when_first_argument_contains_not_a_symbol_nor_cons() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(let* (1))",
            "(let* (1.1))",
            "(let* (#t))",
            "(let* (#f))",
            "(let* (\"string\"))",
            "(let* (:keyword))",
            "(let* (()))",
            "(let* (nil))",
            "(let* ({}))",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_error_when_first_part_of_definitions_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(let* ((1 2)) 1)",
            "(let* ((1.1 2)) 1.1)",
            "(let* ((#t 2)) #t)",
            "(let* ((#f 2)) #f)",
            "(let* ((:keyword 2)) :keyword)",
            "(let* ((\"string\" 2)) \"string\")",
            "(let* (((quote symbol) 2)) (quote symbol))",
            "(let* (({} 2)) {})",
        ];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_error_when_first_symbol_of_a_definition_is_nil() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(let* ((nil 2)) nil)"];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items()
    {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(let* ((sym)) nil)", "(let* ((sym 1 2)) nil)"];

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            specs,
        );
    }

    #[test]
    fn returns_err_when_attempt_to_redefine_already_defined_value() {
        let mut interpreter = Interpreter::new();

        let specs = vec!["(let* ((sym-1 1) (sym-1 2)) sym-1)"];

        assertion::assert_results_are_just_errors(&mut interpreter, specs);
    }
}
