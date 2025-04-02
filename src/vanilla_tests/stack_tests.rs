use crate::vanilla::stack::Stack;
/*---------------Testing Stack Implementation W/O Max Size-------------*/
#[test]
fn test_new_stack() {
    let mut stack: Stack<i32> = Stack::new(None);
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
}

#[test]
fn push_one_to_stack() {
    let mut stack: Stack<i32> = Stack::new(None);
    assert!(stack.push(2).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&2));
}

#[test]
fn push_many_to_stack() {
    let mut stack: Stack<i32> = Stack::new(None);
    assert!(stack.push(2).is_ok());
    assert!(stack.push(4).is_ok());
    assert!(stack.push(8).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&8));
}

#[test]
fn pop_one_from_stack() {
    let mut stack: Stack<&str> = Stack::new(None);
    assert!(stack.push("Testing").is_ok());
    assert_eq!(stack.pop(), Some("Testing"));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.peek(), None);
}

#[test]
fn pop_many_from_stack() {
    let mut stack: Stack<u16> = Stack::new(None);
    assert!(stack.push(700).is_ok());
    assert!(stack.push(699).is_ok());
    assert!(stack.push(698).is_ok());
    assert!(stack.push(697).is_ok());

    assert_eq!(stack.pop(), Some(697));
    assert_eq!(stack.pop(), Some(698));
    assert_eq!(stack.pop(), Some(699));
    assert_eq!(stack.pop(), Some(700));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.peek(), None);
}

#[test]
fn large_test() {
    let mut stack: Stack<f32> = Stack::new(None);
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.peek(), None);

    assert!(stack.push(0.678).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&0.678));

    assert!(stack.push(12.3214).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&12.3214));

    assert!(stack.push(12356787.5432).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&12356787.5432));

    assert!(stack.push(12.0).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 4);
    assert_eq!(stack.peek(), Some(&12.0));

    assert!(stack.push(79679.5463456).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 5);
    assert_eq!(stack.peek(), Some(&79679.5463456));

    assert!(stack.push(45.45).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 6);
    assert_eq!(stack.peek(), Some(&45.45));

    assert!(stack.push(0.0001).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 7);
    assert_eq!(stack.peek(), Some(&0.0001));

    assert!(stack.push(11.11).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 8);
    assert_eq!(stack.peek(), Some(&11.11));

    assert!(stack.push(9.87654321).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 9);
    assert_eq!(stack.peek(), Some(&9.87654321));

    assert!(stack.push(1.0).is_ok());
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

/*-------------Testing Stack Implementation W/ Max Size-------------*/
#[test]
fn test_new_stack_fixed() {
    let mut stack: Stack<i32> = Stack::new(None);
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
}

#[test]
fn push_one_to_stack_fixed() {
    let mut stack: Stack<i32> = Stack::new(Some(1));
    assert!(stack.push(2).is_ok());
    assert!(stack.push(3).is_err());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&2));
}

#[test]
fn pop_many_from_stack_fixed() {
    let mut stack: Stack<u16> = Stack::new(Some(3));
    assert!(stack.push(700).is_ok());
    assert!(stack.push(699).is_ok());
    assert!(stack.push(698).is_ok());
    assert!(stack.push(697).is_err());

    assert_eq!(stack.pop(), Some(698));
    assert_eq!(stack.pop(), Some(699));
    assert_eq!(stack.pop(), Some(700));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.peek(), None);
}

#[test]
fn large_test_fixed() {
    let mut stack: Stack<f32> = Stack::new(Some(9));
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.peek(), None);

    assert!(stack.push(0.678).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&0.678));

    assert!(stack.push(12.3214).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&12.3214));

    assert!(stack.push(12356787.5432).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&12356787.5432));

    assert!(stack.push(12.0).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 4);
    assert_eq!(stack.peek(), Some(&12.0));

    assert!(stack.push(79679.5463456).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 5);
    assert_eq!(stack.peek(), Some(&79679.5463456));

    assert!(stack.push(45.45).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 6);
    assert_eq!(stack.peek(), Some(&45.45));

    assert!(stack.push(0.0001).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 7);
    assert_eq!(stack.peek(), Some(&0.0001));

    assert!(stack.push(11.11).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 8);
    assert_eq!(stack.peek(), Some(&11.11));

    assert!(stack.push(9.87654321).is_ok());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 9);
    assert_eq!(stack.peek(), Some(&9.87654321));

    assert!(stack.push(1.0).is_err());
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 9);
    assert_eq!(stack.peek(), Some(&9.87654321));

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

/*-------------Testing PartialEq-------------*/
#[test]
fn partial_equality() {
    let stack: Stack<i16> = Stack::new(Some(4));
    let stack2: Stack<i16> = Stack::new(Some(4));

    assert_eq!(stack, stack2);
}