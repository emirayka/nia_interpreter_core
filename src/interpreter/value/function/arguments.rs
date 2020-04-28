use crate::interpreter::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalArgument {
    name: String,
    default: Option<Value>,
    provided: Option<String>,
}

impl OptionalArgument {
    pub fn new(
        name: String,
        default: Option<Value>,
        provided: Option<String>
    ) -> OptionalArgument {
        OptionalArgument {
            name,
            default,
            provided,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_default(&self) -> Option<Value> {
        self.default
    }

    pub fn get_provided(&self) -> Option<&String> {
        self.provided.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyArgument {
    name: String,
    default: Option<Value>,
    provided: Option<String>,
}

impl KeyArgument {
    pub fn new(
        name: String,
        default: Option<Value>,
        provided: Option<String>
    )  -> KeyArgument{
        KeyArgument {
            name,
            default,
            provided,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_default(&self) -> Option<Value> {
        self.default
    }

    pub fn get_provided(&self) -> Option<&String> {
        self.provided.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionArguments {
    ordinary: Vec<String>,
    optional: Vec<OptionalArgument>,
    rest: Option<String>,
    keys: Vec<KeyArgument>,
}

impl FunctionArguments {
    pub fn new() -> FunctionArguments {
        FunctionArguments {
            ordinary: Vec::new(),
            optional: Vec::new(),
            rest: None,
            keys: Vec::new()
        }
    }

    pub fn get_ordinary_arguments(&self) -> &Vec<String> {
        &self.ordinary
    }

    pub fn get_optional_arguments(&self) -> &Vec<OptionalArgument> {
        &self.optional
    }

    pub fn get_rest_argument(&self) -> Option<&String> {
        self.rest.as_ref()
    }

    pub fn required_len(&self) -> usize {
        self.ordinary.len()
    }

    pub fn get_key_arguments(&self) -> &Vec<KeyArgument> {
        &self.keys
    }

    fn is_name_free(&self, name: &String) -> bool {
        for ordinary_argument in &self.ordinary {
            if ordinary_argument == name {
                return false;
            }
        }

        for optional_argument in &self.optional {
            if &optional_argument.name == name {
                return false;
            }
        }

        if let Some(rest_argument) = &self.rest {
            if rest_argument == name {
                return false;
            }
        }

        for key_argument in &self.keys {
            if &key_argument.name == name {
                return false;
            }
        }

        true
    }

    pub fn add_ordinary_argument(&mut self, name: String) -> Result<(), ()> {
        if self.optional.len() > 0 || self.rest.is_some() || self.keys.len() > 0  {
            return Err(());
        }

        if !self.is_name_free(&name) {
            return Err(());
        }

        self.ordinary.push(name);
        
        Ok(())
    }
    
    pub fn add_optional_argument(
        &mut self,
        name: String,
        default: Option<Value>,
        provided: Option<String>
    ) -> Result<(), ()> {
        if self.rest.is_some() || self.keys.len() > 0  {
            return Err(());
        }

        if !self.is_name_free(&name) {
            return Err(());
        }

        let optional_argument = OptionalArgument::new(
            name,
            default,
            provided
        );
        
        self.optional.push(optional_argument);
        
        Ok(())
    }
    
    pub fn add_rest_argument(
        &mut self,
        name: String
    ) -> Result<(), ()> {
        if self.rest.is_some() || self.keys.len() > 0  {
            return Err(());
        }

        if !self.is_name_free(&name) {
            return Err(());
        }

        self.rest = Some(name);

        Ok(())
    }
    
    pub fn add_key_argument(
        &mut self,
        name: String,
        default: Option<Value>,
        provided: Option<String>
    ) -> Result<(), ()> {
        if self.optional.len() > 0 || self.rest.is_some()  {
            return Err(());
        }

        if !self.is_name_free(&name) {
            return Err(());
        }

        let keyword_argument = KeyArgument::new(
            name,
            default,
            provided
        );
        
        self.keys.push(keyword_argument);

        Ok(())
    }

    pub fn get_gc_items(&self) -> Vec<Value> {
        let mut result = Vec::new();

        for optional_argument in &self.optional {
            match optional_argument.default {
                Some(value) => result.push(value),
                _ => {}
            }
        }

        for key_argument in &self.keys {
            match key_argument.default {
                Some(value) => result.push(value),
                _ => {}
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn allows_to_add_ordinary_arguments() {
        let mut arguments = FunctionArguments::new();

        assert_eq!(&Vec::<String>::new(), arguments.get_ordinary_arguments());

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);
        assert_eq!(&vec!(String::from("argument-1")), arguments.get_ordinary_arguments());

        let result = arguments.add_ordinary_argument(String::from("argument-2"));
        assert_eq!(Ok(()), result);
        assert_eq!(&vec!(String::from("argument-1"), String::from("argument-2")), arguments.get_ordinary_arguments());
    }

    #[test]
    fn allows_to_add_optional_arguments() {
        let mut arguments = FunctionArguments::new();

        assert_eq!(0, arguments.get_optional_arguments().len());

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);
        assert_eq!(&OptionalArgument::new(String::from("argument-1"), None, None), arguments.get_optional_arguments().last().unwrap());

        let result = arguments.add_optional_argument(String::from("argument-2"), None, None);
        assert_eq!(Ok(()), result);
        assert_eq!(&OptionalArgument::new(String::from("argument-2"), None, None), arguments.get_optional_arguments().last().unwrap());
    }

    #[test]
    fn allows_to_add_rest_argument() {
        let mut arguments = FunctionArguments::new();

        assert_eq!(None, arguments.get_rest_argument());

        let result = arguments.add_rest_argument(String::from("rest"));
        assert_eq!(Ok(()), result);
        assert_eq!(Some(&String::from("rest")), arguments.get_rest_argument());
    }

    #[test]
    fn allows_to_add_key_arguments() {
        let mut arguments = FunctionArguments::new();

        assert_eq!(0, arguments.get_key_arguments().len());

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);
        assert_eq!(&KeyArgument::new(String::from("argument-1"), None, None), arguments.get_key_arguments().last().unwrap());

        let result = arguments.add_key_argument(String::from("argument-2"), None, None);
        assert_eq!(Ok(()), result);
        assert_eq!(&KeyArgument::new(String::from("argument-2"), None, None), arguments.get_key_arguments().last().unwrap());
    }

    #[test]
    fn allows_to_add_ordinary_arguments_after_ordinary_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_ordinary_argument(String::from("argument-2"));
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn allows_to_add_optional_arguments_after_ordinary_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_optional_argument(String::from("argument-2"), None, None);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn allows_to_add_rest_argument_after_ordinary_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_rest_argument(String::from("argument-2"));
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn allows_to_add_key_arguments_after_ordinary_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_key_argument(String::from("argument-2"), None, None);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn does_not_allow_to_add_ordinary_arguments_after_optional_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_ordinary_argument(String::from("argument-2"));
        assert_eq!(Err(()), result);
    }

    #[test]
    fn allows_to_add_optional_arguments_after_optional_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_optional_argument(String::from("argument-2"), None, None);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn allows_to_add_rest_arguments_after_optional_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_rest_argument(String::from("argument-2"));
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn does_not_allow_to_add_key_arguments_after_optional_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_key_argument(String::from("argument-2"), None, None);
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_ordinary_arguments_after_rest_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_rest_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_ordinary_argument(String::from("argument-2"));
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_optional_arguments_after_rest_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_rest_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_optional_argument(String::from("argument-2"), None, None);
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_rest_arguments_after_rest_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_rest_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_rest_argument(String::from("argument-2"));
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_key_arguments_after_rest_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_rest_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_ordinary_arguments_after_key_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_ordinary_argument(String::from("argument-2"));
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_optional_arguments_after_key_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_optional_argument(String::from("argument-2"), None, None);
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_rest_arguments_after_key_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_rest_argument(String::from("argument-2"));
        assert_eq!(Err(()), result);
    }

    #[test]
    fn allows_to_add_key_arguments_after_key_arguments() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_key_argument(String::from("argument-2"), None, None);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn does_not_allow_to_add_ordinary_arguments_after_ordinary_arguments_with_the_same_name() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_optional_arguments_after_ordinary_arguments_with_the_same_name() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_rest_argument_after_ordinary_arguments_with_the_same_name() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_rest_argument(String::from("argument-1"));
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_key_arguments_after_ordinary_arguments_with_the_same_name() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_ordinary_argument(String::from("argument-1"));
        assert_eq!(Ok(()), result);

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_optional_arguments_after_optional_arguments_with_the_same_name() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_rest_arguments_after_optional_arguments_with_the_same_name() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_optional_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_rest_argument(String::from("argument-1"));
        assert_eq!(Err(()), result);
    }

    #[test]
    fn does_not_allow_to_add_key_arguments_after_key_arguments_with_the_same_name() {
        let mut arguments = FunctionArguments::new();

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Ok(()), result);

        let result = arguments.add_key_argument(String::from("argument-1"), None, None);
        assert_eq!(Err(()), result);
    }
}