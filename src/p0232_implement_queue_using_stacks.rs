pub struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.input.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.ensure_front();
        self.output.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        self.ensure_front();
        *self.output.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }

    fn ensure_front(&mut self) {
        if self.output.is_empty() {
            while let Some(value) = self.input.pop() {
                self.output.push(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
    }

    #[test]
    fn new_queue_is_empty() {
        let q = MyQueue::new();
        assert!(q.empty());
    }

    #[test]
    fn push_after_partial_pop_keeps_fifo_order() {
        // Die Folge aus dem Gespräch: ensure_front, dann neu pushen
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.pop(), 1);
        q.push(4);
        q.push(5);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.peek(), 3);
        assert_eq!(q.pop(), 3);
        assert_eq!(q.pop(), 4);
        assert_eq!(q.pop(), 5);
        assert!(q.empty());
    }

    #[test]
    fn peek_does_not_remove() {
        let mut q = MyQueue::new();
        q.push(7);
        assert_eq!(q.peek(), 7);
        assert_eq!(q.peek(), 7);
        assert!(!q.empty());
        assert_eq!(q.pop(), 7);
        assert!(q.empty());
    }

    #[test]
    fn empty_again_then_reuse() {
        let mut q = MyQueue::new();
        q.push(1);
        assert_eq!(q.pop(), 1);
        assert!(q.empty());
        q.push(2);
        q.push(3);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.pop(), 3);
        assert!(q.empty());
    }
}
