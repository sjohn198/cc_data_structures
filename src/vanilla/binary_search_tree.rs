pub struct Node<T> {
    key: i32,
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(key: i32, data: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Self {
        Node {
            key, 
            data,
            left,
            right
        }
    }
}

pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>
}

impl<T> BinarySearchTree<T> {
    pub fn new(root: Option<Box<Node<T>>>) -> Self {
        BinarySearchTree {
            root
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.root.is_none()
    }

    pub fn search(&mut self, key: i32) -> bool {
        Self::search_helper(&self.root, key)
    }

    fn search_helper(node: &Option<Box<Node<T>>>, key: i32) -> bool {
        match node {
            Some(box_node) => {
                if box_node.key == key {
                    true
                } else if Self::search_helper(&box_node.left, key) {
                    true
                } else if Self::search_helper(&box_node.right, key) {
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }

    pub fn insert(&mut self, key: i32, data: T){
        if self.root.is_none() {
            let box_node: Box<Node<T>> = Box::new(Node::new(key, data, None, None));
            self.root = Some(box_node);
        } else{
            Self::insert_helper(&mut self.root, key, data)
        }
    }

    fn insert_helper(node: &mut Option<Box<Node<T>>>, key: i32, data: T) {
        match node {
            Some(box_node) => {
                if box_node.key == key {
                    box_node.data = data;
                } else if box_node.key > key {
                    if (&box_node.left).is_none() {
                        let child: Box<Node<T>> = Box::new(Node::new(key, data, None, None));
                        box_node.left = Some(child);
                    } else {
                        Self::insert_helper(&mut box_node.left, key, data)
                    }
                } else {
                    /*current node key is less than the inserted key*/
                    if (&box_node.right).is_none() {
                        let child: Box<Node<T>> = Box::new(Node::new(key, data, None, None));
                        box_node.right = Some(child);
                    } else {
                        Self::insert_helper(&mut box_node.right, key, data)
                    }
                }
            }
            None => ()
        }
    }
}
