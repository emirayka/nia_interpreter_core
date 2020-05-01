use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::value::Function;

pub fn value_to_string(
    interpreter: &Interpreter,
    value: Value
) -> Result<String, Error> {
    match value {
        Value::Integer(int) => Ok(int.to_string()),
        Value::Float(float) => Ok(float.to_string()),
        Value::Boolean(boolean) => if boolean {
            Ok(String::from("#t"))
        } else {
            Ok(String::from("#f"))
        },
        Value::String(string_id) => {
            let string = interpreter.get_string(string_id)?;

            Ok(String::from(string.get_string()))
        },
        Value::Symbol(symbol_id) => {
            let string = interpreter.get_symbol_name(symbol_id)?;

            Ok(String::from(string))
        },
        Value::Keyword(keyword_id) => {
            let keyword = interpreter.get_keyword(keyword_id)?;

            let mut string = String::from(":");
            string.push_str(keyword.get_name());

            Ok(string)
        },
        Value::Cons(cons_id) => {
            let values = interpreter.list_to_vec(cons_id)?;

            let mut result = String::new();
            result.push_str("(");

            for value in values {
                result.push_str(&value_to_string(interpreter, value)?);
                result.push_str(" ");
            }

            result.remove(result.len() - 1);

            result.push_str(")");
            Ok(result)
        },
        Value::Object(object_id) => {
            let items = interpreter.get_object_items(object_id)?;

            let mut result = String::new();
            result.push_str("{");

            for (symbol_id, value) in items {
                let mut name = String::from(":");
                name.push_str(interpreter.get_symbol_name(*symbol_id)?);
                let string = value_to_string(interpreter, value.force_get_value())?;

                result.push_str(&name);
                result.push_str(" ");
                result.push_str(&string);
                result.push_str(" ");
            }

            if result.len() > 1 {
                result.remove(result.len() - 1);
            }
            result.push_str("}");

            Ok(result)
        },
        Value::Function(function_id) => {
            let function = interpreter.get_function(function_id)?;

            let string = match function {
                Function::Interpreted(_) => String::from("<function>"),
                Function::Builtin(_) => String::from("<builtin-function>"),
                Function::Macro(_) => String::from("<macro>"),
                Function::SpecialForm(_) => String::from("<special-form>"),
            };

            Ok(string)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_string_representation_of_values() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            ("1",                       "1"),
            ("1.1",                     "1.1"),
            ("#t",                      "#t"),
            ("#f",                      "#f"),
            (r#""string""#,             "string"),
            ("'symbol",                 "symbol"),
            (":keyword",                ":keyword"),
            ("'(a b c)",                "(a b c)"),
            ("{}",                      "{}"),
            ("{:key 'value}",           "{:key value}"),
            ("#(+ %1 %2)",              "<function>"),
            ("(flookup 'flookup)",      "<builtin-function>"),
            ("(function (macro () 1))", "<macro>"),
            ("(flookup 'cond)",         "<special-form>"),
        );

        for (code, expected) in pairs {
            let value = interpreter.execute(code).unwrap();
            let result = value_to_string(&mut interpreter, value).unwrap();

            assert_eq!(expected, result);
        }
    }
}
