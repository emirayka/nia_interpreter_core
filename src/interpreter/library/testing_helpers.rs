use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;

// i -> Integer
// f -> Float
// b -> Boolean
// s -> String
// y -> Symbol
// k -> Keyword
// c -> cons
// o -> object
// u -> function

pub fn make_value_pairs_evaluated_ifbsyko(interpreter: & mut Interpreter) -> Vec<(String, Value)> {
    let string_value = interpreter.intern_string_value(String::from("string"));
    let symbol_value = interpreter.intern_symbol_value("symbol");
    let keyword_value = interpreter.intern_keyword_value(String::from("keyword"));
    let object_value = interpreter.make_object_value();

    vec!(
        (String::from("1"), Value::Integer(1)),
        (String::from("1.1"), Value::Float(1.1)),
        (String::from("#t"), Value::Boolean(true)),
        (String::from("#f"), Value::Boolean(false)),
        (String::from("\"string\""), string_value),
        (String::from("'symbol"), symbol_value),
        (String::from(":keyword"), keyword_value),
        (String::from("{}"), object_value),
    )
}

pub fn for_special_symbols<F: Fn(&mut Interpreter, String) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let special_symbols = vec!(
        "#opt",
        "#rest",
        "#keys"
    );

    for special_symbol in special_symbols {
        func(&mut interpreter, String::from(special_symbol));
    }
}

pub fn for_constants<F: Fn(&mut Interpreter, String) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let constants = vec!(
        "nil"
    );

    for constant in constants {
        func(&mut interpreter, String::from(constant));
    }
}

pub fn for_meta_value_pairs_evaluated_ifbsyko<F: Fn(&mut Interpreter, String, Value, String, Value) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let pairs = make_value_pairs_evaluated_ifbsyko(&mut interpreter);

    for pair1 in &pairs {
        for pair2 in &pairs {
            func(&mut interpreter, pair1.0.clone(), pair1.1, pair2.0.clone(), pair2.1);
        }
    }
}