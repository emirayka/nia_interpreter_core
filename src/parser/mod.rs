mod lib;

mod boolean_element;
mod short_lambda_element;
mod object_pattern_element;
mod float_element;
mod integer_element;
mod string_element;
mod keyword_element;
mod s_expression_element;
mod object_element;
mod prefixed_element;
mod symbol_element;
mod delimited_symbols_element;

mod element;
mod code;

mod parse_error;

pub use {
    parse_error::ParseError,
    boolean_element::BooleanElement,
    short_lambda_element::ShortLambdaElement,
    object_pattern_element::ObjectPatternElement,
    float_element::FloatElement,
    integer_element::IntegerElement,
    string_element::StringElement,
    keyword_element::KeywordElement,
    s_expression_element::SExpressionElement,
    object_element::ObjectElement,
    prefixed_element::PrefixedElement,
    prefixed_element::Prefix,
    symbol_element::SymbolElement,
    delimited_symbols_element::DelimitedSymbolsElement,
    element::Element,
    code::Code,
    code::parse,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_is_ok<T, E>(result: Result<T, E>) {
        assert!(result.is_ok());
    }

    fn assert_is_err<T, E>(result: Result<T, E>) {
        assert!(result.is_err());
    }

    macro_rules! assert_code_eq {
        ($expected:expr, $code:expr) => {
            {
                let expected = $expected;
                let parsed = parse($code);

                assert!(parsed.is_ok());
                let parsed = match parsed {
                    Ok((_, code)) => code,
                    _ => unreachable!()
                };

                let elements = parsed.get_elements();
                assert_eq!(expected.len(), elements.len());
                let len = expected.len();

                for i in 0..len {
                    assert_eq!(&expected[i], &elements[i]);
                }
            };
        }
    }

    #[test]
    fn parses_atoms_correctly() {
        assert_is_ok(parse("20"));
        assert_is_ok(parse("20.0"));
        assert_is_ok(parse("#t"));
        assert_is_ok(parse("#f"));
        assert_is_ok(parse("imacutesymbol"));
        assert_is_ok(parse(":imacutekeyword"));
        assert_is_ok(parse(r#""imacutestring""#));
        assert_is_ok(parse("'imaquotedsymbol"));
        assert_is_ok(parse("`imaquotedsymboltoo"));
        assert_is_ok(parse(",imanexecutedsymbol"));
        assert_is_ok(parse(",@imanexecutedsymbolthatexpandstolist"));
        assert_is_ok(parse("#'imasharpquotedsymbol"));
        assert_is_ok(parse("{}"));
        assert_is_ok(parse("#{}"));
        assert_is_ok(parse("#()"));
    }

    #[test]
    fn parses_simple_s_expression_correctly() {
        assert_is_ok(parse("(+ 1 2)"));
        assert_is_ok(parse("(1+ 1)"));
    }

    #[test]
    fn parses_complex_s_expression_correctly() {
        assert_code_eq!(
            vec!(
                Element::SExpression(s_expression_element::SExpressionElement::new(vec!(
                    Element::Integer(integer_element::IntegerElement::new(1)),
                    Element::Float(float_element::FloatElement::new(1.1)),
                    Element::Boolean(boolean_element::BooleanElement::new(true)),
                    Element::Boolean(boolean_element::BooleanElement::new(false)),
                    Element::Keyword(keyword_element::KeywordElement::new(String::from("keyword"))),
                    Element::String(string_element::StringElement::new(String::from("string"))),
                    Element::Symbol(symbol_element::SymbolElement::new(String::from("symbol"))),
                    Element::SExpression(s_expression_element::SExpressionElement::new(vec!(
                        Element::Integer(integer_element::IntegerElement::new(1)),
                        Element::Float(float_element::FloatElement::new(1.1)),
                        Element::Boolean(boolean_element::BooleanElement::new(true)),
                        Element::Boolean(boolean_element::BooleanElement::new(false)),
                        Element::Keyword(keyword_element::KeywordElement::new(String::from("nested-keyword"))),
                        Element::String(string_element::StringElement::new(String::from("nested-string"))),
                        Element::Symbol(symbol_element::SymbolElement::new(String::from("nested-symbol"))),
                    ))),
                    Element::Object(object_element::ObjectElement::new(vec!(
                        (keyword_element::KeywordElement::new(String::from("a")), Element::Integer(integer_element::IntegerElement::new(1))),
                        (keyword_element::KeywordElement::new(String::from("b")), Element::Float(float_element::FloatElement::new(1.1))),
                        (keyword_element::KeywordElement::new(String::from("c")), Element::Boolean(boolean_element::BooleanElement::new(true))),
                        (keyword_element::KeywordElement::new(String::from("d")), Element::Boolean(boolean_element::BooleanElement::new(false))),
                        (keyword_element::KeywordElement::new(String::from("e")), Element::Keyword(keyword_element::KeywordElement::new(String::from("object-keyword")))),
                        (keyword_element::KeywordElement::new(String::from("f")), Element::String(string_element::StringElement::new(String::from("object-string")))),
                        (keyword_element::KeywordElement::new(String::from("g")), Element::Symbol(symbol_element::SymbolElement::new(String::from("object-symbol")))),
                    ))),
                    Element::ObjectPattern(object_pattern_element::ObjectPatternElement::new(vec!()))
                )))
            ),
            "(1 1.1 #t #f :keyword \"string\" symbol (1 1.1 #t #f :nested-keyword \"nested-string\" nested-symbol) {:a 1 :b 1.1 :c #t :d #f :e :object-keyword :f \"object-string\" :g object-symbol} #{})"
        );
    }

    #[test]
    fn distinguishes_between_symbols_and_numbers() {
        assert_code_eq!(vec!(Element::Float(float_element::FloatElement::new(1.1))), "1.1");
        assert_code_eq!(vec!(Element::Symbol(symbol_element::SymbolElement::new("1.1t".to_string()))), "1.1t");

        assert_code_eq!(vec!(Element::Integer(integer_element::IntegerElement::new(1))), "1");
        assert_code_eq!(vec!(Element::Symbol(symbol_element::SymbolElement::new("1t".to_string()))), "1t");
    }

    #[test]
    fn respects_spaces() {
        assert_is_ok(parse("1 1"));
        assert_is_ok(parse("1\t1"));
        assert_is_ok(parse("1\r1"));
        assert_is_ok(parse("1\n1"));
        assert_is_ok(parse("1\r\n1"));

        assert_is_ok(parse("(1 1)"));
        assert_is_ok(parse("(1\t1)"));
        assert_is_ok(parse("(1\r1)"));
        assert_is_ok(parse("(1\n1)"));
        assert_is_ok(parse("(1\r\n1)"));

        assert_is_ok(parse("{:a  1}"));
        assert_is_ok(parse("{:a \t1}"));
        assert_is_ok(parse("{:a \r1}"));
        assert_is_ok(parse("{:a \n1}"));
        assert_is_ok(parse("{:a \r\n1}"));

        assert_is_ok(parse("#{:a  :b}"));
        assert_is_ok(parse("#{:a \t:b}"));
        assert_is_ok(parse("#{:a \r:b}"));
        assert_is_ok(parse("#{:a \n:b}"));
        assert_is_ok(parse("#{:a \r\n:b}"));

        assert_is_ok(parse("#(+ %1 %2)"));
        assert_is_ok(parse("#(+\t%1 %2)"));
        assert_is_ok(parse("#(+\r%1 %2)"));
        assert_is_ok(parse("#(+\n%1 %2)"));
        assert_is_ok(parse("#(+\r\n%1 %2)"));
    }

    #[test]
    fn respects_spaces_between_forms_after_integer() {
        assert_is_ok(parse("1 1"));
        assert_is_ok(parse("11"));

        assert_is_ok(parse("1 1.1"));
        assert_is_ok(parse("11.1"));

        assert_is_ok(parse("1 #t"));
        assert_is_err(parse("1#t"));
        assert_is_ok(parse("1 #f"));
        assert_is_err(parse("1#f"));

        assert_is_ok(parse("1 :t"));
        assert_is_ok(parse("1:t")); // because, "1:t" is neither a valid symbol nor keyword, but it's a valid delimited symbol expression

        assert_is_ok(parse("1 t"));
        assert_is_ok(parse("1t")); // because, "1t" is a valid symbol

        assert_is_ok(parse("1 a:b"));
        assert_is_ok(parse("1a:b"));

        assert_is_ok(parse("1 \"\""));
        assert_is_err(parse("1\"\""));

        assert_is_ok(parse("1 ()"));
        assert_is_err(parse("1()"));

        assert_is_ok(parse("1 {}"));
        assert_is_err(parse("1{}"));

        assert_is_ok(parse("1 #{}"));
        assert_is_err(parse("1#{}"));

        assert_is_ok(parse("1 #()"));
        assert_is_err(parse("1#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_float() {
        assert_is_ok(parse("1.1 1"));
        assert_is_ok(parse("1.11"));

        assert_is_ok(parse("1.1 1.1"));
        assert_is_ok(parse("1.11.1"));

        assert_is_ok(parse("1.1 #t"));
        assert_is_err(parse("1.1#t"));
        assert_is_ok(parse("1.1 #f"));
        assert_is_err(parse("1.1#f"));

        assert_is_ok(parse("1.1 :t"));
        assert_is_ok(parse("1.1:t")); // because, "1:t" is neither a valid symbol nor keyword, but it's a valid delimited symbol expression
        // todo: maybe change that

        assert_is_ok(parse("1.1 t"));
        assert_is_ok(parse("1.1t")); // because, "1t" is a valid symbol

        assert_is_ok(parse("1.1 a:b"));
        assert_is_ok(parse("1.1a:b"));

        assert_is_ok(parse("1.1 \"\""));
        assert_is_err(parse("1.1\"\""));

        assert_is_ok(parse("1.1 ()"));
        assert_is_err(parse("1.1()"));

        assert_is_ok(parse("1.1 {}"));
        assert_is_err(parse("1.1{}"));

        assert_is_ok(parse("1.1 #{}"));
        assert_is_err(parse("1.1#{}"));

        assert_is_ok(parse("1.1 #()"));
        assert_is_err(parse("1.1#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_boolean_true() {
        assert_is_ok(parse("#t 1"));
        assert_is_err(parse("#t1"));

        assert_is_ok(parse("#t 1.1"));
        assert_is_err(parse("#t1.1"));

        assert_is_ok(parse("#t #t"));
        assert_is_err(parse("#t#t"));
        assert_is_ok(parse("#t #f"));
        assert_is_err(parse("#t#f"));

        assert_is_ok(parse("#t :t"));
        assert_is_err(parse("#t:t"));

        assert_is_ok(parse("#t t"));
        assert_is_err(parse("#tt"));

        assert_is_ok(parse("#t a:b"));
        assert_is_err(parse("#ta:b"));

        assert_is_ok(parse("#t \"\""));
        assert_is_err(parse("#t\"\""));

        assert_is_ok(parse("#t ()"));
        assert_is_err(parse("#t()"));

        assert_is_ok(parse("#t {}"));
        assert_is_err(parse("#t{}"));

        assert_is_ok(parse("#t #{}"));
        assert_is_err(parse("#t#{}"));

        assert_is_ok(parse("#t #()"));
        assert_is_err(parse("#t#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_boolean_false() {
        assert_is_ok(parse("#f 1"));
        assert_is_err(parse("#f1"));

        assert_is_ok(parse("#f 1.1"));
        assert_is_err(parse("#f1.1"));

        assert_is_ok(parse("#f #t"));
        assert_is_err(parse("#f#t"));
        assert_is_ok(parse("#f #f"));
        assert_is_err(parse("#f#f"));

        assert_is_ok(parse("#f :t"));
        assert_is_err(parse("#f:t"));

        assert_is_ok(parse("#f t"));
        assert_is_err(parse("#ft"));

        assert_is_ok(parse("#f a:b"));
        assert_is_err(parse("#fa:b"));

        assert_is_ok(parse("#f \"\""));
        assert_is_err(parse("#f\"\""));

        assert_is_ok(parse("#f ()"));
        assert_is_err(parse("#f()"));

        assert_is_ok(parse("#f {}"));
        assert_is_err(parse("#f{}"));

        assert_is_ok(parse("#f #{}"));
        assert_is_err(parse("#f#{}"));

        assert_is_ok(parse("#f #()"));
        assert_is_err(parse("#f#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_keyword() {
        assert_is_ok(parse(":key 1"));
        assert_is_ok(parse(":key1"));

        assert_is_ok(parse(":key 1.1"));
        assert_is_ok(parse(":key1.1"));

        assert_is_ok(parse(":key #t"));
        assert_is_err(parse(":key#t"));
        assert_is_ok(parse(":key #f"));
        assert_is_err(parse(":key#f"));

        assert_is_ok(parse(":key :t"));
        assert_is_ok(parse(":key:t"));

        assert_is_ok(parse(":key t"));
        assert_is_ok(parse(":keyt"));

        assert_is_ok(parse(":key a:b"));
        assert_is_ok(parse(":keya:b"));

        assert_is_ok(parse(":key \"\""));
        assert_is_err(parse(":key\"\""));

        assert_is_ok(parse(":key ()"));
        assert_is_err(parse(":key()"));

        assert_is_ok(parse(":key {}"));
        assert_is_err(parse(":key{}"));

        assert_is_ok(parse(":key #{}"));
        assert_is_err(parse(":key#{}"));

        assert_is_ok(parse(":key #()"));
        assert_is_err(parse(":key#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_string() {
        assert_is_ok(parse("\"str\" 1"));
        assert_is_err(parse("\"str\"1"));

        assert_is_ok(parse("\"str\" 1.1"));
        assert_is_err(parse("\"str\"1.1"));

        assert_is_ok(parse("\"str\" #t"));
        assert_is_err(parse("\"str\"#t"));
        assert_is_ok(parse("\"str\" #f"));
        assert_is_err(parse("\"str\"#f"));

        assert_is_ok(parse("\"str\" :t"));
        assert_is_err(parse("\"str\":t"));

        assert_is_ok(parse("\"str\" t"));
        assert_is_err(parse("\"str\"t"));

        assert_is_ok(parse("\"str\" a:b"));
        assert_is_err(parse("\"str\"a:b"));

        assert_is_ok(parse("\"str\" \"\""));
        assert_is_err(parse("\"str\"\"\""));

        assert_is_ok(parse("\"str\" ()"));
        assert_is_err(parse("\"str\"()"));

        assert_is_ok(parse("\"str\" {}"));
        assert_is_err(parse("\"str\"{}"));

        assert_is_ok(parse("\"str\" #{}"));
        assert_is_err(parse("\"str\"#{}"));

        assert_is_ok(parse("\"str\" #()"));
        assert_is_err(parse("\"str\"#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_symbol() {
        assert_is_ok(parse("sym 1"));
        assert_is_ok(parse("sym1"));

        assert_is_ok(parse("sym 1.1"));
        assert_is_ok(parse("sym1.1"));

        assert_is_ok(parse("sym #t"));
        assert_is_err(parse("sym#t"));
        assert_is_ok(parse("sym #f"));
        assert_is_err(parse("sym#f"));

        assert_is_ok(parse("sym :t"));
        assert_is_ok(parse("sym:t")); // it's a valid delimited symbol expression

        assert_is_ok(parse("sym t"));
        assert_is_ok(parse("symt"));

        assert_is_ok(parse("sym a:b"));
        assert_is_ok(parse("syma:b"));

        assert_is_ok(parse("sym \"\""));
        assert_is_err(parse("sym\"\""));

        assert_is_ok(parse("sym ()"));
        assert_is_err(parse("sym()"));

        assert_is_ok(parse("sym {}"));
        assert_is_err(parse("sym{}"));

        assert_is_ok(parse("sym #{}"));
        assert_is_err(parse("sym#{}"));

        assert_is_ok(parse("sym #()"));
        assert_is_err(parse("sym#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_s_expression() {
        assert_is_ok(parse("() 1"));
        assert_is_err(parse("()1"));

        assert_is_ok(parse("() 1.1"));
        assert_is_err(parse("()1.1"));

        assert_is_ok(parse("() #t"));
        assert_is_err(parse("()#t"));
        assert_is_ok(parse("() #f"));
        assert_is_err(parse("()#f"));

        assert_is_ok(parse("() :t"));
        assert_is_err(parse("():t"));

        assert_is_ok(parse("() t"));
        assert_is_err(parse("()t"));

        assert_is_ok(parse("() a:b"));
        assert_is_err(parse("()a:b"));

        assert_is_ok(parse("() \"\""));
        assert_is_err(parse("()\"\""));

        assert_is_ok(parse("() ()"));
        assert_is_ok(parse("()()"));

        assert_is_ok(parse("() {}"));
        assert_is_err(parse("(){}"));

        assert_is_ok(parse("() #{}"));
        assert_is_err(parse("()#{}"));

        assert_is_ok(parse("() #()"));
        assert_is_err(parse("()#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_object_literal() {
        assert_is_ok(parse("{} 1"));
        assert_is_err(parse("{}1"));

        assert_is_ok(parse("{} 1.1"));
        assert_is_err(parse("{}1.1"));

        assert_is_ok(parse("{} #t"));
        assert_is_err(parse("{}#t"));
        assert_is_ok(parse("{} #f"));
        assert_is_err(parse("{}#f"));

        assert_is_ok(parse("{} :t"));
        assert_is_err(parse("{}:t"));

        assert_is_ok(parse("{} t"));
        assert_is_err(parse("{}t"));

        assert_is_ok(parse("{} a:b"));
        assert_is_err(parse("{}a:b"));

        assert_is_ok(parse("{} \"\""));
        assert_is_err(parse("{}\"\""));

        assert_is_ok(parse("{} ()"));
        assert_is_err(parse("{}()"));

        assert_is_ok(parse("{} {}"));
        assert_is_err(parse("{}{}"));

        assert_is_ok(parse("{} #{}"));
        assert_is_err(parse("{}#{}"));

        assert_is_ok(parse("{} #()"));
        assert_is_err(parse("{}#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_object_pattern_literal() {
        assert_is_ok(parse("#{} 1"));
        assert_is_err(parse("#{}1"));

        assert_is_ok(parse("#{} 1.1"));
        assert_is_err(parse("#{}1.1"));

        assert_is_ok(parse("#{} #t"));
        assert_is_err(parse("#{}#t"));
        assert_is_ok(parse("#{} #f"));
        assert_is_err(parse("#{}#f"));

        assert_is_ok(parse("#{} :t"));
        assert_is_err(parse("#{}:t"));

        assert_is_ok(parse("#{} t"));
        assert_is_err(parse("#{}t"));

        assert_is_ok(parse("#{} a:b"));
        assert_is_err(parse("#{}a:b"));

        assert_is_ok(parse("#{} \"\""));
        assert_is_err(parse("#{}\"\""));

        assert_is_ok(parse("#{} ()"));
        assert_is_err(parse("#{}()"));

        assert_is_ok(parse("#{} {}"));
        assert_is_err(parse("#{}{}"));

        assert_is_ok(parse("#{} #{}"));
        assert_is_err(parse("#{}#{}"));

        assert_is_ok(parse("#{} #()"));
        assert_is_err(parse("#{}#()"));
    }

    #[test]
    fn respects_spaces_between_forms_after_short_lambda_literal() {
        assert_is_ok(parse("#() 1"));
        assert_is_err(parse("#()1"));

        assert_is_ok(parse("#() 1.1"));
        assert_is_err(parse("#()1.1"));

        assert_is_ok(parse("#() #t"));
        assert_is_err(parse("#()#t"));
        assert_is_ok(parse("#() #f"));
        assert_is_err(parse("#()#f"));

        assert_is_ok(parse("#() :t"));
        assert_is_err(parse("#():t"));

        assert_is_ok(parse("#() t"));
        assert_is_err(parse("#()t"));

        assert_is_ok(parse("#() a:b"));
        assert_is_err(parse("#()a:b"));

        assert_is_ok(parse("#() \"\""));
        assert_is_err(parse("#()\"\""));

        assert_is_ok(parse("#() ()"));
        assert_is_err(parse("#()()"));

        assert_is_ok(parse("#() {}"));
        assert_is_err(parse("#(){}"));

        assert_is_ok(parse("#() #{}"));
        assert_is_err(parse("#()#{}"));

        assert_is_ok(parse("#() #()"));
        assert_is_err(parse("#()#()"));
    }

    #[test]
    fn respects_spaces_at_the_beginning_of_the_input() {
        assert_is_ok(parse(" \t\r\n1"));
        assert_is_ok(parse(" \t\r\n1.1"));
        assert_is_ok(parse(" \t\r\n#t"));
        assert_is_ok(parse(" \t\r\n#f"));
        assert_is_ok(parse(" \t\r\n\"string\""));
        assert_is_ok(parse(" \t\r\n:keyword"));
        assert_is_ok(parse(" \t\r\nsymbol"));
        assert_is_ok(parse(" \t\r\n'(1 2 3)"));
        assert_is_ok(parse(" \t\r\n{}"));
        assert_is_ok(parse(" \t\r\n#{}"));
        assert_is_ok(parse(" \t\r\n#()"));
    }

    #[test]
    fn respects_spaces_at_the_end_of_the_input() {
        assert_is_ok(parse("1 \t\r\n"));
        assert_is_ok(parse("1.1 \t\r\n"));
        assert_is_ok(parse("#t \t\r\n"));
        assert_is_ok(parse("#f \t\r\n"));
        assert_is_ok(parse("\"string\" \t\r\n"));
        assert_is_ok(parse(":keyword \t\r\n"));
        assert_is_ok(parse("symbol \t\r\n"));
        assert_is_ok(parse("'(1 2 3) \t\r\n"));
        assert_is_ok(parse("{} \t\r\n"));
        assert_is_ok(parse("#{} \t\r\n"));
        assert_is_ok(parse("#() \t\r\n"));
    }

    #[test]
    fn respects_spaces_between_sexprs_inside_of_sexpr() {
        assert_is_ok(parse("'(()())"));
        assert_is_ok(parse("'(()() )"));
        assert_is_ok(parse("'(() ())"));
        assert_is_ok(parse("'(() () )"));
        assert_is_ok(parse("'( ()())"));
        assert_is_ok(parse("'( ()() )"));
        assert_is_ok(parse("'( () ())"));
        assert_is_ok(parse("'( () () )"));
    }

    #[test]
    fn does_not_allow_unfinished_s_expressions() {
        assert_is_err(parse("("));
        assert_is_err(parse("()("));
        assert_is_err(parse("(()"));
        assert_is_err(parse("\"string"));
        assert_is_err(parse("((\"string))"));
    }

    #[test]
    fn does_not_allow_unfinished_object_literals() {
        assert_is_err(parse("{"));
        assert_is_err(parse("{}{"));
        assert_is_err(parse("{{}"));
        assert_is_err(parse("\"string"));
        assert_is_err(parse("{{\"string}}"));
    }

    // todo: add tests when input is not complete
}
