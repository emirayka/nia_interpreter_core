use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::cons::cons_arena::ConsId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::lib;

pub fn match_value_recursive(
    interpreter: &mut Interpreter,
    environment_id: EnvironmentId,
    binding: Value,
    value: Value
) -> Result<(), Error> {
    match binding {
        Value::Symbol(symbol_id) => {
            match lib::check_if_symbol_assignable(interpreter, symbol_id) {
                Ok(_) => {},
                Err(err) => {
                    if interpreter.deep_equal(binding, value)? {
                        return Ok(())
                    } else {
                        return Err(err);
                    }
                }
            };

            interpreter.define_variable(
                environment_id,
                symbol_id,
                value
            )
        },
        Value::Cons(binding_cons_id) => {
            match value {
                Value::Cons(value_cons_id) => {
                    let binding_car = interpreter.get_car(binding_cons_id)?;
                    let value_car = interpreter.get_car(value_cons_id)?;

                    let binding_cdr = interpreter.get_cdr(binding_cons_id)?;
                    let value_cdr = interpreter.get_cdr(value_cons_id)?;

                    match_value_recursive(
                        interpreter,
                        environment_id,
                        binding_car,
                        value_car
                    )?;

                    match_value_recursive(
                        interpreter,
                        environment_id,
                        binding_cdr,
                        value_cdr
                    )?;

                    Ok(())
                },
                _ => {
                    return interpreter.make_generic_execution_error(
                        ""
                    ).into_result()
                }
            }
        },
        Value::Object(binding_object_id) => {
            match value {
                Value::Object(value_object_id) => {
                    let binding_keys = interpreter.get_items(binding_object_id)?;
                    let value_keys = interpreter.get_items(value_object_id)?;

                    let mut checkings = Vec::new();

                    for (symbol_id, binding_value) in binding_keys {
                        match value_keys.get(symbol_id) {
                            Some(value_value) => {
                                checkings.push((*binding_value, *value_value));
                            },
                            _ => return interpreter.make_generic_execution_error(
                                ""
                            ).into_result()
                        }
                    }

                    for (binding_value, value) in checkings {
                        match_value_recursive(
                            interpreter,
                            environment_id,
                            binding_value,
                            value
                        )?;
                    }

                    Ok(())
                },
                _ => {
                    return interpreter.make_generic_execution_error(
                        ""
                    ).into_result()
                }
            }
        },
        Value::Function(_) => return interpreter.make_generic_execution_error(
            ""
        ).into_result(),
        binding => {
            if binding == value {
                Ok(())
            } else {
                interpreter.make_generic_execution_error(
                    ""
                ).into_result()
            }
        }
    }
}

pub fn match_value(
    interpreter: &mut Interpreter,
    parent_environment: EnvironmentId,
    binding: Value,
    value: Value
) -> Result<EnvironmentId, Error> {
    let environment_id = interpreter.make_environment(parent_environment)?;

    let result = match_value_recursive(
        interpreter,
        environment_id,
        binding,
        value
    );

    match result {
        Ok(_) => Ok(environment_id),
        Err(err) => {
            // todo: remove environment here
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lib::assertion::assert_deep_equal;

    fn assert_matches_correctly(interpreter: &mut Interpreter, specs: Vec<(&str, &str, &str, &str)>) {
        for spec in specs {
            let binding = interpreter.execute(spec.0).unwrap();
            let value = interpreter.execute(spec.1).unwrap();

            let form = interpreter.execute(spec.2).unwrap();
            let expected = interpreter.execute(spec.3).unwrap();

            let environment_id = match_value(
                interpreter,
                interpreter.get_root_environment(),
                binding,
                value
            ).unwrap();

            let result = interpreter.evaluate_value(
                environment_id,
                form
            ).unwrap();

            assert_deep_equal(
                interpreter,
                expected,
                result
            );
        }
    }

    fn assert_match_fails(interpreter: &mut Interpreter, specs: Vec<(&str, &str)>) {
        for spec in specs {
            let binding = interpreter.execute(spec.0).unwrap();
            let value = interpreter.execute(spec.1).unwrap();

            let environment_id = match_value(
                interpreter,
                interpreter.get_root_environment(),
                binding,
                value
            );

            assert!(environment_id.is_err());
        }
    }

    #[test]
    fn matches_a_value_to_symbol() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("'a", "1",          "'a", "1"),
            ("'a", "1.1",        "'a", "1.1"),
            ("'a", "#t",         "'a", "#t"),
            ("'a", "#f",         "'a", "#f"),
            ("'a", "\"string\"", "'a", "\"string\""),
            ("'a", "'symbol",    "'a", "'symbol"),
            ("'a", ":keyword",   "'a", ":keyword"),
            ("'a", "'(1 2)",     "'a", "'(1 2)"),
            ("'a", "{}",         "'a", "{}"),
            ("'a", "#()",        "'a", "#()"),
        );

        assert_matches_correctly(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn matches_lists() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("'()", "'()", "'nil", "nil"),
            ("'(a)", "'(1)", "'(list a)", "(list 1)"),
            ("'(a b)", "'(1 2)", "'(list a b)", "(list 1 2)"),
            ("'(a b ())", "'(1 2 ())", "'(list a b)", "(list 1 2)"),
            ("'(a b (c))", "'(1 2 (3))", "'(list a b c)", "(list 1 2 3)"),
            ("'(a b (c d))", "'(1 2 (3 4))", "'(list a b c d)", "(list 1 2 3 4)"),
            ("'(a b (c d e))", "'(1 2 (3 4 5))", "'(list a b c d e)", "(list 1 2 3 4 5)"),
        );

        assert_matches_correctly(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn matches_objects() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("{}", "{}", "nil", "nil"),
            ("#{:a}", "{:a 1}", "'a", "1"),
            ("#{:a :b}", "{:a 1 :b 2}", "'(list a b)", "(list 1 2)"),
            ("#{:a :b}", "{:b 2 :a 1}", "'(list a b)", "(list 1 2)"),

            ("{:a '(a b c)}", "{:a '(1 2 3)}", "'(list a b c)", "(list 1 2 3)"),
        );

        assert_matches_correctly(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn matches_simple_values() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("1", "1", "nil", "nil"),
            ("1.1", "1.1", "nil", "nil"),
            ("#t", "#t", "nil", "nil"),
            ("#f", "#f", "nil", "nil"),
            ("\"string\"", "\"string\"", "nil", "nil"),
            (":keyword", ":keyword", "nil", "nil"),
        );

        assert_matches_correctly(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn matches_simple_values_in_lists() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("'(a 2)", "'(1 2)", "'a", "1"),
            ("'(a 2.2)", "'(1.1 2.2)", "'a", "1.1"),
            ("'(a #f)", "'(#t #f)", "'a", "#t"),
            ("'(a #t)", "'(#f #t)", "'a", "#f"),
            ("'(a \"string-2\")", "'(\"string-1\" \"string-2\")", "'a", "\"string-1\""),
            ("'(a :keyword-2)", "'(:keyword-1 :keyword-2)", "'a", ":keyword-1"),
        );

        assert_matches_correctly(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn does_not_match_when_simple_values_are_not_equal() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("'(a 2)", "'(1 3)"),
            ("'(a 2.2)", "'(1.1 3.3)"),
            ("'(a #f)", "'(#t #t)"),
            ("'(a #t)", "'(#f #f)"),
            ("'(a \"string-2\")", "'(\"string-1\" \"string-3\")"),
            ("'(a :keyword-2)", "'(:keyword-1 :keyword-3)"),
        );

        assert_match_fails(
            &mut interpreter,
            specs
        );
    }

    #[test]
    fn fails_when_list_are_not_equal() {
        let mut interpreter = Interpreter::new();

        let specs = vec!(
            ("'()", "'(1)"),
            ("'(a)", "'(1 2)"),
            ("'(a b)", "'(1 2 3)"),
        );

        assert_match_fails(
            &mut interpreter,
            specs
        );
    }
}
