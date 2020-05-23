use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn alist_acons(
    interpreter: &mut Interpreter,
    key: Value,
    value: Value,
    alist: Value,
) -> Result<Value, Error> {
    let cons = interpreter.make_cons_value(key, value);
    let new_alist = interpreter.make_cons_value(cons, alist);

    Ok(new_alist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_alist_with_new_key_additional_key_value_pair() {
        let mut interpreter = Interpreter::new();

        let mut resulting_alist = interpreter.intern_nil_symbol_value();

        let specs = vec![
            ("1", "2", "(cons:new (cons:new 1 2) nil)"),
            ("3", "4", "(list:new (cons:new 3 4) (cons:new 1 2))"),
            (
                "5",
                "6",
                "(list:new (cons:new 5 6) (cons:new 3 4) (cons:new 1 2))",
            ),
        ];

        for (key, value, expected) in specs {
            let expected_alist =
                interpreter.execute_in_main_environment(expected).unwrap();

            let key = interpreter.execute_in_main_environment(key).unwrap();
            let value = interpreter.execute_in_main_environment(value).unwrap();

            resulting_alist =
                alist_acons(&mut interpreter, key, value, resulting_alist)
                    .unwrap();

            utils::assert_deep_equal(
                &mut interpreter,
                expected_alist,
                resulting_alist,
            )
        }
    }
}
