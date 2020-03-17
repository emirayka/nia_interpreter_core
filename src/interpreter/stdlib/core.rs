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

        _fn,

        _if,
        when,
        unless,

        func__bind,

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

mod _fn {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(
            "(defm fn (#rest args) (list 'function (cons 'lambda args)))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn makes_a_function() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("(fn () 1)", "(function (lambda () 1))"),

                ("(fn (a) (+ a a))", "(function (lambda (a) (+ a a)))"),
                ("(fn (a #opt args) args)", "(function (lambda (a #opt args) args))"),
                ("(fn (a #rest args) args)", "(function (lambda (a #rest args) args))"),
                ("(fn (a #keys args) args)", "(function (lambda (a #keys args) args))"),
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
"(defm if (condition then-clause else-clause)\
  (list 'cond (list condition then-clause) (list #t else-clause)))"
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
"(defm when (condition then-clause)\
  (list 'cond (list condition then-clause)))"
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

    // The emptiness that we confess
    // In the dimmest hour of day
    // In the common town they make a sound
    // Like the low sad moan of prey

    // The bitter taste the hidden face
    // Of the lost forgotten child
    // The darkest need the slowest speed
    // The debt unreconciled

    // These photographs mean nothing
    // To the poison that they take
    // Before a moment's glory
    // The light begins to fade

    // The awful cost of all we lost
    // As we looked the other way
    // We've paid the price of this cruel device
    // Till we've nothing left to pay

    // The river goes where the current flows
    // The light we must destroy
    // Events conspire to set afire
    // The methods we employ

    // These dead men walk on water
    // Cold blood runs through their veins
    // The angry river rises
    // As we step into the rain

    // These photographs mean nothing
    // To the poison that they take
    // The angry river rises
    // As we step into the rain

    // The Hat - The Angry River

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(
"(defm unless (condition else-clause)\
  (list 'cond (list (list 'not condition) else-clause)))"
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

mod func__bind {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(r#"
(object:set! func 'bind (fn (f #rest args)
  (unless (or (is:interpreted? f) (is:builtin? f))
    (throw invalid-argument-error "Function `func:bind' binds only functions."))

  (fn (#rest other-args)
    (func:apply f (list:join args other-args)))))"#)?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;
        use crate::interpreter::interpreter::Interpreter;

        #[test]
        fn calls_a_function_with_provided_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("((func:bind #(+ 1  2  3)))",        "6"),

                ("((func:bind #(+ %1 2  3)) 1)",      "6"),
                ("((func:bind #(+ %1 2  3) 1))",      "6"),

                ("((func:bind #(+ %1 %2 3)) 1 2)",    "6"),
                ("((func:bind #(+ %1 %2 3) 1) 2)",    "6"),
                ("((func:bind #(+ %1 %2 3) 1 2))",    "6"),

                ("((func:bind #(+ %1 %2 %3)) 1 2 3)", "6"),
                ("((func:bind #(+ %1 %2 %3) 1) 2 3)", "6"),
                ("((func:bind #(+ %1 %2 %3) 1 2) 3)", "6"),
                ("((func:bind #(+ %1 %2 %3) 1 2 3))", "6"),
            );

            assertion::assert_results_are_equal(
                &mut interpreter,
                pairs
            );
        }

        #[test]
        fn returns_invalid_argument_error_when_macro_or_special_form_was_provided() {
            let mut interpreter = Interpreter::new();

            let code_vector = vec!(
                "(func:bind (function (macro () 1)) '())",
                "(func:bind (flookup 'cond) '())",
            );

            assertion::assert_results_are_invalid_argument_errors(
                &mut interpreter,
                code_vector
            );
        }

        #[test]
        fn returns_invalid_argument_error_when_invalid_arguments_were_passed() {
            let mut interpreter = Interpreter::new();

            let code_vector = vec!(
                "(func:bind 1)",
                "(func:bind 1.1)",
                "(func:bind #t)",
                "(func:bind #f)",
                "(func:bind \"string\")",
                "(func:bind 'symbol)",
                "(func:bind :keyword)",
                "(func:bind '(1 2 3))",
                "(func:bind {})",
            );

            assertion::assert_results_are_invalid_argument_errors(
                &mut interpreter,
                code_vector
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

    // Dreams splintered in two
    // Replaced by the endless drone
    // Of broken spirits and open wounds
    // Emptiness deeply sown

    // Distant voices echo in a sea of beings
    // Within it's ebb. Half forgotten memories
    // An Unbroken Moment

    // Woods of Desolation - An Unbroken Moment

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
