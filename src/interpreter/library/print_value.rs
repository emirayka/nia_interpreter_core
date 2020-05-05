use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn print_value(interpreter: &mut Interpreter, value: Value) {
    let string = match value {
        Value::String(string_id) => {
            let vstring = match interpreter.get_string(string_id) {
                Ok(string) => string,
                _ => panic!("Cannot print value"),
            };

            let mut result = String::from("\"");
            result.push_str(vstring.get_string());
            result.push_str("\"");

            result
        },
        _ => match library::value_to_string(interpreter, value) {
            Ok(string) => string,
            Err(_) => panic!("Cannot print value"),
        },
    };

    println!("{}", string)
}
