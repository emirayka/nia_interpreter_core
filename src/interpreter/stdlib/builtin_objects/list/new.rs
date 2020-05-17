use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub fn new(
    interpreter: &mut Interpreter,
    _environment: EnvironmentId,
    values: Vec<Value>,
) -> Result<Value, Error> {
    Ok(interpreter.vec_to_list(values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[allow(unused_imports)]
    use crate::utils;

    #[test]
    fn returns_nil_when_was_called_with_zero_arguments() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![("(list:new)", interpreter.intern_nil_symbol_value())];

        utils::assert_results_are_correct(&mut interpreter, pairs);
    }

    #[test]
    fn returns_a_list_of_one_value_when_was_called_with_one_argument() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(list:new 1)", "(cons:new 1 nil)"),
            ("(list:new 1.1)", "(cons:new 1.1 nil)"),
            ("(list:new #t)", "(cons:new #t nil)"),
            ("(list:new #f)", "(cons:new #f nil)"),
            ("(list:new \"string\")", "(cons:new \"string\" nil)"),
            ("(list:new 'symbol)", "(cons:new 'symbol nil)"),
            ("(list:new :keyword)", "(cons:new :keyword nil)"),
            ("(list:new {})", "(cons:new {} nil)"),
            ("(list:new #())", "(cons:new #() nil)"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }

    #[test]
    fn returns_a_list_of_two_values_when_was_called_with_two_arguments() {
        let mut interpreter = Interpreter::new();

        let pairs = vec![
            ("(list:new 1 2)", "(cons:new 1 (cons:new 2 nil))"),
            ("(list:new 1.1 2.2)", "(cons:new 1.1 (cons:new 2.2 nil))"),
            ("(list:new #t #f)", "(cons:new #t (cons:new #f nil))"),
            ("(list:new #f #t)", "(cons:new #f (cons:new #t nil))"),
            (
                "(list:new \"string-1\" \"string-2\")",
                "(cons:new \"string-1\" (cons:new \"string-2\" nil))",
            ),
            (
                "(list:new 'symbol-1 'symbol-2)",
                "(cons:new 'symbol-1 (cons:new 'symbol-2 nil))",
            ),
            (
                "(list:new :keyword-1 :keyword-2)",
                "(cons:new :keyword-1 (cons:new :keyword-2 nil))",
            ),
            (
                "(list:new {:a 1} {:b 2})",
                "(cons:new {:a 1} (cons:new {:b 2} nil))",
            ),
            //            ("(list:new #(+ %1 %2) #(+ %2 %1))", "(cons:new #(+ %1 %2) (cons:new #(+ %2 %1) nil))"),
        ];

        utils::assert_results_are_equal(&mut interpreter, pairs);
    }
}
