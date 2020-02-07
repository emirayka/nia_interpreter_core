use crate::parser::keyword_element::{KeywordElement, parse_keyword_element};
use crate::parser::{Element, parse_element};
use nom::sequence::{terminated, preceded, pair};
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::multi::many0;
use nom::combinator::map_res;

#[derive(Debug)]
pub struct ObjectElement {
    values: Vec<(KeywordElement, Element)>,
}

impl ObjectElement {
    pub fn new(values: Vec<(KeywordElement, Element)>) -> ObjectElement {
        ObjectElement {
            values
        }
    }

    pub fn get_values(&self) -> &Vec<(KeywordElement, Element)> {
        &self.values
    }
}

impl PartialEq for ObjectElement {
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

fn make_object_element(values: Vec<(KeywordElement, Element)>) -> Result<ObjectElement, String> {
    Ok(ObjectElement::new(values))
}

pub fn parse_object_element(s: &str) -> Result<(&str, ObjectElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_pairs = many0(pair(
        preceded(space0, parse_keyword_element),
        preceded(space0, parse_element)
    ));

    let opening_brace = terminated(tag("{"), space0);
    let closing_brace = preceded(space0, tag("}"));

    let parse_object = preceded(
        opening_brace,
        terminated(
            parse_pairs,
            closing_brace
        )
    );

    let parse_object_element = map_res(parse_object, make_object_element);

    parse_object_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_parsed_correctly {
        ($what:expr) => {
            //println!("{:?}", parse_object_element($what));
            assert!(parse_object_element($what).is_ok());
        }
    }

    macro_rules! assert_failed_correctly {
        ($what:expr) => {
            //println!("{:?}", parse_object_element($what));
            assert!(parse_object_element($what).is_err());
        }
    }

    #[test]
    fn test_simple_object_1() {
        assert_parsed_correctly!(r#"{:f :keyword}"#);
    }

    #[test]
    fn test_simple_object_2() {
        assert_parsed_correctly!(r#"{:oi test :a 1 :b 1.0 :c "test" :d #t :e #f :f :keyword}"#);
    }

    #[test]
    fn test_spaces_are_processed_correctly() {
        assert_parsed_correctly!(r#"{:a test}"#);
        assert_parsed_correctly!(r#"{ :a test}"#);
        assert_parsed_correctly!(r#"{:a test }"#);
        assert_parsed_correctly!(r#"{ :a test }"#);
    }

    #[test]
    fn test_nested_objects_are_processed() {
        assert_parsed_correctly!(r#"{:a test :b :list :c {:a b :b 1 :c 2}}"#);
        assert_parsed_correctly!(r#"{:a test :b {:a test :b {:a b :b 1 :c 2}}}"#);
    }

    #[test]
    fn returns_err_when_pairs_are_incomplete() {
        assert_failed_correctly!(r#"{:a test :b}"#);
    }
}
