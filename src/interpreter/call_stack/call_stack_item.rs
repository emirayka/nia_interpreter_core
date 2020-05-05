use crate::Value;
use crate::{FunctionId, SymbolId};

#[derive(Debug, Clone, PartialEq, Eq)]
enum CallStackItemContent {
    NamedFunctionInvocation(FunctionId, SymbolId, Vec<Value>),
    AnonymousFunctionInvocation(FunctionId, Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallStackItem(CallStackItemContent);

impl CallStackItem {
    pub fn new_named(
        function_id: FunctionId,
        function_name_symbol_id: SymbolId,
        arguments: Vec<Value>,
    ) -> CallStackItem {
        CallStackItem(CallStackItemContent::NamedFunctionInvocation(
            function_id,
            function_name_symbol_id,
            arguments,
        ))
    }

    pub fn new_anonymous(
        function_id: FunctionId,
        arguments: Vec<Value>,
    ) -> CallStackItem {
        CallStackItem(CallStackItemContent::AnonymousFunctionInvocation(
            function_id,
            arguments,
        ))
    }

    pub fn get_function_id(&self) -> FunctionId {
        match &self.0 {
            CallStackItemContent::NamedFunctionInvocation(
                function_id,
                _,
                _,
            ) => *function_id,
            CallStackItemContent::AnonymousFunctionInvocation(
                function_id,
                _,
            ) => *function_id,
        }
    }

    pub fn get_function_symbol(&self) -> Option<SymbolId> {
        match &self.0 {
            CallStackItemContent::NamedFunctionInvocation(_, symbol_id, _) => {
                Some(*symbol_id)
            },
            CallStackItemContent::AnonymousFunctionInvocation(_, _) => None,
        }
    }

    pub fn get_arguments(&self) -> &Vec<Value> {
        match &self.0 {
            CallStackItemContent::NamedFunctionInvocation(_, _, arguments) => {
                arguments
            },
            CallStackItemContent::AnonymousFunctionInvocation(_, arguments) => {
                arguments
            },
        }
    }
}
