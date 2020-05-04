use nom::error::ErrorKind;
use nom::{IResult, InputLength};

pub fn parse_symbol_character(input: &str) -> IResult<&str, char, (&str, nom::error::ErrorKind)> {
    match input.chars().next() {
        Some('\\') => {
            let next_input = &input['\\'.len_utf8()..];

            match next_input.chars().next() {
                Some('\\') => Ok((&next_input['\\'.len_utf8()..], '\\')),
                Some('(') => Ok((&next_input['('.len_utf8()..], '(')),
                Some(')') => Ok((&next_input[')'.len_utf8()..], ')')),
                Some('{') => Ok((&next_input['{'.len_utf8()..], '{')),
                Some('}') => Ok((&next_input['}'.len_utf8()..], '}')),
                Some(',') => Ok((&next_input[','.len_utf8()..], ',')),
                Some('`') => Ok((&next_input['`'.len_utf8()..], '`')),
                Some('\'') => Ok((&next_input['\''.len_utf8()..], '\'')),
                Some(' ') => Ok((&next_input[' '.len_utf8()..], ' ')),
                Some(':') => Ok((&next_input[':'.len_utf8()..], ':')),
                Some('"') => Ok((&next_input['"'.len_utf8()..], '"')),
                Some(';') => Ok((&next_input[';'.len_utf8()..], ';')),
                None => Err(nom::Err::Error((input, nom::error::ErrorKind::Eof))),
                _ => Err(nom::Err::Error((input, nom::error::ErrorKind::IsA))),
            }
        }
        Some(c) => match c {
            '(' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ')' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '{' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '}' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ',' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '`' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '\'' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ' ' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ':' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '#' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '"' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ';' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            cc if !cc.is_whitespace() => Ok((&input[c.len_utf8()..], c)),
            _ => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
        },
        _ => Err(nom::Err::Error((input, nom::error::ErrorKind::Eof))),
    }
}

pub fn parse_keyword_character(input: &str) -> IResult<&str, char, (&str, nom::error::ErrorKind)> {
    match input.chars().next() {
        Some('\\') => {
            let next_input = &input['\\'.len_utf8()..];

            match next_input.chars().next() {
                Some('\\') => Ok((&next_input['\\'.len_utf8()..], '\\')),
                Some('(') => Ok((&next_input['('.len_utf8()..], '(')),
                Some(')') => Ok((&next_input[')'.len_utf8()..], ')')),
                Some('{') => Ok((&next_input['{'.len_utf8()..], '{')),
                Some('}') => Ok((&next_input['}'.len_utf8()..], '}')),
                Some(',') => Ok((&next_input[','.len_utf8()..], ',')),
                Some('`') => Ok((&next_input['`'.len_utf8()..], '`')),
                Some('\'') => Ok((&next_input['\''.len_utf8()..], '\'')),
                Some(' ') => Ok((&next_input[' '.len_utf8()..], ' ')),
                Some('"') => Ok((&next_input['"'.len_utf8()..], '"')),
                Some(';') => Ok((&next_input[';'.len_utf8()..], ';')),
                None => Err(nom::Err::Error((input, nom::error::ErrorKind::Eof))),
                _ => Err(nom::Err::Error((input, nom::error::ErrorKind::IsA))),
            }
        }
        Some(c) => match c {
            '(' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ')' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '{' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '}' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ',' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '`' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '\'' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ' ' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '#' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            '"' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            ';' => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
            cc if !cc.is_whitespace() => Ok((&input[c.len_utf8()..], c)),
            _ => Err(nom::Err::Error((input, nom::error::ErrorKind::IsNot))),
        },
        _ => Err(nom::Err::Error((input, nom::error::ErrorKind::Eof))),
    }
}

pub fn parse_comment_character(input: &str) -> IResult<&str, char, (&str, nom::error::ErrorKind)> {
    match input.chars().next() {
        Some(c) => {
            if c == '\n' {
                IResult::Err(nom::Err::Error((input, ErrorKind::Tag)))
            } else {
                Ok((&input[c.len_utf8()..], c))
            }
        }
        None => {
            if input.is_empty() {
                IResult::Err(nom::Err::Error((input, ErrorKind::Eof)))
            } else {
                IResult::Err(nom::Err::Failure((input, ErrorKind::Eof)))
            }
        }
    }
}

pub fn end_of_input(input: &str) -> IResult<&str, &str, (&str, nom::error::ErrorKind)> {
    if input.input_len() == 0 {
        IResult::Ok((input, input))
    } else {
        IResult::Err(nom::Err::Error((input, ErrorKind::Eof)))
    }
}
