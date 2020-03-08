use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::function::arguments::{Arguments, KeyArgument, OptionalArgument};
use crate::interpreter::error::Error;
use crate::interpreter::function::Function;
use crate::interpreter::value::Value;

fn deep_equal_option_values(
    interpreter: &Interpreter,
    option_value1: Option<Value>,
    option_value2: Option<Value>
) -> Result<bool, Error> {
    match (option_value1, option_value2) {
        (Some(v1), Some(v2)) => {
            deep_equal(interpreter, v1, v2)
        },
        (None, None) => Ok(true),
        _ => Ok(false)
    }
}

fn deep_equal_key_argument(
    interpreter: &Interpreter,
    key_argument1: &KeyArgument,
    key_argument2: &KeyArgument
) -> Result<bool, Error> {
    if key_argument1.get_name() != key_argument2.get_name() {
        return Ok(false)
    }

    if key_argument1.get_provided() != key_argument2.get_provided() {
        return Ok(false)
    }

    deep_equal_option_values(
        interpreter,
        key_argument1.get_default(),
        key_argument2.get_default(),
    )
}

fn deep_equal_key_arguments(
    interpreter: &Interpreter,
    key_arguments1: &Vec<KeyArgument>,
    key_arguments2: &Vec<KeyArgument>
) -> Result<bool, Error> {
    if key_arguments1.len() != key_arguments2.len() {
        return Ok(false)
    }

    let iterator1 = key_arguments1.iter();
    let iterator2 = key_arguments2.iter();
    let iterator = iterator1.zip(iterator2);

    for (key_argument1, key_argument2) in iterator {
        if !deep_equal_key_argument(
            interpreter,
            key_argument1,
            key_argument2
        )? {
            return Ok(false)
        }
    }

    return Ok(true)
}

fn deep_equal_optional_argument(
    interpreter: &Interpreter,
    optional_argument1: &OptionalArgument,
    optional_argument2: &OptionalArgument
) -> Result<bool, Error> {
    if !deep_equal_option_values(
        interpreter,
        optional_argument1.get_default(),
        optional_argument2.get_default()
    )? {
        return Ok(false)
    }

    if optional_argument1.get_provided() != optional_argument2.get_provided() {
        return Ok(false)
    }

    if optional_argument1.get_name() != optional_argument2.get_name() {
        return Ok(false)
    }

    Ok(true)
}

fn deep_equal_optional_arguments(
    interpreter: &Interpreter,
    optional_arguments1: &Vec<OptionalArgument>,
    optional_arguments2: &Vec<OptionalArgument>
) -> Result<bool, Error> {
    if optional_arguments1.len() != optional_arguments2.len() {
        return Ok(false)
    }

    let iterator1 = optional_arguments1.iter();
    let iterator2 = optional_arguments2.iter();
    let iterator = iterator1.zip(iterator2);

    for (optional_argument1, optional_argument2) in iterator {
        if !deep_equal_optional_argument(
            interpreter,
            optional_argument1,
            optional_argument2
        )? {
            return Ok(false)
        }
    }

    return Ok(true)
}

fn deep_equal_arguments(
    interpreter: &Interpreter,
    arguments1: &Arguments,
    arguments2: &Arguments
) -> Result<bool, Error> {
    if arguments1.get_ordinary_arguments() != arguments2.get_ordinary_arguments() {
        return Ok(false)
    }

    if !deep_equal_optional_arguments(
        interpreter,
        arguments1.get_optional_arguments(),
        arguments2.get_optional_arguments(),
    )? {
        return Ok(false)
    }

    if arguments1.get_rest_argument() != arguments2.get_rest_argument() {
        return Ok(false)
    }

    if !deep_equal_key_arguments(
        interpreter,
        arguments1.get_key_arguments(),
        arguments2.get_key_arguments(),
    )? {
        return Ok(false)
    }

    return Ok(true)
}

fn deep_equal_code(
    interpreter: &Interpreter,
    code1: &Vec<Value>,
    code2: &Vec<Value>
) -> Result<bool, Error> {
    if code1.len() != code2.len() {
        return Ok(false)
    }

    let iterator1 = code1.iter();
    let iterator2 = code2.iter();
    let iterator = iterator1.zip(iterator2);

    for (value1, value2) in iterator {
        if !deep_equal(
            interpreter,
            *value1,
            *value2
        )? {
            return Ok(false)
        }
    }

    return Ok(true)
}

fn deep_equal_function(
    interpreter: &Interpreter,
    function1: &Function,
    function2: &Function
) -> Result<bool, Error> {
    match (function1, function2) {
        (Function::Interpreted(b1), Function::Interpreted(b2)) => {
            if b1.get_environment() != b2.get_environment() {
                return Ok(false)
            }

            if !deep_equal_arguments(
                interpreter,
                b1.get_arguments(),
                b2.get_arguments()
            )? {
                return Ok(false)
            }

            if !deep_equal_code(
                interpreter,
                b1.get_code(),
                b2.get_code()
            )? {
                return Ok(false)
            }

            Ok(true)
        },
        (Function::Builtin(b1), Function::Builtin(b2)) => {
            return Ok(b1 == b2)
        },
        (Function::Macro(b1), Function::Macro(b2)) => {
            if b1.get_environment() != b2.get_environment() {
                return Ok(false)
            }

            if !deep_equal_arguments(
                interpreter,
                b1.get_arguments(),
                b2.get_arguments()
            )? {
                return Ok(false)
            }

            if !deep_equal_code(
                interpreter,
                b1.get_code(),
                b2.get_code()
            )? {
                return Ok(false)
            }

            Ok(true)
        },
        (Function::SpecialForm(b1), Function::SpecialForm(b2)) => {
            return Ok(b1 == b2)
        },
        _ => Ok(false)
    }
}

pub fn deep_equal(interpreter: &Interpreter, value1: Value, value2: Value) -> Result<bool, Error> {
    use crate::interpreter::value::Value::*;

    match (value1, value2) {
        (Integer(val1), Integer(val2)) => Ok(val1 == val2),
        (Float(val1), Float(val2)) => Ok(val1 == val2),
        (Boolean(val1), Boolean(val2)) => Ok(val1 == val2),
        (Keyword(val1), Keyword(val2)) => Ok(val1 == val2),
        (Symbol(val1), Symbol(val2)) => Ok(val1 == val2),
        (String(val1), String(val2)) => {
            let string1 = interpreter.get_string(val1)?;
            let string2 = interpreter.get_string(val2)?;

            Ok(string1 == string2)
        },
        (Cons(val1), Cons(val2)) => {
            let car1 = interpreter.get_car(val1)?;
            let car2 = interpreter.get_car(val2)?;

            let cdr1 = interpreter.get_cdr(val1)?;
            let cdr2 = interpreter.get_cdr(val2)?;

            let car_equals = deep_equal(&interpreter, car1, car2)?;
            let cdr_equals = deep_equal(&interpreter, cdr1, cdr2)?;

            Ok(car_equals && cdr_equals)
        },
        (Object(object1_id), Object(object2_id)) => {
            let object1_items = interpreter.get_items(object1_id)?;
            let object2_items = interpreter.get_items(object2_id)?;

            if object1_items.len() != object2_items.len() {
                return Ok(false)
            }

            for (item_symbol, value1) in object1_items.iter() {
                let result = match object2_items.get(item_symbol) {
                    Some(value2) => deep_equal(interpreter, *value1, *value2),
                    None => return Ok(false)
                };

                match result {
                    Ok(false) => return Ok(false),
                    Err(error) => return Err(error),
                    _ => {}
                }
            }

            Ok(true)
        }
        (Function(val1), Function(val2)) => {
            let function_1 = interpreter.get_function(val1)?;
            let function_2 = interpreter.get_function(val2)?;

            let result = deep_equal_function(
                interpreter,
                function_1,
                function_2
            )?;

            Ok(result)
        },
        _ => Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_values_are_deeply_equal(interpreter: &mut Interpreter, pairs: Vec<(&str, &str)>) {
        for pair in pairs {
            let value1 = interpreter.execute(pair.0).unwrap();
            let value2 = interpreter.execute(pair.1).unwrap();

            if !deep_equal(interpreter, value1, value2).unwrap() {
                panic!();
            }
        }
    }

    fn assert_values_are_not_deeply_equal(interpreter: &mut Interpreter, pairs: Vec<(&str, &str)>) {
        for pair in pairs {
            let value1 = interpreter.execute(pair.0).unwrap();
            let value2 = interpreter.execute(pair.1).unwrap();

            if deep_equal(interpreter, value1, value2).unwrap() {
                panic!();
            }
        }
    }

    #[test]
    fn returns_true_when_values_are_equal() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("1", "1"),
            ("1.1", "1.1"),
            ("#t", "#t"),
            ("#f", "#f"),
            ("\"string\"", "\"string\""),
            ("'symbol", "'symbol"),
            (":keyword", ":keyword"),
            ("{:a 1}", "{:a 1}"),
            ("'(1 2)", "'(1 2)"),
            ("#(+ %1 %2)", "#(+ %1 %2)"),
        );

        assert_values_are_deeply_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_values_are_not_equal() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("1", "2"),
            ("1.1", "1.2"),
            ("#t", "#f"),
            ("#f", "#t"),
            ("\"string-1\"", "\"string-2\""),
            ("'symbol-1", "'symbol-2"),
            (":keyword-1", ":keyword-2"),
            ("{:a 1}", "{:a 2}"),
            ("'(1 2)", "'(1 3)"),
            ("#(+ %1 %2)", "#(+ %1 %3)"),
        );

        assert_values_are_not_deeply_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_true_when_functions_are_equal() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(function (lambda () 1))", "(function (lambda () 1))"),
            ("(function (lambda (#opt a) 1))", "(function (lambda (#opt a) 1))"),
            ("(function (lambda (#opt (a 1)) 1))", "(function (lambda (#opt (a 1)) 1))"),
            ("(function (lambda (#opt (a 1 a?)) 1))", "(function (lambda (#opt (a 1 a?)) 1))"),
            ("(function (lambda (#opt a #rest b) 1))", "(function (lambda (#opt a #rest b) 1))"),
            ("(function (lambda (#rest b) 1))", "(function (lambda (#rest b) 1))"),
            ("(function (lambda (#keys a) 1))", "(function (lambda (#keys a) 1))"),
            ("(function (lambda (#keys (a 1)) 1))", "(function (lambda (#keys (a 1)) 1))"),
            ("(function (lambda (#keys (a 1 a?)) 1))", "(function (lambda (#keys (a 1 a?)) 1))"),

            ("(function (lambda (c) 1))", "(function (lambda (c) 1))"),
            ("(function (lambda (c #opt a) 1))", "(function (lambda (c #opt a) 1))"),
            ("(function (lambda (c #opt (a 1)) 1))", "(function (lambda (c #opt (a 1)) 1))"),
            ("(function (lambda (c #opt (a 1 a?)) 1))", "(function (lambda (c #opt (a 1 a?)) 1))"),
            ("(function (lambda (c #opt a #rest b) 1))", "(function (lambda (c #opt a #rest b) 1))"),
            ("(function (lambda (c #rest b) 1))", "(function (lambda (c #rest b) 1))"),
            ("(function (lambda (c #keys a) 1))", "(function (lambda (c #keys a) 1))"),
            ("(function (lambda (c #keys (a 1)) 1))", "(function (lambda (c #keys (a 1)) 1))"),
            ("(function (lambda (c #keys (a 1 a?)) 1))", "(function (lambda (c #keys (a 1 a?)) 1))"),
        );

        assert_values_are_deeply_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_argument_names_are_not_equal() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(function (lambda (#opt a) 1))", "(function (lambda (#opt aa) 1))"),
            ("(function (lambda (#opt (a 1)) 1))", "(function (lambda (#opt (aa 1)) 1))"),
            ("(function (lambda (#opt (a 1 a?)) 1))", "(function (lambda (#opt (aa 1 a?)) 1))"),
            ("(function (lambda (#opt (a 1 a?)) 1))", "(function (lambda (#opt (a 1 aa?)) 1))"),
            ("(function (lambda (#opt a #rest b) 1))", "(function (lambda (#opt aa #rest b) 1))"),
            ("(function (lambda (#opt a #rest b) 1))", "(function (lambda (#opt a #rest bb) 1))"),
            ("(function (lambda (#rest b) 1))", "(function (lambda (#rest bb) 1))"),
            ("(function (lambda (#keys a) 1))", "(function (lambda (#keys aa) 1))"),
            ("(function (lambda (#keys (a 1)) 1))", "(function (lambda (#keys (aa 1)) 1))"),
            ("(function (lambda (#keys (a 1 a?)) 1))", "(function (lambda (#keys (aa 1 a?)) 1))"),
            ("(function (lambda (#keys (a 1 a?)) 1))", "(function (lambda (#keys (a 1 aa?)) 1))"),

            ("(function (lambda (c) 1))", "(function (lambda (cc) 1))"),
            ("(function (lambda (c #opt a) 1))", "(function (lambda (cc #opt a) 1))"),
            ("(function (lambda (c #opt (a 1)) 1))", "(function (lambda (cc #opt (a 1)) 1))"),
            ("(function (lambda (c #opt (a 1 a?)) 1))", "(function (lambda (cc #opt (a 1 a?)) 1))"),
            ("(function (lambda (c #opt a #rest b) 1))", "(function (lambda (cc #opt a #rest b) 1))"),
            ("(function (lambda (c #rest b) 1))", "(function (lambda (cc #rest b) 1))"),
            ("(function (lambda (c #keys a) 1))", "(function (lambda (cc #keys a) 1))"),
            ("(function (lambda (c #keys (a 1)) 1))", "(function (lambda (cc #keys (a 1)) 1))"),
            ("(function (lambda (c #keys (a 1 a?)) 1))", "(function (lambda (cc #keys (a 1 a?)) 1))"),
        );

        assert_values_are_not_deeply_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_argument_count_are_not_equal() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(function (lambda () 1))", "(function (lambda (a) 1))"),
            ("(function (lambda (#opt a) 1))", "(function (lambda () 1))"),
            ("(function (lambda (#rest b) 1))", "(function (lambda () 1))"),
            ("(function (lambda (#keys a) 1))", "(function (lambda () 1))"),
        );

        assert_values_are_not_deeply_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_function_code_are_not_equal() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(function (lambda () 1))", "(function (lambda () 11))"),
            ("(function (lambda (#opt a) 1))", "(function (lambda (#opt a) 11))"),
            ("(function (lambda (#opt (a 1)) 1))", "(function (lambda (#opt (a 1)) 11))"),
            ("(function (lambda (#opt (a 1 a?)) 1))", "(function (lambda (#opt (a 1 a?)) 11))"),
            ("(function (lambda (#opt a #rest b) 1))", "(function (lambda (#opt a #rest b) 11))"),
            ("(function (lambda (#rest b) 1))", "(function (lambda (#rest b) 11))"),
            ("(function (lambda (#keys a) 1))", "(function (lambda (#keys a) 11))"),
            ("(function (lambda (#keys (a 1)) 1))", "(function (lambda (#keys (a 1)) 11))"),
            ("(function (lambda (#keys (a 1 a?)) 1))", "(function (lambda (#keys (a 1 a?)) 11))"),

            ("(function (lambda (c) 1))", "(function (lambda (c) 11))"),
            ("(function (lambda (c #opt a) 1))", "(function (lambda (c #opt a) 11))"),
            ("(function (lambda (c #opt (a 1)) 1))", "(function (lambda (c #opt (a 1)) 11))"),
            ("(function (lambda (c #opt (a 1 a?)) 1))", "(function (lambda (c #opt (a 1 a?)) 11))"),
            ("(function (lambda (c #opt a #rest b) 1))", "(function (lambda (c #opt a #rest b) 11))"),
            ("(function (lambda (c #rest b) 1))", "(function (lambda (c #rest b) 11))"),
            ("(function (lambda (c #keys a) 1))", "(function (lambda (c #keys a) 11))"),
            ("(function (lambda (c #keys (a 1)) 1))", "(function (lambda (c #keys (a 1)) 11))"),
            ("(function (lambda (c #keys (a 1 a?)) 1))", "(function (lambda (c #keys (a 1 a?)) 11))"),
        );

        assert_values_are_not_deeply_equal(
            &mut interpreter,
            pairs
        )
    }

    #[test]
    fn returns_false_when_was_defined_in_different_environments() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("(let () (function (lambda () 1)))", "(let () (function (lambda () 11)))"),
        );

        assert_values_are_not_deeply_equal(
            &mut interpreter,
            pairs
        )
    }
}
