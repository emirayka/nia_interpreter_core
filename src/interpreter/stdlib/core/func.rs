use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let functions: Vec<fn(&mut Interpreter) -> Result<(), Error>> = vec!(
        func__bind::infect,
        func__curry::infect,
        func__curry_star::infect,
    );

    for function in functions {
        function(interpreter)?;
    }

    Ok(())
}

#[allow(non_snake_case)]
mod func__bind {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(r#"
(object:set! func 'bind (fn (f #rest args)
  (unless (or (is:interpreted? f) (is:builtin? f))
    (throw 'invalid-argument-error "Function `func:bind' binds only functions."))

  (fn (#rest other-args)
    (func:apply f (list:join args other-args)))))"#)?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

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

                ("((func:bind (fn (#rest a) (func:apply #'+ a)) 1 2 3))", "6"),
                ("((func:bind (fn (#rest a) (func:apply #'+ a)) 1 2 3) 4 5)", "15"),

                ("((func:bind (fn (#opt (a 0) (b 0) (c 0)) (+ a b c))))", "0"),
                ("((func:bind (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 1))", "1"),
                ("((func:bind (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 1 2))", "3"),
                ("((func:bind (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 1 2 3))", "6"),

                ("((func:bind (fn (#opt (a 0) (b 0) (c 0)) (+ a b c))) 1)", "1"),
                ("((func:bind (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 1) 2)", "3"),
                ("((func:bind (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 1 2) 3)", "6"),
                ("((func:bind (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 1 2 3))", "6"),

                ("((func:bind (fn (#keys (a 0) (b 0) (c 0)) (+ a b c))))", "0"),
                ("((func:bind (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) :a 1))", "1"),
                ("((func:bind (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) :a 1 :b 2))", "3"),
                ("((func:bind (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) :a 1 :b 2 :c 3))", "6"),

                ("((func:bind (fn (#keys (a 0) (b 0) (c 0)) (+ a b c))) :a 1)", "1"),
                ("((func:bind (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) :a 1) :b 2)", "3"),
                ("((func:bind (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) :a 1 :b 2) :c 3)", "6"),
                ("((func:bind (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) :a 1 :b 2 :c 3))", "6"),
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

#[allow(non_snake_case)]
mod func__curry {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(r#"
(object:set! func 'curry (fn (f n)
  (unless (or (is:interpreted? f) (is:builtin? f))
    (throw 'invalid-argument-error "Function `func:curry' takes only functions as its first argument."))

  (unless (and (is:int? n) (is:positive? n))
    (throw 'invalid-argument-error "Function `func:curry' takes only positive integers as its second argument."))

  (fn (#rest args)
    (cond ((< (list:length args) n)
           (func:curry (fn (#rest other-args) (func:apply f (list:join args other-args)))
                       (- n (list:length args))))
          ((= (list:length args) n)
           (func:apply f args))
          ((> (list:length args) n)
           (throw 'invalid-argument-error))))))
           "#)?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn calls_a_function_with_provided_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("((func:curry #(+ %1 %2 %3) 3) 1 2 3)", "6"),
                ("(((func:curry #(+ %1 %2 %3) 3) 1) 2 3)", "6"),
                ("((((func:curry #(+ %1 %2 %3) 3) 1) 2) 3)", "6"),
                ("(((func:curry #(+ %1 %2 %3) 3) 1 2) 3)", "6"),

                ("((func:curry (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 1) 1)", "1"),
                ("((func:curry (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 2) 1 2)", "3"),
                ("(((func:curry (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 2) 1) 2)", "3"),
                ("((func:curry (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 3) 1 2 3)", "6"),
                ("(((func:curry (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 3) 1) 2 3)", "6"),
                ("((((func:curry (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 3) 1) 2) 3)", "6"),
                ("(((func:curry (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 3) 1 2) 3)", "6"),

                ("((func:curry (fn (#rest a) (func:apply #'+ a)) 3) 1 2 3)", "6"),
                ("(((func:curry (fn (#rest a) (func:apply #'+ a)) 3) 1) 2 3)", "6"),
                ("((((func:curry (fn (#rest a) (func:apply #'+ a)) 3) 1) 2) 3)", "6"),
                ("(((func:curry (fn (#rest a) (func:apply #'+ a)) 3) 1 2) 3)", "6"),

                ("((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 2) :a 1)", "1"),
                ("(((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 2) :a) 1)", "1"),

                ("((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 4) :a 1 :b 2)", "3"),
                ("(((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 4) :a) 1 :b 2)", "3"),
                ("((((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 4) :a) 1) :b 2)", "3"),
                ("(((((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 4) :a) 1) :b) 2)", "3"),

                ("((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a 1 :b 2 :c 3)", "6"),
                ("(((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1 :b 2 :c 3)", "6"),
                ("((((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1) :b 2 :c 3)", "6"),
                ("(((((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1) :b) 2 :c 3)", "6"),
                ("((((((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1) :b) 2) :c 3)", "6"),
                ("(((((((func:curry (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1) :b) 2) :c) 3)", "6"),
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
                "(func:curry (function (macro () 1)) 2)",
                "(func:curry (flookup 'cond) 2)",
            );

            assertion::assert_results_are_invalid_argument_errors(
                &mut interpreter,
                code_vector
            );
        }

        #[test]
        fn returns_invalid_argument_error_when_curried_function_was_called_with_too_many_arguments() {
            let mut interpreter = Interpreter::new();

            let code_vector = vec!(
                "((func:curry (fn (#rest args) (func:apply #'+ args)) 3) 1 2 3 4)",
                "(((func:curry (fn (#rest args) (func:apply #'+ args)) 3) 1) 2 3 4)",
                "((((func:curry (fn (#rest args) (func:apply #'+ args)) 3) 1) 2) 3 4)",
                "(((func:curry (fn (#rest args) (func:apply #'+ args)) 3) 1 2) 3 4)",
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
                "(func:curry 1 2)",
                "(func:curry 1.1 2)",
                "(func:curry #t 2)",
                "(func:curry #f 2)",
                "(func:curry \"string\" 2)",
                "(func:curry 'symbol 2)",
                "(func:curry :keyword 2)",
                "(func:curry '(1 2 3) 2)",
                "(func:curry {} 2)",

                "(func:curry #(+ %1 %2) 1.1)",
                "(func:curry #(+ %1 %2) #t)",
                "(func:curry #(+ %1 %2) #f)",
                "(func:curry #(+ %1 %2) \"string\")",
                "(func:curry #(+ %1 %2) 'symbol)",
                "(func:curry #(+ %1 %2) :keyword)",
                "(func:curry #(+ %1 %2) '(1 2 3))",
                "(func:curry #(+ %1 %2) {})",
                "(func:curry #(+ %1 %2) #{})",
            );

            assertion::assert_results_are_invalid_argument_errors(
                &mut interpreter,
                code_vector
            );
        }
    }
}

#[allow(non_snake_case)]
mod func__curry_star {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(r#"
(object:set! func 'curry* (fn (f n)
  (unless (or (is:interpreted? f) (is:builtin? f))
    (throw 'invalid-argument-error "Function `func:curry*' takes only functions as its first argument."))

  (unless (and (is:int? n) (is:positive? n))
    (throw 'invalid-argument-error "Function `func:curry*' takes only positive integers as its second argument."))

  (fn (#rest args)
    (cond ((< (list:length args) n)
           (func:curry* (fn (#rest other-args) (func:apply f (list:join args other-args)))
                        (- n (list:length args))))
          (#t
           (func:apply f args))))))
           "#)?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::interpreter::library::assertion;

        #[test]
        fn calls_a_function_with_provided_arguments() {
            let mut interpreter = Interpreter::new();

            let pairs = vec!(
                ("((func:curry* #(+ %1 %2 %3) 3) 1 2 3)", "6"),
                ("(((func:curry* #(+ %1 %2 %3) 3) 1) 2 3)", "6"),
                ("((((func:curry* #(+ %1 %2 %3) 3) 1) 2) 3)", "6"),
                ("(((func:curry* #(+ %1 %2 %3) 3) 1 2) 3)", "6"),

                ("((func:curry* (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 1) 1)", "1"),
                ("((func:curry* (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 2) 1 2)", "3"),
                ("(((func:curry* (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 2) 1) 2)", "3"),
                ("((func:curry* (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 3) 1 2 3)", "6"),
                ("(((func:curry* (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 3) 1) 2 3)", "6"),
                ("((((func:curry* (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 3) 1) 2) 3)", "6"),
                ("(((func:curry* (fn (#opt (a 0) (b 0) (c 0)) (+ a b c)) 3) 1 2) 3)", "6"),

                ("((func:curry* (fn (#rest a) (func:apply #'+ a)) 3) 1 2 3)", "6"),
                ("(((func:curry* (fn (#rest a) (func:apply #'+ a)) 3) 1) 2 3)", "6"),
                ("((((func:curry* (fn (#rest a) (func:apply #'+ a)) 3) 1) 2) 3)", "6"),
                ("(((func:curry* (fn (#rest a) (func:apply #'+ a)) 3) 1 2) 3)", "6"),

                ("((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 2) :a 1)", "1"),
                ("(((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 2) :a) 1)", "1"),

                ("((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 4) :a 1 :b 2)", "3"),
                ("(((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 4) :a) 1 :b 2)", "3"),
                ("((((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 4) :a) 1) :b 2)", "3"),
                ("(((((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 4) :a) 1) :b) 2)", "3"),

                ("((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a 1 :b 2 :c 3)", "6"),
                ("(((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1 :b 2 :c 3)", "6"),
                ("((((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1) :b 2 :c 3)", "6"),
                ("(((((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1) :b) 2 :c 3)", "6"),
                ("((((((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1) :b) 2) :c 3)", "6"),
                ("(((((((func:curry* (fn (#keys (a 0) (b 0) (c 0)) (+ a b c)) 6) :a) 1) :b) 2) :c) 3)", "6"),

                // curry* specific
                ("((func:curry* (fn (#opt (a 1) (b 2) (c 3)) (+ a b c)) 1) 1 2 3)", "6"),
                ("((func:curry* (fn (#opt (a 1) (b 2) (c 3)) (+ a b c)) 2) 1 2 3)", "6"),
                ("((func:curry* (fn (#opt (a 1) (b 2) (c 3)) (+ a b c)) 3) 1 2 3)", "6"),

                ("((func:curry* (fn (#rest args) (func:apply #'+ args)) 3) 1 2 3 4)", "10"),
                ("(((func:curry* (fn (#rest args) (func:apply #'+ args)) 3) 1) 2 3 4)", "10"),
                ("((((func:curry* (fn (#rest args) (func:apply #'+ args)) 3) 1) 2) 3 4)", "10"),
                ("(((func:curry* (fn (#rest args) (func:apply #'+ args)) 3) 1 2) 3 4)", "10"),

                ("((func:curry* (fn (#keys (a 1) (b 2) (c 3)) (+ a b c)) 2) :a 1 :b 2 :c 3)", "6"),
                ("((func:curry* (fn (#keys (a 1) (b 2) (c 3)) (+ a b c)) 4) :a 1 :b 2 :c 3)", "6"),
                ("((func:curry* (fn (#keys (a 1) (b 2) (c 3)) (+ a b c)) 6) :a 1 :b 2 :c 3)", "6"),
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
                "(func:curry* (function (macro () 1)) 2)",
                "(func:curry* (flookup 'cond) 2)",
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
                "(func:curry* 1 2)",
                "(func:curry* 1.1 2)",
                "(func:curry* #t 2)",
                "(func:curry* #f 2)",
                "(func:curry* \"string\" 2)",
                "(func:curry* 'symbol 2)",
                "(func:curry* :keyword 2)",
                "(func:curry* '(1 2 3) 2)",
                "(func:curry* {} 2)",

                "(func:curry* #(+ %1 %2) 1.1)",
                "(func:curry* #(+ %1 %2) #t)",
                "(func:curry* #(+ %1 %2) #f)",
                "(func:curry* #(+ %1 %2) \"string\")",
                "(func:curry* #(+ %1 %2) 'symbol)",
                "(func:curry* #(+ %1 %2) :keyword)",
                "(func:curry* #(+ %1 %2) '(1 2 3))",
                "(func:curry* #(+ %1 %2) {})",
                "(func:curry* #(+ %1 %2) #{})",
            );

            assertion::assert_results_are_invalid_argument_errors(
                &mut interpreter,
                code_vector
            );
        }
    }
}
