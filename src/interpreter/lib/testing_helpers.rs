use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::environment_arena::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::function::interpreted_function::InterpretedFunction;
use crate::interpreter::function::Function;

// i -> Integer
// f -> Float
// b -> Boolean
// s -> String
// y -> Symbol
// k -> Keyword
// c -> cons
// o -> object
// u -> function

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

pub fn make_value_pairs_ifbsykcou(interpreter: & mut Interpreter) -> Vec<(String, Value)> {
    let string_value = interpreter.intern_string_value(String::from("string"));
    let symbol_value = interpreter.intern_symbol_value("symbol");
    let keyword_value = interpreter.intern_keyword_value(String::from("keyword"));
    let cons_value = interpreter.make_cons_value(Value::Integer(1), Value::Integer(2));
    let object_value = interpreter.make_object_value();
    let function_value = Value::Function(interpreter.register_function(Function::Interpreted(InterpretedFunction::new(
        interpreter.get_root_environment(),
        vec!(),
        vec!(
            Value::Integer(1)
        )
    ))));

    vec!(
        (String::from("1"), Value::Integer(1)),
        (String::from("1.1"), Value::Float(1.1)),
        (String::from("#t"), Value::Boolean(true)),
        (String::from("#f"), Value::Boolean(false)),
        (String::from("\"string\""), string_value),
        (String::from("'symbol"), symbol_value),
        (String::from(":keyword"), keyword_value),
        (String::from("(cons 1 2)"), cons_value),
        (String::from("{}"), object_value),
        (String::from("(function (lambda () 1))"), function_value),
    )
}

pub fn for_value_pairs<F: Fn(&mut Interpreter, String, Value) -> ()>(
    func: F,
    interpreter: &mut Interpreter,
    pairs: Vec<(String, Value)>
) {
    for pair in pairs {
        func(interpreter, pair.0.clone(), pair.1);
    }
}

pub fn for_value_pairs_evaluated_ifbsyk<F: Fn(&mut Interpreter, String, Value) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let pairs = make_value_pairs_ifbsyk(&mut interpreter);

    for_value_pairs(
        func,
        &mut interpreter,
        pairs
    )
}


pub fn for_value_pairs_evaluated_ifbsykcou<F: Fn(&mut Interpreter, String, Value) -> ()>(
    func: F
) {
    let mut interpreter = Interpreter::new();
    let pairs = make_value_pairs_ifbsykcou(&mut interpreter);

    for_value_pairs(
        func,
        &mut interpreter,
        pairs
    )
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
