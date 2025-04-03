use std::cmp::max;
use crate::vanilla::queue::Queue;

/*TODO?: Remove node function? */

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
    pub root: Option<Box<Node<T>>>,
    pub size: usize
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0
        }
    }

    /*return whether the tree has any nodes*/
    pub fn is_empty(&mut self) -> bool {
        self.root.is_none()
    }

    /*helper function for contains and find. Walks the tree looking for a certain key */
    fn search_helper(node: &Option<Box<Node<T>>>, key: i32) -> Option<&Box<Node<T>>> {
        match node {
            Some(box_node) => {
                if box_node.key == key {
                    Some(box_node)
                } else if box_node.key > key {
                    Self::search_helper(&box_node.left, key)
                } else {
                    Self::search_helper(&box_node.right, key)
                }
            }
            None => None
        }
    }

    /*returns whether the tree contains a certain key */
    pub fn contains(&mut self, key: i32) -> bool {
        match Self::search_helper(&self.root, key) {
            Some(_) => true,
            None => false
        }
    }

    /*returns the data associated with a key if it exists. Otherwise, None */
    pub fn find(&mut self, key: i32) -> Option<&T> {
        match Self::search_helper(&self.root, key) {
            Some(box_node) => {
                Some(&box_node.data)
            }
            None => None
        }
    }

    /*insert a new node into the tree */
    pub fn insert(&mut self, key: i32, data: T){
        if self.root.is_none() {
            let box_node: Box<Node<T>> = Box::new(Node::new(key, data, None, None));
            self.root = Some(box_node);
            self.size += 1;
        } else{
            Self::insert_helper(&mut self.root, key, data); 
            self.size += 1;
        }
    }

    /*recursive helper function for insert */
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

    /*finds the minimum key value in the tree */
    pub fn find_min(&mut self) -> Result<i32, &'static str> {
        Self::find_min_helper(&self.root)
    }

    /*recursive helper for find_min */
    fn find_min_helper(node: &Option<Box<Node<T>>>) -> Result<i32, &'static str> {
        match node {
            Some(box_node) => {
                if box_node.left.is_none() {
                    Ok(box_node.key)
                } else {
                    Self::find_min_helper(&(box_node.left))
                }
            }
            None => Err("Attempting to find min of empty tree")
        }
    }

    /*find the maximum key value in the tree */
    pub fn find_max(&mut self) -> Result<i32, &'static str> {
        Self::find_max_helper(&self.root)
    }

    /*recursive helper for find_max */
    fn find_max_helper(node: &Option<Box<Node<T>>>) -> Result<i32, &'static str> {
        match node {
            Some(box_node) => {
                if box_node.right.is_none() {
                    Ok(box_node.key)
                } else {
                    Self::find_max_helper(&(box_node.right))
                }
            }
            None => Err("Attempting to find max of empty tree")
        }
    }

    /*finds the height of the tree */
    pub fn tree_height(&mut self) -> i32 {
        Self::tree_height_helper(&self.root)
    }

    /*recursive helper for tree_height */
    fn tree_height_helper(node: &Option<Box<Node<T>>>) -> i32 {
        match node {
            Some(box_node) => {
                if box_node.left.is_none() && box_node.right.is_none() {
                    0
                } else {
                    1 + max(Self::tree_height_helper(&box_node.left), Self::tree_height_helper(&box_node.right))
                }
            }
            None => 0
        }
    }

    /*-------------Tree Traversal Algorithms-------------*/
    /*Traverses the tree in a depth-first in-order manner, i.e. left, node, right */
    pub fn inorder_traversal(&mut self) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::with_capacity(self.size);
        Self::inorder_helper(&self.root, &mut vec);
        vec
    }

    fn inorder_helper(node: &Option<Box<Node<T>>>, vec: &mut Vec<i32>) {
        match node {
            Some(box_node) => {
                Self::inorder_helper(&box_node.left, vec);
                vec.push(box_node.key);
                Self::inorder_helper(&box_node.right, vec);
            }
            None => ()
        }
    }

    /*Traverses the tree in a depth-first pre-order manner, i.e. node, left, right */
    pub fn preorder_traversal(&mut self) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::with_capacity(self.size);
        Self::preorder_helper(&self.root, &mut vec);
        vec
    }

    fn preorder_helper(node: &Option<Box<Node<T>>>, vec: &mut Vec<i32>) {
        match node {
            Some(box_node) => {
                vec.push(box_node.key);
                Self::preorder_helper(&box_node.left, vec);
                Self::preorder_helper(&box_node.right, vec);
            }
            None => ()
        }
    }

    /*Traverses the tree in a depth-first in-order manner, i.e. left, right, node */
    pub fn postorder_traversal(&mut self) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::with_capacity(self.size);
        Self::postorder_helper(&self.root, &mut vec);
        vec
    }

    fn postorder_helper(node: &Option<Box<Node<T>>>, vec: &mut Vec<i32>) {
        match node {
            Some(box_node) => {
                Self::postorder_helper(&box_node.left, vec);
                Self::postorder_helper(&box_node.right, vec);
                vec.push(box_node.key);
            }
            None => ()
        }
    }

    /*Traverses the tree in a breadth-first manner.
    This gets super complicated with error handling from my queue returning a result.
    TODO: simplify queue max_size property */
    pub fn bfs_traversal(&mut self) -> Vec<i32> {
        let mut q: Queue<&Option<Box<Node<T>>>> = Queue::new(None); 
        let mut trav: Vec<i32> = Vec::with_capacity(self.size);

        match q.enqueue(&self.root) {
            Ok(_) => {},
            Err(_) => return Vec::new()
        };

        while !q.is_empty() {
            let temp = q.dequeue();
            match temp {
                Some(option_node) => {
                    match option_node {
                        Some(box_node) => {
                            trav.push(box_node.key);

                            match q.enqueue(&box_node.left) {
                                Ok(_) => {},
                                Err(_) => return Vec::new()
                            };

                            match q.enqueue(&box_node.right) {
                                Ok(_) => {},
                                Err(_) => return Vec::new()
                            };
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
        trav
    }
}
