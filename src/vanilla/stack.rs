/*creating a dynamically-sized stack*/
pub struct Stack<T> {
    elements: Vec<T>,
    max_size: Option<usize>
}

impl<T> Stack<T> {
    /*initialize a new empty stack*/
    pub fn new(max_size: Option<usize>) -> Self {
        Stack { 
            elements: Vec::new(),
            max_size,
        }
    }

    /*push an element to the stack*/
    pub fn push(&mut self, item: T) -> Result<(), &'static str>{
        if let Some(limit) = self.max_size {
            if self.elements.len() >= limit{
                return Err("Stack overflow: max size reached!");
            }
        }
        self.elements.push(item);
        Ok(())
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

    pub fn get_max(&mut self) -> Option<usize> {
        self.max_size
    }
}