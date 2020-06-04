use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let functions: Vec<fn(&mut Interpreter) -> Result<(), Error>> = vec![
        defv::infect,
        defc::infect,
        defn::infect,
        defm::infect,
        defon::infect,
        _fn::infect,
        _if::infect,
        when::infect,
        unless::infect,
        inc_mark::infect,
        dec_mark::infect,
        // cr::infect,
        empty::infect,
    ];

    for function in functions {
        function(interpreter)?;
    }

    Ok(())
}

mod defv {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            "(define-function defv (function (macro (name #opt (value nil)) (list:new 'define-variable name value))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn defines_variable() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![("(defv a 1) a", "1"), ("(defv b) b", "nil")];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod defc {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            "(define-function defc (function (macro (name #opt (value nil)) (list:new 'define-variable name value :const))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn defines_const_variable() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                ("(defc a 1) a", "1"),
                ("(defc b) b", "nil"),
                // todo: probably change error symbol to smth like "setting-constant-error"
                (
                    "(try (progn (defc c 2) (set! c 3) c) (catch 'generic-execution-error #t))",
                    "#t",
                ),
            ];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod defn {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            "(define-function defn (function (macro (name #rest params) (list:new 'define-function name (list:new 'function (cons:new 'lambda params))))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn defines_function() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                ("(defn a () 1) (a)", "1"),
                ("(defn b (a) a) (b 2)", "2"),
                ("(defn c (#opt b c) (list:new b c)) (c)", "'(nil nil)"),
                ("(defn d (#opt b c) (list:new b c)) (d 2)", "'(2 nil)"),
                ("(defn e (#opt b c) (list:new b c)) (e 2 3)", "'(2 3)"),
                ("(defn f (#rest b) b) (f 2 3 4)", "'(2 3 4)"),
                ("(defn g (#keys b) b) (g :b 1)", "1"),
            ];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod defon {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            r#"
(define-function defon (function (macro (object-name name #rest params) (list:new 'object:set! object-name (list:new 'quote name) (list:new 'function (cons:new 'lambda params))))))"#
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn defines_function() {
            let mut interpreter = Interpreter::new();

            interpreter
                .execute_in_root_environment("(defv o {})")
                .unwrap();

            let pairs = vec![
                ("(defon o a () 1) (o:a)", "1"),
                ("(defon o b (a) a) (o:b 2)", "2"),
                ("(defon o c (#opt b c) (list:new b c)) (o:c)", "'(nil nil)"),
                ("(defon o d (#opt b c) (list:new b c)) (o:d 2)", "'(2 nil)"),
                ("(defon o e (#opt b c) (list:new b c)) (o:e 2 3)", "'(2 3)"),
                ("(defon o f (#rest b) b) (o:f 2 3 4)", "'(2 3 4)"),
                ("(defon o g (#keys b) b) (o:g :b 1)", "1"),
            ];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod defm {
    #[allow(unused_imports)]
    use super::*;

    use crate::library;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            "(define-function defm (function (macro (name #rest params) (list:new 'define-function name (list:new 'function (cons:new 'macro params))))))"
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn defines_macro() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                ("(defm a () 1) (a)", "1"),
                ("(defm b (a) a) (b 2)", "2"),
                (
                    "(defm c (#opt b c) (list:new 'list:new b c)) (c)",
                    "'(nil nil)",
                ),
                (
                    "(defm d (#opt b c) (list:new 'list:new b c)) (d 2)",
                    "'(2 nil)",
                ),
                (
                    "(defm e (#opt b c) (list:new 'list:new b c)) (e 2 3)",
                    "'(2 3)",
                ),
                (
                    "(defm f (#rest b) (list:new 'quote b)) (f 2 3 4)",
                    "'(2 3 4)",
                ),
                ("(defm g (#keys b) b) (g :b 1)", "1"),
                (
                    "(defm h (a b) (list:new 'list:new a b)) (h 'a 'b)",
                    "(list:new 'a 'b)",
                ),
            ];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod _fn {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            "(defm fn (#rest args) (list:new 'function (cons:new 'lambda args)))",
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn makes_a_function() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                ("(fn () 1)", "(function (lambda () 1))"),
                ("(fn (a) (+ a a))", "(function (lambda (a) (+ a a)))"),
                (
                    "(fn (a #opt args) args)",
                    "(function (lambda (a #opt args) args))",
                ),
                (
                    "(fn (a #rest args) args)",
                    "(function (lambda (a #rest args) args))",
                ),
                (
                    "(fn (a #keys args) args)",
                    "(function (lambda (a #keys args) args))",
                ),
            ];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod _if {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            "(defm if (condition then-clause else-clause)\
  (list:new 'cond (list:new condition then-clause) (list:new #t else-clause)))",
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn works_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                (
                    "(defv a 1) (defv b 2) (list:new (if #t a b) (if #f a b))",
                    "'(1 2)",
                ),
                (
                    "(defv c 0) (defv d 0) (list:new (if #t (set! c (inc c)) (set! d (inc d))) (if #f (set! c (inc c)) (set! d (inc d)))) (list:new c d)",
                    "'(1 1)",
                ),
            ];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod when {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            "(defm when (condition then-clause)\
  (list:new 'cond (list:new condition then-clause)))",
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn works_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![(
                "(defv a 0) (defv b 0) (list:new (when #t (set! a (inc a))) (when #f (set! b (inc b)))) (list:new a b)",
                "'(1 0)",
            )];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod unless {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            "(defm unless (condition else-clause)\
  (list:new 'cond (list:new (list:new 'not condition) else-clause)))",
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn works_correctly() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![(
                "(defv a 0) (defv b 0) (list:new (unless #t (set! a (inc a))) (unless #f (set! b (inc b)))) (list:new a b)",
                "'(0 1)",
            )];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod inc_mark {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            r#"
            (defm inc! (symbol)
              (let ((symbol symbol))
                (list:new 'set! symbol (list:new 'inc symbol))))
            "#,
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn increments_value_by_place() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                ("(let ((a 0)) a)", "0"),
                ("(let ((a 0)) (inc! a) a)", "1"),
                ("(let ((a 0)) (inc! a) (inc! a) a)", "2"),
            ];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod dec_mark {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            r#"
            (defm dec! (symbol)
              (let ((symbol symbol))
                (list:new 'set! symbol (list:new 'dec symbol))))
            "#,
        )?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[allow(unused_imports)]
        use crate::utils;

        #[test]
        fn increments_value_by_place() {
            let mut interpreter = Interpreter::new();

            let pairs = vec![
                ("(let ((a 0)) a)", "0"),
                ("(let ((a 0)) (dec! a) a)", "-1"),
                ("(let ((a 0)) (dec! a) (dec! a) a)", "-2"),
            ];

            utils::assert_results_are_equal(&mut interpreter, pairs);
        }
    }
}

mod cr {
    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute_in_root_environment(
            r#"
            (defn caar (c) (car (car c)))
            (defn cadr (c) (car (cdr c)))
            (defn cdar (c) (cdr (car c)))
            (defn cddr (c) (cdr (cdr c)))

            (defn caaar (c) (car (car (car c))))
            (defn caadr (c) (car (car (cdr c))))
            (defn cadar (c) (car (cdr (car c))))
            (defn caddr (c) (car (cdr (cdr c))))
            (defn cdaar (c) (cdr (car (car c))))
            (defn cdadr (c) (cdr (car (cdr c))))
            (defn cddar (c) (cdr (cdr (car c))))
            (defn cdddr (c) (cdr (cdr (cdr c))))

            (defn caaaar (c) (car (car (car (car c)))))
            (defn caaadr (c) (car (car (car (cdr c)))))
            (defn caadar (c) (car (car (cdr (car c)))))
            (defn caaddr (c) (car (car (cdr (cdr c)))))
            (defn cadaar (c) (car (cdr (car (car c)))))
            (defn cadadr (c) (car (cdr (car (cdr c)))))
            (defn caddar (c) (car (cdr (cdr (car c)))))
            (defn cadddr (c) (car (cdr (cdr (cdr c)))))
            (defn cdaaar (c) (cdr (car (car (car c)))))
            (defn cdaadr (c) (cdr (car (car (cdr c)))))
            (defn cdadar (c) (cdr (car (cdr (car c)))))
            (defn cdaddr (c) (cdr (car (cdr (cdr c)))))
            (defn cddaar (c) (cdr (cdr (car (car c)))))
            (defn cddadr (c) (cdr (cdr (car (cdr c)))))
            (defn cdddar (c) (cdr (cdr (cdr (car c)))))
            (defn cddddr (c) (cdr (cdr (cdr (cdr c)))))

            (defn caaaaar (c) (car (car (car (car (car c))))))
            (defn caaaadr (c) (car (car (car (car (cdr c))))))
            (defn caaadar (c) (car (car (car (cdr (car c))))))
            (defn caaaddr (c) (car (car (car (cdr (cdr c))))))
            (defn caadaar (c) (car (car (cdr (car (car c))))))
            (defn caadadr (c) (car (car (cdr (car (cdr c))))))
            (defn caaddar (c) (car (car (cdr (cdr (car c))))))
            (defn caadddr (c) (car (car (cdr (cdr (cdr c))))))
            (defn cadaaar (c) (car (cdr (car (car (car c))))))
            (defn cadaadr (c) (car (cdr (car (car (cdr c))))))
            (defn cadadar (c) (car (cdr (car (cdr (car c))))))
            (defn cadaddr (c) (car (cdr (car (cdr (cdr c))))))
            (defn caddaar (c) (car (cdr (cdr (car (car c))))))
            (defn caddadr (c) (car (cdr (cdr (car (cdr c))))))
            (defn cadddar (c) (car (cdr (cdr (cdr (car c))))))
            (defn caddddr (c) (car (cdr (cdr (cdr (cdr c))))))
            (defn cdaaaar (c) (cdr (car (car (car (car c))))))
            (defn cdaaadr (c) (cdr (car (car (car (cdr c))))))
            (defn cdaadar (c) (cdr (car (car (cdr (car c))))))
            (defn cdaaddr (c) (cdr (car (car (cdr (cdr c))))))
            (defn cdadaar (c) (cdr (car (cdr (car (car c))))))
            (defn cdadadr (c) (cdr (car (cdr (car (cdr c))))))
            (defn cdaddar (c) (cdr (car (cdr (cdr (car c))))))
            (defn cdadddr (c) (cdr (car (cdr (cdr (cdr c))))))
            (defn cddaaar (c) (cdr (cdr (car (car (car c))))))
            (defn cddaadr (c) (cdr (cdr (car (car (cdr c))))))
            (defn cddadar (c) (cdr (cdr (car (cdr (car c))))))
            (defn cddaddr (c) (cdr (cdr (car (cdr (cdr c))))))
            (defn cdddaar (c) (cdr (cdr (cdr (car (car c))))))
            (defn cdddadr (c) (cdr (cdr (cdr (car (cdr c))))))
            (defn cddddar (c) (cdr (cdr (cdr (cdr (car c))))))
            (defn cdddddr (c) (cdr (cdr (cdr (cdr (cdr c))))))

            (defn set-caar! (c v) (set-car! (car c) v))
            (defn set-cadr! (c v) (set-car! (cdr c) v))
            (defn set-cdar! (c v) (set-cdr! (car c) v))
            (defn set-cddr! (c v) (set-cdr! (cdr c) v))

            (defn set-caaar! (c v) (set-car! (car (car c)) v))
            (defn set-caadr! (c v) (set-car! (car (cdr c)) v))
            (defn set-cadar! (c v) (set-car! (cdr (car c)) v))
            (defn set-caddr! (c v) (set-car! (cdr (cdr c)) v))
            (defn set-cdaar! (c v) (set-cdr! (car (car c)) v))
            (defn set-cdadr! (c v) (set-cdr! (car (cdr c)) v))
            (defn set-cddar! (c v) (set-cdr! (cdr (car c)) v))
            (defn set-cdddr! (c v) (set-cdr! (cdr (cdr c)) v))

            (defn set-caaaar! (c v) (set-car! (car (car (car c))) v))
            (defn set-caaadr! (c v) (set-car! (car (car (cdr c))) v))
            (defn set-caadar! (c v) (set-car! (car (cdr (car c))) v))
            (defn set-caaddr! (c v) (set-car! (car (cdr (cdr c))) v))
            (defn set-cadaar! (c v) (set-car! (cdr (car (car c))) v))
            (defn set-cadadr! (c v) (set-car! (cdr (car (cdr c))) v))
            (defn set-caddar! (c v) (set-car! (cdr (cdr (car c))) v))
            (defn set-cadddr! (c v) (set-car! (cdr (cdr (cdr c))) v))
            (defn set-cdaaar! (c v) (set-cdr! (car (car (car c))) v))
            (defn set-cdaadr! (c v) (set-cdr! (car (car (cdr c))) v))
            (defn set-cdadar! (c v) (set-cdr! (car (cdr (car c))) v))
            (defn set-cdaddr! (c v) (set-cdr! (car (cdr (cdr c))) v))
            (defn set-cddaar! (c v) (set-cdr! (cdr (car (car c))) v))
            (defn set-cddadr! (c v) (set-cdr! (cdr (car (cdr c))) v))
            (defn set-cdddar! (c v) (set-cdr! (cdr (cdr (car c))) v))
            (defn set-cddddr! (c v) (set-cdr! (cdr (cdr (cdr c))) v))

            (defn set-caaaaar! (c v) (set-car! (car (car (car (car c)))) v))
            (defn set-caaaadr! (c v) (set-car! (car (car (car (cdr c)))) v))
            (defn set-caaadar! (c v) (set-car! (car (car (cdr (car c)))) v))
            (defn set-caaaddr! (c v) (set-car! (car (car (cdr (cdr c)))) v))
            (defn set-caadaar! (c v) (set-car! (car (cdr (car (car c)))) v))
            (defn set-caadadr! (c v) (set-car! (car (cdr (car (cdr c)))) v))
            (defn set-caaddar! (c v) (set-car! (car (cdr (cdr (car c)))) v))
            (defn set-caadddr! (c v) (set-car! (car (cdr (cdr (cdr c)))) v))
            (defn set-cadaaar! (c v) (set-car! (cdr (car (car (car c)))) v))
            (defn set-cadaadr! (c v) (set-car! (cdr (car (car (cdr c)))) v))
            (defn set-cadadar! (c v) (set-car! (cdr (car (cdr (car c)))) v))
            (defn set-cadaddr! (c v) (set-car! (cdr (car (cdr (cdr c)))) v))
            (defn set-caddaar! (c v) (set-car! (cdr (cdr (car (car c)))) v))
            (defn set-caddadr! (c v) (set-car! (cdr (cdr (car (cdr c)))) v))
            (defn set-cadddar! (c v) (set-car! (cdr (cdr (cdr (car c)))) v))
            (defn set-caddddr! (c v) (set-car! (cdr (cdr (cdr (cdr c)))) v))
            (defn set-cdaaaar! (c v) (set-cdr! (car (car (car (car c)))) v))
            (defn set-cdaaadr! (c v) (set-cdr! (car (car (car (cdr c)))) v))
            (defn set-cdaadar! (c v) (set-cdr! (car (car (cdr (car c)))) v))
            (defn set-cdaaddr! (c v) (set-cdr! (car (car (cdr (cdr c)))) v))
            (defn set-cdadaar! (c v) (set-cdr! (car (cdr (car (car c)))) v))
            (defn set-cdadadr! (c v) (set-cdr! (car (cdr (car (cdr c)))) v))
            (defn set-cdaddar! (c v) (set-cdr! (car (cdr (cdr (car c)))) v))
            (defn set-cdadddr! (c v) (set-cdr! (car (cdr (cdr (cdr c)))) v))
            (defn set-cddaaar! (c v) (set-cdr! (cdr (car (car (car c)))) v))
            (defn set-cddaadr! (c v) (set-cdr! (cdr (car (car (cdr c)))) v))
            (defn set-cddadar! (c v) (set-cdr! (cdr (car (cdr (car c)))) v))
            (defn set-cddaddr! (c v) (set-cdr! (cdr (car (cdr (cdr c)))) v))
            (defn set-cdddaar! (c v) (set-cdr! (cdr (cdr (car (car c)))) v))
            (defn set-cdddadr! (c v) (set-cdr! (cdr (cdr (car (cdr c)))) v))
            (defn set-cddddar! (c v) (set-cdr! (cdr (cdr (cdr (car c)))) v))
            (defn set-cdddddr! (c v) (set-cdr! (cdr (cdr (cdr (cdr c)))) v))
            "#,
        )?;

        Ok(())
    }
}

mod empty {
    #[allow(unused_imports)]
    use super::*;

    pub fn infect(_interpreter: &mut Interpreter) -> Result<(), Error> {
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn does_nothing() {}
    }
}
