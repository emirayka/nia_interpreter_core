use crate::utils::Stack;
use crate::SymbolId;
use crate::Value;
use crate::{CallStackItem, FunctionId};

#[derive(Debug, Clone)]
pub struct CallStack {
    items: Stack<CallStackItem>,
}

impl CallStack {
    pub fn new() -> CallStack {
        CallStack {
            items: Stack::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_not_empty(&self) -> bool {
        self.items.is_not_empty()
    }

    pub fn push_named_function_invocation(
        &mut self,
        function_id: FunctionId,
        function_symbol_id: SymbolId,
        arguments: Vec<Value>,
    ) {
        let call_stack_item = CallStackItem::new_named(
            function_id,
            function_symbol_id,
            arguments,
        );

        self.items.push(call_stack_item)
    }

    pub fn push_anonymous_function_invocation(
        &mut self,
        function_id: FunctionId,
        arguments: Vec<Value>,
    ) {
        let call_stack_item =
            CallStackItem::new_anonymous(function_id, arguments);

        self.items.push(call_stack_item)
    }

    pub fn pop(&mut self) -> Option<CallStackItem> {
        self.items.pop()
    }

    pub fn clear(&mut self) {
        self.items.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[cfg(test)]
    mod len {
        use super::*;

        #[test]
        fn returns_call_stack_length() {
            let mut call_stack = CallStack::new();

            nia_assert_equal(0, call_stack.len());

            call_stack
                .push_anonymous_function_invocation(FunctionId::new(0), vec![]);
            nia_assert_equal(1, call_stack.len());

            call_stack
                .push_anonymous_function_invocation(FunctionId::new(1), vec![]);
            nia_assert_equal(2, call_stack.len());

            call_stack.pop();
            nia_assert_equal(1, call_stack.len());

            call_stack.pop();
            nia_assert_equal(0, call_stack.len());
        }
    }

    #[cfg(test)]
    mod is_empty {
        use super::*;

        #[test]
        fn returns_true_if_empty() {
            let mut stack = Stack::new();

            nia_assert_equal(true, stack.is_empty());

            stack.push(1);
            nia_assert_equal(false, stack.is_empty());

            stack.push(2);
            nia_assert_equal(false, stack.is_empty());

            stack.pop();
            nia_assert_equal(false, stack.is_empty());

            stack.pop();
            nia_assert_equal(true, stack.is_empty());
        }
    }

    #[cfg(test)]
    mod is_not_empty {
        use super::*;

        #[test]
        fn returns_true_if_empty() {
            let mut stack = Stack::new();

            nia_assert_equal(false, stack.is_not_empty());

            stack.push(1);
            nia_assert_equal(true, stack.is_not_empty());

            stack.push(2);
            nia_assert_equal(true, stack.is_not_empty());

            stack.pop();
            nia_assert_equal(true, stack.is_not_empty());

            stack.pop();
            nia_assert_equal(false, stack.is_not_empty());
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod push_anonymous_function_invocation__push_named_function_invocation {
        use super::*;

        #[test]
        fn pushes_anonymous_function_invocation() {
            let mut call_stack = CallStack::new();

            call_stack
                .push_anonymous_function_invocation(FunctionId::new(0), vec![]);
            call_stack
                .push_anonymous_function_invocation(FunctionId::new(1), vec![]);

            nia_assert_equal(
                Some(CallStackItem::new_anonymous(FunctionId::new(1), vec![])),
                call_stack.pop(),
            );
            nia_assert_equal(
                Some(CallStackItem::new_anonymous(FunctionId::new(0), vec![])),
                call_stack.pop(),
            );
        }

        #[test]
        fn pushes_named_function_invocation() {
            let mut call_stack = CallStack::new();

            call_stack.push_named_function_invocation(
                FunctionId::new(0),
                SymbolId::new(1),
                vec![],
            );
            call_stack.push_named_function_invocation(
                FunctionId::new(1),
                SymbolId::new(0),
                vec![],
            );

            nia_assert_equal(
                Some(CallStackItem::new_named(
                    FunctionId::new(1),
                    SymbolId::new(0),
                    vec![],
                )),
                call_stack.pop(),
            );
            nia_assert_equal(
                Some(CallStackItem::new_named(
                    FunctionId::new(0),
                    SymbolId::new(1),
                    vec![],
                )),
                call_stack.pop(),
            );
        }
    }

    #[cfg(test)]
    mod pop {
        use super::*;

        #[test]
        fn returns_last_pushed_item() {
            let mut call_stack = CallStack::new();

            call_stack
                .push_anonymous_function_invocation(FunctionId::new(0), vec![]);
            call_stack.push_named_function_invocation(
                FunctionId::new(1),
                SymbolId::new(0),
                vec![],
            );

            nia_assert_equal(
                Some(CallStackItem::new_named(
                    FunctionId::new(1),
                    SymbolId::new(0),
                    vec![],
                )),
                call_stack.pop(),
            );
            nia_assert_equal(
                Some(CallStackItem::new_anonymous(FunctionId::new(0), vec![])),
                call_stack.pop(),
            );
        }

        #[test]
        fn returns_none_when_no_items_left() {
            let mut call_stack = CallStack::new();

            call_stack.push_named_function_invocation(
                FunctionId::new(1),
                SymbolId::new(0),
                vec![],
            );

            nia_assert_equal(
                Some(CallStackItem::new_named(
                    FunctionId::new(1),
                    SymbolId::new(0),
                    vec![],
                )),
                call_stack.pop(),
            );
            nia_assert_equal(None, call_stack.pop());
        }
    }

    #[cfg(test)]
    mod clear {
        use super::*;

        #[test]
        fn clears_call_stack() {
            let mut call_stack = CallStack::new();

            call_stack
                .push_anonymous_function_invocation(FunctionId::new(1), vec![]);
            call_stack
                .push_anonymous_function_invocation(FunctionId::new(2), vec![]);
            call_stack
                .push_anonymous_function_invocation(FunctionId::new(3), vec![]);
            nia_assert_equal(3, call_stack.len());

            call_stack.clear();
            nia_assert_equal(0, call_stack.len());
        }
    }
}
