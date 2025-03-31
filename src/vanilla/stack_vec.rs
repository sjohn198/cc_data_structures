/*creating a dynamically-sized stack*/
pub struct VectorStack<T> {
    elements: Vec<T>
}

impl<T> VectorStack<T> {
    /*initialize a new empty stack*/
    pub fn new() -> Self {
        VectorStack { elements: Vec::new() }
    }

    /*push an element to the stack*/
    pub fn push(&mut self, item: T){
        self.elements.push(item);
    }

    /*pop an element off the stack*/
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /*look at the top of the stack*/
    pub fn peek(&mut self) -> Option<&T> {
        self.elements.last()
    }

    /*return whether the stack is empty*/
    pub fn is_empty(&mut self) -> bool {
        self.elements.is_empty()
    }

    /*return the number of elements on the stack*/
    pub fn size(&mut self) -> usize {
        self.elements.len()
    }
}