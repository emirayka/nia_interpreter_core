use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::lib;

pub fn _match(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 1 {
        return interpreter.make_invalid_argument_count_error(
            ""
        ).into_result()
    }

    let mut values = values;

    let expression = values.remove(0);
    let clauses = values;

    let value_to_match = interpreter.evaluate_value(
        environment_id,
        expression
    ).map_err(|err| interpreter.make_generic_execution_error_caused(
        "Cannot evaluate value in the `match' special form.",
        err
    ))?;

    let mut child_environment_id = None;
    let mut code_to_execute = None;

    for clause in clauses {
        let cons_id = lib::read_as_cons_id(
            interpreter,
            clause
        )?;

        let mut clause = interpreter.list_to_vec(cons_id)?;
        let pattern = clause.remove(0);
        let evaluated_pattern = interpreter.evaluate_value(
            environment_id,
            pattern
        )?;

        match lib::match_value(
            interpreter,
            environment_id,
            evaluated_pattern,
            value_to_match
        ) {
            Ok(environment_id) => {
                child_environment_id = Some(environment_id);

                let mut forms = clause;

                code_to_execute = Some(forms);
                break;
            },
            _ => {}
        }
    };

    match child_environment_id {
        Some(environment_id) => {
            lib::execute_forms(
                interpreter,
                environment_id,
                code_to_execute.unwrap()
            )
        },
        _ => interpreter.make_generic_execution_error(
            ""
        ).into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion;

    #[test]
    fn executes_forms() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(match () ('()))", interpreter.intern_nil_symbol_value()),
            ("(match () ('() 1))", Value::Integer(1)),
            ("(match () ('() 1 2))", Value::Integer(2)),
            ("(match () ('() 1 2 3))", Value::Integer(3)),
        );

        assertion::assert_results_are_correct(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn able_to_destructurize_lists() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(match '()      ('() nil))", "nil"),
            ("(match '(1)     ('(a) (list a)))", "(list 1)"),
            ("(match '(1 2)   ('(a b) (list a b)))", "(list 1 2)"),
            ("(match '(1 2 3) ('(a b c) (list a b c)))", "(list 1 2 3)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn able_to_destructurize_inner_lists() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(match '(1)     ('(a) (list a)))", "(list 1)"),
            ("(match '((1))   ('((a)) (list a)))", "(list 1)"),
            ("(match '(((1))) ('(((a))) (list a)))", "(list 1)"),

            ("(match '(1)         ('(a) (list a)))", "(list 1)"),
            ("(match '((1) 2)     ('((a) b) (list a b)))", "(list 1 2)"),
            ("(match '(((1) 2) 3) ('(((a) b) c) (list a b c)))", "(list 1 2 3)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn able_to_use_different_patterns() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(match '() (())  ('(a) (list a)) ('(a b) (list a b)) ('(a b c) (list a b c)))", "()"),
            ("(match '(1) (()) ('(a) (list a)) ('(a b) (list a b)) ('(a b c) (list a b c)))", "(list 1)"),
            ("(match '(1 2) (()) ('(a) (list a)) ('(a b) (list a b)) ('(a b c) (list a b c)))", "(list 1 2)"),
            ("(match '(1 2 3) (()) ('(a) (list a)) ('(a b) (list a b)) ('(a b c) (list a b c)))", "(list 1 2 3)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn able_to_destructurize_objects() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(match {:a 1 :b 2 :c 3} (#{:a} (list a)))", "(list 1)"),
            ("(match {:a 1 :b 2 :c 3} (#{:a :b} (list a b)))", "(list 1 2)"),
            ("(match {:a 1 :b 2 :c 3} (#{:a :b :c} (list a b c)))", "(list 1 2 3)"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }
}
