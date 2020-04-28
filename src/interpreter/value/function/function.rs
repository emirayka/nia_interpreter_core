use crate::interpreter::value::{
    BuiltinFunction,
    InterpretedFunction,
    MacroFunction,
    SpecialFormFunction,
};
use crate::interpreter::value::Value;
use crate::interpreter::environment::EnvironmentId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Function {
    Builtin(BuiltinFunction),
    Interpreted(InterpretedFunction),
    Macro(MacroFunction),
    SpecialForm(SpecialFormFunction),
}

impl Function {
    pub fn get_gc_items(&self) -> Option<Vec<Value>> {
        match self {
            Function::Interpreted(interpreted_function) => interpreted_function.get_gc_items(),
            Function::Builtin(builtin_function) => builtin_function.get_gc_items(),
            Function::Macro(macro_function) => macro_function.get_gc_items(),
            Function::SpecialForm(special_form_function) => special_form_function.get_gc_items(),
        }
    }

    pub fn get_gc_environment(&self) -> Option<EnvironmentId> {
        match self {
            Function::Interpreted(interpreted_function) => interpreted_function.get_gc_environment(),
            Function::Builtin(builtin_function) => builtin_function.get_gc_environment(),
            Function::Macro(macro_function) => macro_function.get_gc_environment(),
            Function::SpecialForm(special_form_function) => special_form_function.get_gc_environment(),
        }
    }
}
