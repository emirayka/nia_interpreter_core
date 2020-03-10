use crate::interpreter::value::Value;
use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

macro_rules! infect {
    ($interpreter:ident, $($module:tt),*) => (
        $(
            $module::infect($interpreter)?;
        )*
    );
}

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    infect!(
        interpreter,

        nil,
        defv,
        defn,
        defm,

        empty
    );

    Ok(())
}

mod nil {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        let root = interpreter.get_root_environment();
        let nil_value = interpreter.intern_nil_symbol_value();

        if let Value::Symbol(symbol_id) = nil_value {
            interpreter.define_variable(root, symbol_id, nil_value)?;
        } else {
            panic!()
        }

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn defined_to_nil() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("nil", "'()")
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs
            );
        }
    }
}

mod defv {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(
            "(define-function defv (function (macro (name #opt (value nil)) (list 'define-variable name value))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn defines_variable() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("(defv a 1) a", "1"),
                ("(defv b) b", "nil")
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs
            );
        }
    }
}

mod defn {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(
            "(define-function defn (function (macro (name #rest params) (list 'define-function name (list 'function (cons 'lambda params))))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn defines_function() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("(defn a () 1) (a)", "1"),
                ("(defn b (a) a) (b 2)", "2"),

                ("(defn c (#opt b c) (list b c)) (c)", "'(nil nil)"),
                ("(defn d (#opt b c) (list b c)) (d 2)", "'(2 nil)"),
                ("(defn e (#opt b c) (list b c)) (e 2 3)", "'(2 3)"),

                ("(defn f (#rest b) b) (f 2 3 4)", "'(2 3 4)"),
                ("(defn g (#keys b) b) (g :b 1)", "1"),
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs
            );
        }
    }
}

mod defm {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(
            "(define-function defm (function (macro (name #rest params) (list 'define-function name (list 'function (cons 'macro params))))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn defines_macro() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("(defm a () 1) (a)", "1"),
                ("(defm b (a) a) (b 2)", "2"),

                ("(defm c (#opt b c) (list 'list b c)) (c)", "'(nil nil)"),
                ("(defm d (#opt b c) (list 'list b c)) (d 2)", "'(2 nil)"),
                ("(defm e (#opt b c) (list 'list b c)) (e 2 3)", "'(2 3)"),

                ("(defm f (#rest b) (list 'quote b)) (f 2 3 4)", "'(2 3 4)"),
                ("(defm g (#keys b) b) (g :b 1)", "1"),

                ("(defm h (a b) (list 'cons a b)) (h 'a 'b)", "(cons 'a 'b)")
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs
            );
        }
    }
}

mod empty {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn does_nothing() {
        }
    }
}
