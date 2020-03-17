use crate::parser::keyword_element::{KeywordElement, parse_keyword_element};
use nom::sequence::{terminated, preceded};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::combinator::{map_res};

#[derive(Debug)]
pub struct ObjectPatternElement {
    values: Vec<KeywordElement>,
}

impl ObjectPatternElement {
    pub fn new(values: Vec<KeywordElement>) -> ObjectPatternElement {
        ObjectPatternElement {
            values
        }
    }

    pub fn get_values(self) -> Vec<KeywordElement> {
        self.values
    }

    pub fn get_values_ref(&self) -> &Vec<KeywordElement> {
        &self.values
    }
}

impl PartialEq for ObjectPatternElement {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }

        let len = self.values.len();

        for i in 0..len {
            let mut found = false;

            for j in 0..len {
                if self.values[i] == other.values[j] {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        return true;
    }
}

fn make_object_pattern_element(values: Vec<KeywordElement>) -> Result<ObjectPatternElement, String> {
    let object_element = ObjectPatternElement::new(values);

    Ok(object_element)
}

pub fn parse_object_pattern_element(s: &str) -> Result<(&str, ObjectPatternElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_pairs = many0(
        preceded(multispace0, parse_keyword_element)
    );

    let opening_brace = terminated(tag("#{"), multispace0);
    let closing_brace = preceded(multispace0, tag("}"));

    let parse_object = preceded(
        opening_brace,
        terminated(
            parse_pairs,
            closing_brace
        )
    );

    let parse_object_element = map_res(parse_object, make_object_pattern_element);

    parse_object_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parsed_correctly(code: &str) {
        //println!("{:?}", parse_object_element(code));
        assert!(parse_object_pattern_element(code).is_ok());
    }

    fn assert_failed_correctly(code: &str) {
        //println!("{:?}", parse_object_element(code));
        assert!(parse_object_pattern_element(code).is_err());
    }

    #[test]
    fn parses_simple_objects() {
        assert_parsed_correctly(r#"#{:key-1}"#);
        assert_parsed_correctly(r#"#{:key-1 :key-2}"#);
        assert_parsed_correctly(r#"#{:key-1 :key-2 :key-3}"#);
        assert_parsed_correctly(r#"#{:key-1 :key-2 :key-3 :key-4}"#);
    }

    #[test]
    fn parses_spaces_are_processed_correctly() {
        let specs = vec!(
            "#{:a :test}",
            "#{ :a :test}",
            "#{:a :test }",
            "#{ :a :test }",

            "#{:a\t:test}",
            "#{\t:a :test}",
            "#{:a\t:test\t}",
            "#{\t:a :test\t}",

            "#{:a\n:test}",
            "#{\n:a\n:test}",
            "#{:a\n:test\n}",
            "#{\n:a\n:test\n}",

            "#{:a\r:test}",
            "#{\r:a\r:test}",
            "#{:a\r:test\r}",
            "#{\r:a\r:test\r}",

            "#{:a\r\n:test}",
            "#{\r\n:a\r\n:test}",
            "#{:a\r\n:test\r\n}",
            "#{\r\n:a\r\n:test\r\n}",
        );

        for spec in specs {
            assert_parsed_correctly(spec);
        }
    }

    #[test]
    fn does_not_parse_values_that_are_not_keywords() {
        assert_failed_correctly(r#"#{:key 1}"#);
        assert_failed_correctly(r#"#{:key 1.1}"#);
        assert_failed_correctly(r#"#{:key #t}"#);
        assert_failed_correctly(r#"#{:key #f}"#);
        assert_failed_correctly(r#"#{:key "string"}"#);
        assert_failed_correctly(r#"#{:key symbol}"#);
        assert_failed_correctly(r#"#{:key 'quoted-symbol}"#);
        assert_failed_correctly(r#"#{:key (cons 1 2)}"#);
        assert_failed_correctly(r#"#{:key {}}"#);
        assert_failed_correctly(r#"#{:key #()}"#);
    }
}
