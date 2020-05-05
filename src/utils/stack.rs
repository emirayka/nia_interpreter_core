#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_not_empty(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn clear(&mut self) {
        self.items.clear();
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
        fn returns_correct_length() {
            let mut stack = Stack::new();

            nia_assert_equal(0, stack.len());

            stack.push(1);
            nia_assert_equal(1, stack.len());

            stack.push(1);
            nia_assert_equal(2, stack.len());

            stack.pop();
            nia_assert_equal(1, stack.len());
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

    #[cfg(test)]
    mod push {
        use super::*;

        #[test]
        fn adds_item_to_the_head() {
            let mut stack = Stack::new();

            nia_assert_equal(0, stack.len());

            stack.push(1);
            stack.push(2);
            stack.push(3);

            nia_assert_equal(Some(3), stack.pop());
            nia_assert_equal(Some(2), stack.pop());
            nia_assert_equal(Some(1), stack.pop());
        }
    }

    #[cfg(test)]
    mod pop {
        use super::*;

        #[test]
        fn returns_last_pushed_item() {
            let mut stack = Stack::new();

            stack.push(0);

            stack.push(1);
            nia_assert_equal(Some(1), stack.pop());

            stack.push(2);
            nia_assert_equal(Some(2), stack.pop());

            stack.push(3);
            nia_assert_equal(Some(3), stack.pop());

            nia_assert_equal(Some(0), stack.pop());
        }

        #[test]
        fn returns_none_when_no_more_items_in_the_stack() {
            let mut stack = Stack::new();

            stack.push(0);
            stack.push(1);

            nia_assert_equal(Some(1), stack.pop());
            nia_assert_equal(Some(0), stack.pop());
            nia_assert_equal(None, stack.pop());
        }
    }

    #[cfg(test)]
    mod clear {
        use super::*;

        #[test]
        fn clears_call_stack() {
            let mut call_stack = Stack::new();

            call_stack.push(1);
            call_stack.push(2);
            call_stack.push(3);
            nia_assert_equal(3, call_stack.len());

            call_stack.clear();
            nia_assert_equal(0, call_stack.len());
        }
    }
}
