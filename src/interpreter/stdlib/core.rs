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

        defv,
        defn,
        defm,

        _if,
        when,
        unless,

        empty
    );

    Ok(())
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

mod _if {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(
            "(define-function if (function (macro (condition then-clause else-clause) (list 'cond (list condition then-clause) (list #t else-clause)))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn works_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("(defv a 1) (defv b 2) (list (if #t a b) (if #f a b))", "'(1 2)"),
                ("(defv c 0) (defv d 0) (list (if #t (set! c (inc c)) (set! d (inc d))) (if #f (set! c (inc c)) (set! d (inc d)))) (list c d)", "'(1 1)"),
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs
            );
        }
    }
}

mod when {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(
            "(define-function when (function (macro (condition then-clause) (list 'cond (list condition then-clause)))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn works_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("(defv a 0) (defv b 0) (list (when #t (set! a (inc a))) (when #f (set! b (inc b)))) (list a b)", "'(1 0)"),
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs
            );
        }
    }
}

mod unless {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(
            "(define-function unless (function (macro (condition else-clause) (list 'cond (list (list 'not condition) else-clause)))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn works_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("(defv a 0) (defv b 0) (list (unless #t (set! a (inc a))) (unless #f (set! b (inc b)))) (list a b)", "'(0 1)"),
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

    // I walk a million miles and get nowhere
    // Against the cold rain
    // Torn asunder
    // Clutching at the withered strands of life
    //
    // Dreams splintered in two
    // Replaced by the endless drone
    // Of broken spirits and open wounds
    // Emptiness deeply sown
    //
    // Distant voices echo in a sea of beings
    // Within it's ebb. Half forgotten memories
    // An Unbroken Moment

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
