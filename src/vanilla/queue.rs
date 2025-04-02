use std::collections::VecDeque;
use std::fmt::{self, Debug, Formatter};

pub struct Queue<T> {
    elements: VecDeque<T>,
    max_size: Option<usize>
}

impl<T> Queue<T>{
    /*initialize a new queue with optional max_size argument*/
    pub fn new(max_size: Option<usize>) -> Self{
        Queue {
            elements: VecDeque::new(),
            max_size
        }
    }

    /*enqueue an item if possible*/
    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if let Some(limit) = self.max_size {
            if self.elements.len() >= limit {
                return Err("Queue overflow: max size reached");
            }
        }
        self.elements.push_back(item);
        Ok(())
    }

    /*remove an element from the head of the queue */
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /*return a reference to the element at the head of the queue */
    pub fn peek(&mut self) -> Option<&T> {
        self.elements.front()
    }

    /*return a boolean indicating whether the queue is empty */
    pub fn is_empty(&mut self) -> bool {
        self.elements.is_empty()
    }

    /*return the number of elements in the queue */
    pub fn size(&mut self) -> usize {
        self.elements.len()
    }

    /*return the option representing the max_size if it was set */
    pub fn get_max(&mut self) -> Option<usize> {
        self.max_size
    }
}

/*equality by value. used for testing */
impl<T: PartialEq> PartialEq for Queue<T> {
    fn eq(&self, other: &Self) ->  bool {
        self.elements == other.elements && self.max_size == other.max_size
    }
}

/*print formatter for debugging */
impl<T: Debug> Debug for Queue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Queue")
            .field("elements", &self.elements)
            .field("max_size", &self.max_size)
            .finish()
    }
}