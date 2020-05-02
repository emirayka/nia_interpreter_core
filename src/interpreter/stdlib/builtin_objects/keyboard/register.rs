use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

use crate::interpreter::library;

pub fn register(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() < 2 || values.len() > 3 {
        return Error::invalid_argument_count_error(
            "Built-in function `keyboard:register' takes two or three arguments."
        ).into()
    }

    let mut values = values;

    let root_environment_id = interpreter.get_root_environment();
    let symbol_id_registered_keyboards = interpreter.intern("registered-keyboards");

    let keyboard_list = interpreter.lookup_variable(
        root_environment_id,
        symbol_id_registered_keyboards
    )?.ok_or_else(|| Error::generic_execution_error(
        "Cannot find registered_keyboards"
    ))?;

    let path = values.remove(0);
    let name = values.remove(0);

    let modifier_keys = if values.len() != 0 {
        let vec = library::read_as_vector(interpreter, values.remove(0))?;

        interpreter.vec_to_list(vec)
    } else {
        interpreter.intern_nil_symbol_value()
    };

    library::check_value_is_string(interpreter, path)?;
    library::check_value_is_string(interpreter, name)?;

    let new_list = interpreter.vec_to_list(vec!(
        path,
        name,
        modifier_keys
    ));

    let cons = interpreter.make_cons_value(
        new_list,
        keyboard_list
    );

    interpreter.set_variable(
        root_environment_id,
        symbol_id_registered_keyboards,
        cons
    )?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nia_basic_assertions::*;

    use crate::interpreter::library::assertion;

    #[test]
    fn adds_keyboards() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"'()"#, "registered-keyboards"),
            (r#"nil"#, r#"(keyboard:register "/dev/input/event1" "Keyboard 1")"#),
            (r#"(list '("/dev/input/event1" "Keyboard 1" ()))"#, "registered-keyboards"),
            (r#"nil"#, r#"(keyboard:register "/dev/input/event2" "Keyboard 2")"#),
            (r#"(list '("/dev/input/event2" "Keyboard 2" ()) '("/dev/input/event1" "Keyboard 1" ()))"#, "registered-keyboards"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn allows_to_set_modifier_keys() {
        let mut interpreter = Interpreter::new();

        let pairs = vec!(
            (r#"nil"#, r#"(keyboard:register "/dev/input/event1" "Keyboard 1" '("a" "b"))"#),
            (r#"(list '("/dev/input/event1" "Keyboard 1" ("a" "b")))"#, "registered-keyboards"),
        );

        assertion::assert_results_are_equal(
            &mut interpreter,
            pairs
        );
    }

    #[test]
    fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(keyboard:register 1 \"name\")",
            "(keyboard:register 1.1 \"name\")",
            "(keyboard:register #t \"name\")",
            "(keyboard:register #f \"name\")",
            "(keyboard:register :keyword \"name\")",
            "(keyboard:register 'symbol \"name\")",
            "(keyboard:register '(1 2) \"name\")",
            "(keyboard:register {} \"name\")",
            "(keyboard:register #() \"name\")",

            "(keyboard:register \"path\" 1)",
            "(keyboard:register \"path\" 1.1)",
            "(keyboard:register \"path\" #t)",
            "(keyboard:register \"path\" #f)",
            "(keyboard:register \"path\" :keyword)",
            "(keyboard:register \"path\" 'symbol)",
            "(keyboard:register \"path\" '(1 2))",
            "(keyboard:register \"path\" {})",
            "(keyboard:register \"path\" #())",

            "(keyboard:register \"path\" \"name\" 1)",
            "(keyboard:register \"path\" \"name\" 1.1)",
            "(keyboard:register \"path\" \"name\" #t)",
            "(keyboard:register \"path\" \"name\" #f)",
            "(keyboard:register \"path\" \"name\" \"str\")",
            "(keyboard:register \"path\" \"name\" :keyword)",
            "(keyboard:register \"path\" \"name\" 'symbol)",
            "(keyboard:register \"path\" \"name\" {})",
            "(keyboard:register \"path\" \"name\" #())",
        );

        assertion::assert_results_are_invalid_argument_errors(
            &mut interpreter,
            code_vector
        );
    }

    #[test]
    fn returns_invalid_argument_count_error_when_incorrect_count_of_arguments_were_passed() {
        let mut interpreter = Interpreter::new();

        let code_vector = vec!(
            "(keyboard:register)",
            "(keyboard:register \"path\")",
            "(keyboard:register \"path\" \"name\" '() '())",
        );

        assertion::assert_results_are_invalid_argument_count_errors(
            &mut interpreter,
            code_vector
        );
    }
}
