use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::interpreter::library;

pub fn _match(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    if values.len() < 1 {
        return Error::invalid_argument_count_error("").into();
    }

    let mut values = values;

    let expression = values.remove(0);
    let clauses = values;

    let value_to_match = interpreter
        .execute_value(environment_id, expression)
        .map_err(|err| {
            Error::generic_execution_error_caused(
                "Cannot evaluate value in the `match' special form.",
                err,
            )
        })?;

    let mut child_environment_id = None;
    let mut code_to_execute = None;

    for clause in clauses {
        let cons_id = library::read_as_cons_id(clause)?;

        let mut clause = interpreter.list_to_vec(cons_id)?;
        let pattern = clause.remove(0);
        let evaluated_pattern =
            interpreter.execute_value(environment_id, pattern)?;

        match library::match_value(
            interpreter,
            environment_id,
            evaluated_pattern,
            value_to_match,
        ) {
            Ok(environment_id) => {
                child_environment_id = Some(environment_id);

                let forms = clause;

                code_to_execute = Some(forms);
                break;
            },
            _ => {},
        }
    }

    match child_environment_id {
        Some(environment_id) => library::evaluate_forms_return_last(
            interpreter,
            environment_id,
            &code_to_execute.unwrap(),
        ),
        _ => Error::generic_execution_error("").into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn executes_forms() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(match () ('()))", interpreter.intern_nil_symbol_value()),
            ("(match () ('() 1))", Value::Integer(1)),
            ("(match () ('() 1 2))", Value::Integer(2)),
            ("(match () ('() 1 2 3))", Value::Integer(3)),
        ];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn able_to_destructurize_lists() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(match '()      ('() nil))", "nil"),
            ("(match '(1)     ('(a) (list a)))", "(list 1)"),
            ("(match '(1 2)   ('(a b) (list a b)))", "(list 1 2)"),
            ("(match '(1 2 3) ('(a b c) (list a b c)))", "(list 1 2 3)"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn able_to_destructurize_inner_lists() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(match '(1)     ('(a) (list a)))", "(list 1)"),
            ("(match '((1))   ('((a)) (list a)))", "(list 1)"),
            ("(match '(((1))) ('(((a))) (list a)))", "(list 1)"),
            ("(match '(1)         ('(a) (list a)))", "(list 1)"),
            ("(match '((1) 2)     ('((a) b) (list a b)))", "(list 1 2)"),
            (
                "(match '(((1) 2) 3) ('(((a) b) c) (list a b c)))",
                "(list 1 2 3)",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn able_to_use_different_patterns() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            (
                "(match '() (())  ('(a) (list a)) ('(a b) (list a b)) ('(a b c) (list a b c)))",
                "()",
            ),
            (
                "(match '(1) (()) ('(a) (list a)) ('(a b) (list a b)) ('(a b c) (list a b c)))",
                "(list 1)",
            ),
            (
                "(match '(1 2) (()) ('(a) (list a)) ('(a b) (list a b)) ('(a b c) (list a b c)))",
                "(list 1 2)",
            ),
            (
                "(match '(1 2 3) (()) ('(a) (list a)) ('(a b) (list a b)) ('(a b c) (list a b c)))",
                "(list 1 2 3)",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn able_to_destructurize_objects() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(match {:a 1 :b 2 :c 3} (#{:a} (list a)))", "(list 1)"),
            (
                "(match {:a 1 :b 2 :c 3} (#{:a :b} (list a b)))",
                "(list 1 2)",
            ),
            (
                "(match {:a 1 :b 2 :c 3} (#{:a :b :c} (list a b c)))",
                "(list 1 2 3)",
            ),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }
}
