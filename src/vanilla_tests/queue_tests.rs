use crate::vanilla::queue::Queue;

/*I'm aware that I use some ridiculous types in these tests, 
I just wanted to learn about new types that Rust has to offer*/

/*-------------Testing Queue Implementation W/O Max Size-------------*/
#[test]
fn new_queue() {
    let mut q: Queue<f32> = Queue::new(None);
    assert!(q.is_empty());
    assert_eq!(q.size(), 0);
}

#[test]
fn enqueue_one() {
    let mut q: Queue<bool> = Queue::new(None);
    assert!(q.enqueue(true).is_ok());
    assert!(!q.is_empty());
    assert_eq!(q.size(), 1);
}

#[test]
fn enqueue_many() {
    let mut q: Queue<Result<(), &'static str>> = Queue::new(None);
    assert!(q.enqueue(Ok(())).is_ok());
    assert!(!q.is_empty());
    assert_eq!(q.size(), 1);
    assert_eq!(q.peek(), Some(&Ok(())));

    assert!(q.enqueue(Err("idk why you would want a queue of result values")).is_ok());
    assert!(!q.is_empty());
    assert_eq!(q.size(), 2);
    assert_eq!(q.peek(), Some(&Ok(())));

    assert!(q.enqueue(Err("ig maybe if you wanted to track and deal with the return values later")).is_ok());
    assert!(!q.is_empty());
    assert_eq!(q.size(), 3);
    assert_eq!(q.peek(), Some(&Ok(())));

    assert!(q.enqueue(Err("like maybe the function was called asynchronously")).is_ok());
    assert!(!q.is_empty());
    assert_eq!(q.size(), 4);
    assert_eq!(q.peek(), Some(&Ok(())));
}

#[test]
fn dequeue_one() {
    let mut q: Queue<Queue<i16>> = Queue::new(None);
    assert!(q.enqueue(Queue::new(None)).is_ok());
    assert!(!q.is_empty());
    assert_eq!(q.size(), 1);
    assert_eq!(q.peek(), Some(&Queue::new(None)));

    assert_eq!(q.dequeue(), Some(Queue::new(None)));
    assert!(q.is_empty());
    assert_eq!(q.size(), 0);
    assert_eq!(q.peek(), None);
}

#[test]
fn dequeue_many() {
    let mut q: Queue<f64> = Queue::new(None);
    assert!(q.enqueue(1.3).is_ok());
    assert!(q.enqueue(2.4).is_ok());
    assert!(q.enqueue(5.6).is_ok());

    assert_eq!(q.size(), 3);
    assert!(!q.is_empty());
    assert_eq!(q.peek(), Some(&1.3));

    assert_eq!(q.dequeue(), Some(1.3));
    assert_eq!(q.size(), 2);
    assert!(!q.is_empty());
    assert_eq!(q.peek(), Some(&2.4));
    
    assert_eq!(q.dequeue(), Some(2.4));
    assert_eq!(q.size(), 1);
    assert!(!q.is_empty());
    assert_eq!(q.peek(), Some(&5.6));

    assert_eq!(q.dequeue(), Some(5.6));
    assert_eq!(q.size(), 0);
    assert!(q.is_empty());
    assert_eq!(q.peek(), None);
}

/*-------------Testing Queue Implementation W/ Max Size-------------*/
#[test]
fn new_queue_fixed() {
    let mut q: Queue<char> = Queue::new(Some(3));
    assert!(q.is_empty());
    assert_eq!(q.size(), 0);
    assert_eq!(q.peek(), None);
}

#[test]
fn enqueue_one_fixed() {
    let mut q: Queue<Queue<Queue<u16>>> = Queue::new(Some(1));
    assert!(q.enqueue(Queue::new(Some(3))).is_ok());
    assert_eq!(q.size(), 1);
    assert!(!q.is_empty());
    assert_eq!(q.peek(), Some(&Queue::new(Some(3))));
    assert_ne!(q.peek(), Some(&Queue::new(Some(2))));

    assert!(q.enqueue(Queue::new(Some(1))).is_err());
}

#[test]
fn enqueue_many_fixed() {
    let mut q: Queue<Option<i32>> = Queue::new(Some(3));
    assert!(q.enqueue(Some(8)).is_ok());
    assert!(q.enqueue(Some(32)).is_ok());
    assert!(q.enqueue(Some(100)).is_ok());
    assert!(q.enqueue(Some(8)).is_err());
    
    assert_eq!(q.size(), 3);
    assert_eq!(q.peek(), Some(&Some(8)));
}
