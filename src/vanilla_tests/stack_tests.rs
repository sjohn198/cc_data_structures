use crate::vanilla::stack_vec::VectorStack;

#[test]
fn test_new_stack() {
    let mut stack: VectorStack<i32> = VectorStack::new();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
}

#[test]
fn push_one_to_stack() {
    let mut stack: VectorStack<i32> = VectorStack::new();
    stack.push(2);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&2));
}

#[test]
fn push_many_to_stack() {
    let mut stack: VectorStack<i32> = VectorStack::new();
    stack.push(2);
    stack.push(4);
    stack.push(8);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&8));
}

#[test]
fn pop_one_from_stack() {
    let mut stack: VectorStack<&str> = VectorStack::new();
    stack.push("Testing");
    assert_eq!(stack.pop(), Some("Testing"));

}