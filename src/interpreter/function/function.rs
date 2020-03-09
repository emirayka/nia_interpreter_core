use crate::interpreter::function::{
    builtin_function,
    interpreted_function,
    macro_function,
    special_form_function
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Function {
    Builtin(builtin_function::BuiltinFunction),
    Interpreted(interpreted_function::InterpretedFunction),
    Macro(macro_function::MacroFunction),
    SpecialForm(special_form_function::SpecialFormFunction),
}
