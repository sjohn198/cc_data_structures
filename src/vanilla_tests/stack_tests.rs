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
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.peek(), None);
}

#[test]
fn pop_many_from_stack() {
    let mut stack: VectorStack<u16> = VectorStack::new();
    stack.push(700);
    stack.push(699);
    stack.push(698);
    stack.push(697);

    assert_eq!(stack.pop(), Some(697));
    assert_eq!(stack.pop(), Some(698));
    assert_eq!(stack.pop(), Some(699));
    assert_eq!(stack.pop(), Some(700));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.peek(), None);
}

#[test]
fn large_test() {
    let mut stack: VectorStack<f32> = VectorStack::new();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.peek(), None);

    stack.push(0.678);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&0.678));

    stack.push(12.3214);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&12.3214));

    stack.push(12356787.5432);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&12356787.5432));

    stack.push(12.0);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 4);
    assert_eq!(stack.peek(), Some(&12.0));

    stack.push(79679.5463456);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 5);
    assert_eq!(stack.peek(), Some(&79679.5463456));

    stack.push(45.45);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 6);
    assert_eq!(stack.peek(), Some(&45.45));

    stack.push(0.0001);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 7);
    assert_eq!(stack.peek(), Some(&0.0001));

    stack.push(11.11);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 8);
    assert_eq!(stack.peek(), Some(&11.11));

    stack.push(9.87654321);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 9);
    assert_eq!(stack.peek(), Some(&9.87654321));

    stack.push(1.0);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 10);
    assert_eq!(stack.peek(), Some(&1.0));

    assert_eq!(stack.pop(), Some(1.0));
    assert_eq!(stack.size(), 9);
    assert_eq!(stack.peek(), Some(&9.87654321));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(9.87654321));
    assert_eq!(stack.size(), 8);
    assert_eq!(stack.peek(), Some(&11.11));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(11.11));
    assert_eq!(stack.size(), 7);
    assert_eq!(stack.peek(), Some(&0.0001));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(0.0001));
    assert_eq!(stack.size(), 6);
    assert_eq!(stack.peek(), Some(&45.45));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(45.45));
    assert_eq!(stack.size(), 5);
    assert_eq!(stack.peek(), Some(&79679.5463456));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(79679.5463456));
    assert_eq!(stack.size(), 4);
    assert_eq!(stack.peek(), Some(&12.0));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(12.0));
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&12356787.5432));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(12356787.5432));
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&12.3214));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(12.3214));
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&0.678));
    assert!(!stack.is_empty());

    assert_eq!(stack.pop(), Some(0.678));
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.peek(), None);
    assert!(stack.is_empty());
}