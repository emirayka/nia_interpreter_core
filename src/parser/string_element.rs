use nom::{
    character::complete::{
        none_of,
        one_of,
    },
    bytes::complete::{
        tag,
    },
    sequence::{
        preceded,
        terminated,
    },
    combinator::{
        map_res
    },
    branch::alt,
    multi::{
        many0,
    },
    error::ErrorKind
};

#[derive(Debug)]
pub struct StringElement {
    value: String,
}

impl StringElement {
    pub fn new(value: String) -> StringElement {
        StringElement {
            value
        }
    }

    pub fn get_value(self) -> String {
        self.value
    }
}

impl PartialEq for StringElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn make_string_element(value: String) -> Result<StringElement, String> {
    Ok(StringElement {value})
}

pub fn parse_string_element(s: &str) -> Result<(&str, StringElement), nom::Err<(&str, nom::error::ErrorKind)>> {
    let parse_escaped_character = preceded(tag(r#"\"#), one_of(r#"\""#));
    let parse_not_escaped_character = none_of::<_, _, (&str, ErrorKind)>(r#"\""#);
    let parse_inner_character = alt((parse_escaped_character, parse_not_escaped_character));
    let parse_inner_characters = map_res::<_, _, _, _, (&str, ErrorKind), _, _>(
        many0(parse_inner_character),
        |chars: Vec<char>| Ok(chars.iter().cloned().collect::<String>())
    );

    let parse_string = preceded(
        tag("\""),
        terminated(
            parse_inner_characters,
            tag("\"")
        )
    );

    let parse_string_element = map_res(parse_string, make_string_element);

    parse_string_element(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works_on_simple_values() {
        assert_eq!(Ok(("", StringElement{value: r"test".to_string()})), parse_string_element(r#""test""#));
    }

    #[test]
    fn test_escapes_behave_correctly() {
        assert_eq!(Ok(("", StringElement{value: r"\".to_string()})), parse_string_element(r#""\\""#));
        assert_eq!(Ok(("", StringElement{value: r#"""#.to_string()})), parse_string_element(r#""\"""#));
        assert_eq!(Ok(("", StringElement{value: r#"knock"knockknock"#.to_string()})), parse_string_element(r#""knock\"knockknock""#));
    }

//    #[test]
//    fn test_makes_terminated_trash_symbols_incorrect() {
//        assert!(parse_string_element(r#""a"1"#).is_err());
//        assert!(parse_string_element(r#""a"1.0"#).is_err());
//        assert!(parse_string_element(r##""a"#t"##).is_err());
//        assert!(parse_string_element(r##""a"#f"##).is_err());
//        assert!(parse_string_element(r#""a"cutesym"#).is_err());
//        assert!(parse_string_element(r#""a":cutekey"#).is_err());
//        assert!(parse_string_element(r#""a""b""#).is_err());
//    }
}
