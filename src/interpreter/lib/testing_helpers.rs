use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value::Integer;

// i -> Integer
// f -> Float
// b -> Boolean
// s -> String
// y -> Symbol
// k -> Keyword

fn make_value_pairs_ifbsyk(interpreter: & mut Interpreter) -> Vec<(String, Value)> {
    vec!(
        (String::from("1"), Value::Integer(1)),
        (String::from("1.1"), Value::Float(1.1)),
        (String::from("#t"), Value::Boolean(true)),
        (String::from("#f"), Value::Boolean(false)),
        (String::from("\"string\""), Value::String(String::from("string"))),
        (String::from("'symbol"), interpreter.intern("symbol")),
        (String::from(":keyword"), Value::Keyword(String::from("keyword"))),
    )
}

pub fn for_value_pairs_evaluated_ifbsyk<F: Fn(&mut Interpreter, String, Value) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let pairs = make_value_pairs_ifbsyk(&mut interpreter);

    for pair in pairs {
        func(&mut interpreter, pair.0.clone(), pair.1.clone());
    }
}

pub fn for_meta_value_pairs_evaluated_ifbsyk<F: Fn(&mut Interpreter, String, Value, String, Value) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let pairs = make_value_pairs_ifbsyk(&mut interpreter);

    for pair1 in &pairs {
        for pair2 in &pairs {
            func(&mut interpreter, pair1.0.clone(), pair1.1.clone(), pair2.0.clone(), pair2.1.clone());
        }
    }
}
