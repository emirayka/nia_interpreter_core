use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::library;
use crate::interpreter::value::Value;

pub fn mlet_star(
    interpreter: &mut Interpreter,
    special_form_calling_environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() == 0 {
        return Error::invalid_argument_count_error(
            "Special form `mlet*' must have at least one argument.",
        )
        .into();
    }

    let mut values = values;

    let definitions = library::read_as_flet_definitions(interpreter, values.remove(0))
        .map_err(|_| Error::invalid_argument_error("Invalid `mlet*' definitions."))?;

    let forms = values;
    let macro_definition_environment = interpreter
        .make_environment(special_form_calling_environment)
        .map_err(|err| Error::generic_execution_error_caused("", err))?;

    super::mlet::set_definitions(
        interpreter,
        macro_definition_environment,
        macro_definition_environment,
        definitions,
    )?;

    library::execute_forms(interpreter, macro_definition_environment, &forms)
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

        let pairs = vec![
            ("(mlet* () 3)", Value::Integer(3)),
            ("(mlet* () 3 2)", Value::Integer(2)),
            ("(mlet* () 3 2 1)", Value::Integer(1)),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn able_to_execute_defined_macros() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (
                "(mlet* ((test-macro () 1)) (test-macro))",
                Value::Integer(1),
            ),
            (
                "(mlet* ((test-macro (a) a)) (test-macro 2))",
                Value::Integer(2),
            ),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn able_to_define_several_macros() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(mlet* ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-1))", Value::Integer(1)),
            ("(mlet* ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-2))", Value::Integer(2)),
            ("(mlet* ((test-macro-1 () 1) (test-macro-2 () 2) (test-macro-3 () 3)) (test-macro-3))", Value::Integer(3)),
        );

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn makes_a_correct_macro() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![(
            "(mlet* ((test-macro-1 () (quote (quote test)))) (test-macro-1))",
            "'test",
        )];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn possible_to_use_rest_opt_key_arguments() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(mlet* ((a (#rest a) (list 'quote a))) (a))", "(list)"),
            ("(mlet* ((a (#rest a) (list 'quote a))) (a 1))", "(list 1)"),
            (
                "(mlet* ((a (#rest a) (list 'quote a))) (a 1 2))",
                "(list 1 2)",
            ),
            ("(mlet* ((a (#opt a) (list 'list a))) (a))", "(list nil)"),
            ("(mlet* ((a (#opt (a 1)) (list 'list a))) (a))", "(list 1)"),
            (
                "(mlet* ((a (#opt (a 1 a?)) (list 'list a a?))) (a))",
                "(list 1 #f)",
            ),
            ("(mlet* ((a (#opt a) (list 'list a))) (a 2))", "(list 2)"),
            (
                "(mlet* ((a (#opt (a 1)) (list 'list a))) (a 2))",
                "(list 2)",
            ),
            (
                "(mlet* ((a (#opt (a 1 a?)) (list 'list a a?))) (a 2))",
                "(list 2 #t)",
            ),
            ("(mlet* ((a (#keys a) (list 'list a))) (a))", "(list nil)"),
            ("(mlet* ((a (#keys (a 1)) (list 'list a))) (a))", "(list 1)"),
            (
                "(mlet* ((a (#keys (a 1 a?)) (list 'list a a?))) (a))",
                "(list 1 #f)",
            ),
            (
                "(mlet* ((a (#keys a) (list 'list a))) (a :a 2))",
                "(list 2)",
            ),
            (
                "(mlet* ((a (#keys (a 1)) (list 'list a))) (a :a 2))",
                "(list 2)",
            ),
            (
                "(mlet* ((a (#keys (a 1 a?)) (list 'list a a?))) (a :a 2))",
                "(list 2 #t)",
            ),
        ];

        assertion::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn possible_to_nest_let_invocations() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(mlet* ((a () 1)) (a))", Value::Integer(1)),
            (
                "(mlet* ((a () 1)) (mlet ((a () 2) (b () 3)) (a)))",
                Value::Integer(2),
            ),
            (
                "(mlet* ((a () 1)) (mlet ((a () 2) (b () 3)) (b)))",
                Value::Integer(3),
            ),
        ];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn able_to_use_previously_defined_macros() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![(
            "(mlet* ((sym-1 () 1) (sym-2 () (sym-1))) (sym-2))",
            Value::Integer(1),
        )];

        assertion::assert_results_are_correct(&mut interpreter, pairs);
    }

    fn returns_error_when_first_symbol_of_a_definition_is_constant_or_special_symbol() {
        let mut interpreter = Interpreter::new();

        let mut specs = vec![
            // todo: when new constants will be, add them here
            "(mlet* ((nil 2)) nil)",
            // todo: when new special symbols will be, add them here
            "(mlet* ((#opt 2)) #opt)",
            "(mlet* ((#rest 2)) #rest)",
            "(mlet* ((#keys 2)) #keys)",
            // todo: remainder, when new special variable will be introduced, add them here
            "(mlet* ((this 2)) nil)",
            "(mlet* ((super 2)) nil)",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, specs);
    }

    #[test]
    fn returns_error_when_first_argument_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let incorrect_strings = vec![
            "(mlet* 1)",
            "(mlet* 1.1)",
            "(mlet* #t)",
            "(mlet* #f)",
            "(mlet* 'symbol)",
            "(mlet* \"string\")",
            "(mlet* :keyword)",
            "(mlet* {})",
            "(mlet* #(+ %1 %2))",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, incorrect_strings);
    }

    #[test]
    fn returns_error_when_first_argument_contains_not_a_symbol_nor_cons() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(mlet* (1))",
            "(mlet* (1.1))",
            "(mlet* (#t))",
            "(mlet* (#f))",
            "(mlet* (\"string\"))",
            "(mlet* (:keyword))",
            "(mlet* ({}))",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_error_when_first_part_of_macro_definition_is_not_a_symbol() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(mlet* ((1 () 2)) 1)",
            "(mlet* ((1.1 () 2)) 1.1)",
            "(mlet* ((#t () 2)) #t)",
            "(mlet* ((#f () 2)) #f)",
            "(mlet* ((\"string\" () 2)) \"string\")",
            "(mlet* ((:keyword () 2)) :keyword)",
            "(mlet* (((quote symbol) () 2)) (quote symbol))",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_error_when_arguments_is_not_a_list() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec![
            "(mlet* ((func 1 2)) (func))",
            "(mlet* ((func 1.1 2)) (func))",
            "(mlet* ((func #t 2)) (func))",
            "(mlet* ((func #f 2)) (func))",
            "(mlet* ((func \"string\" 2)) (func))",
            "(mlet* ((func :keyword 2)) (func))",
            "(mlet* ((func some-symbol 2)) (func))",
        ];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_err_when_definition_is_a_list_but_have_incorrect_count_of_items() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(mlet* ((sym)) nil)"];

        assertion::assert_results_are_invalid_argument_errors(&mut interpreter, code_vector);
    }

    #[test]
    fn returns_err_when_attempts_to_redefine_already_defined_macro() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!["(mlet* ((sym-1 () 1) (sym-1 () 2)) (sym-1))"];

        assertion::assert_results_are_just_errors(&mut interpreter, code_vector);
    }
}
