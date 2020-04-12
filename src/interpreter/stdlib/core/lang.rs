use crate::interpreter::error::Error;
use crate::interpreter::interpreter::Interpreter;

pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
    let functions: Vec<fn(&mut Interpreter) -> Result<(), Error>> = vec!(
        defv::infect,
        defn::infect,
        defm::infect,

        _fn::infect,

        _if::infect,
        when::infect,
        unless::infect,

        // cr::infect,

        empty::infect
    );

    for function in functions {
        function(interpreter)?;
    }

    Ok(())
}

mod defv {
    use super::*;

    // The god of man is a failure
    // Our fortress is burning against the grain of the shattered sky
    // Charred birds escape from the ruins and return as cascading blood
    // Dying bloodbirds pooling, feeding the flood
    // The god of man is a failure
    // And all of our shadows are ashes against the grain

    // Agalloch - Our Fortress is Burning... II - Bloodbirds

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

mod cr {
    use super::*;

    pub fn infect(interpreter: &mut Interpreter) -> Result<(), Error> {
        interpreter.execute(r#"
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

            (defn set-caaar! (c v) (set-car! (car (car c)) v ))
            (defn set-caadr! (c v) (set-car! (car (cdr c)) v ))
            (defn set-cadar! (c v) (set-car! (cdr (car c)) v ))
            (defn set-caddr! (c v) (set-car! (cdr (cdr c)) v ))
            (defn set-cdaar! (c v) (set-cdr! (car (car c)) v ))
            (defn set-cdadr! (c v) (set-cdr! (car (cdr c)) v ))
            (defn set-cddar! (c v) (set-cdr! (cdr (car c)) v ))
            (defn set-cdddr! (c v) (set-cdr! (cdr (cdr c)) v ))

            (defn set-caaaar! (c v) (set-car! (car (car (car c))) v ))
            (defn set-caaadr! (c v) (set-car! (car (car (cdr c))) v ))
            (defn set-caadar! (c v) (set-car! (car (cdr (car c))) v ))
            (defn set-caaddr! (c v) (set-car! (car (cdr (cdr c))) v ))
            (defn set-cadaar! (c v) (set-car! (cdr (car (car c))) v ))
            (defn set-cadadr! (c v) (set-car! (cdr (car (cdr c))) v ))
            (defn set-caddar! (c v) (set-car! (cdr (cdr (car c))) v ))
            (defn set-cadddr! (c v) (set-car! (cdr (cdr (cdr c))) v ))
            (defn set-cdaaar! (c v) (set-cdr! (car (car (car c))) v ))
            (defn set-cdaadr! (c v) (set-cdr! (car (car (cdr c))) v ))
            (defn set-cdadar! (c v) (set-cdr! (car (cdr (car c))) v ))
            (defn set-cdaddr! (c v) (set-cdr! (car (cdr (cdr c))) v ))
            (defn set-cddaar! (c v) (set-cdr! (cdr (car (car c))) v ))
            (defn set-cddadr! (c v) (set-cdr! (cdr (car (cdr c))) v ))
            (defn set-cdddar! (c v) (set-cdr! (cdr (cdr (car c))) v ))
            (defn set-cddddr! (c v) (set-cdr! (cdr (cdr (cdr c))) v ))

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
            "#)?;

        Ok(())
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

    pub fn infect(_interpreter: &mut Interpreter) -> Result<(), Error> {
        Ok(())
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn does_nothing() {
        }
    }
}
