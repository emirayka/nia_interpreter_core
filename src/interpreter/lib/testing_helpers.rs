use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;

// i -> Integer
// f -> Float
// b -> Boolean
// s -> String
// y -> Symbol
// k -> Keyword

pub fn make_value_pairs_ifbsyk(interpreter: & mut Interpreter) -> Vec<(String, Value)> {
    let string_value = interpreter.intern_string_value(String::from("string"));
    let keyword_value = interpreter.intern_keyword_value(String::from("keyword"));

    vec!(
        (String::from("1"), Value::Integer(1)),
        (String::from("1.1"), Value::Float(1.1)),
        (String::from("#t"), Value::Boolean(true)),
        (String::from("#f"), Value::Boolean(false)),
        (String::from("\"string\""), string_value),
        (String::from("'symbol"), interpreter.intern_symbol_value("symbol")),
        (String::from(":keyword"), keyword_value),
    )
}

pub fn for_value_pairs_evaluated_ifbsyk<F: Fn(&mut Interpreter, String, Value) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let pairs = make_value_pairs_ifbsyk(&mut interpreter);

    for pair in pairs {
        func(&mut interpreter, pair.0.clone(), pair.1);
    }
}

pub fn for_meta_value_pairs_evaluated_ifbsyk<F: Fn(&mut Interpreter, String, Value, String, Value) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let pairs = make_value_pairs_ifbsyk(&mut interpreter);

    for pair1 in &pairs {
        for pair2 in &pairs {
            func(&mut interpreter, pair1.0.clone(), pair1.1, pair2.0.clone(), pair2.1);
        }
    }
}
