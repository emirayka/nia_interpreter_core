pub mod arguments;
pub mod builtin_function;
pub mod interpreted_function;
pub mod macro_function;
pub mod special_form_function;

pub mod function_arena;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Function {
    Builtin(builtin_function::BuiltinFunction),
    Interpreted(interpreted_function::InterpretedFunction),
    Macro(macro_function::MacroFunction),
    SpecialForm(special_form_function::SpecialFormFunction),
}
