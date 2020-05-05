use crate::parser::KeywordElement;

use crate::Error;
use crate::Interpreter;
use crate::Value;

pub fn read_keyword_element(
    interpreter: &mut Interpreter,
    keyword_element: KeywordElement,
) -> Result<Value, Error> {
    let keyword_name = keyword_element.get_value();
    let keyword_value = interpreter.intern_keyword_value(keyword_name);

    Ok(keyword_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn reads_keyword_elements_correctly() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                "cute-keyword",
                KeywordElement::new(String::from("cute-keyword")),
            ),
            (
                ":cute-keyword",
                KeywordElement::new(String::from(":cute-keyword")),
            ),
            (
                "::cute-keyword",
                KeywordElement::new(String::from("::cute-keyword")),
            ),
        ];

        for (keyword_name, keyword_element) in specs {
            let keyword =
                read_keyword_element(&mut interpreter, keyword_element)
                    .unwrap();
            let keyword_id = keyword.try_into().unwrap();
            let keyword = interpreter.get_keyword(keyword_id).unwrap();

            nia_assert_equal(keyword_name, keyword.get_name());
        }
    }
}
