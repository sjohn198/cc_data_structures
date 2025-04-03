use crate::vanilla::binary_search_tree::BinarySearchTree;

#[test]
fn new_bst() {
    let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
    assert!(bst.is_empty());
}

#[test]
fn insert_one_node() {
    let mut bst: BinarySearchTree<&str> = BinarySearchTree::new();
    bst.insert(12, "Test");
    assert!(bst.contains(12));
    assert_eq!(bst.find(12), Some(&"Test"));
    assert_eq!(bst.tree_height(), 0);
    assert_eq!(bst.size, 1);
}

#[test]
fn insert_many_node() {
    let mut bst: BinarySearchTree<char> = BinarySearchTree::new();
    bst.insert(12, 'a');
    assert!(bst.contains(12));
    assert_eq!(bst.find(12), Some(&'a'));
    assert_eq!(bst.tree_height(), 0);

    bst.insert(10, 'b');
    assert!(bst.contains(10));
    assert_eq!(bst.find(10), Some(&'b'));
    assert_eq!(bst.tree_height(), 1);

    bst.insert(13, 'c');
    assert!(bst.contains(13));
    assert_eq!(bst.find(13), Some(&'c'));
    assert_eq!(bst.tree_height(), 1);

    bst.insert(11, 'd');
    assert!(bst.contains(11));
    assert_eq!(bst.find(11), Some(&'d'));
    assert_eq!(bst.tree_height(), 2);

    assert!(!bst.contains(14));
    assert_eq!(bst.find(14), None);
    assert_eq!(bst.size, 4);
}

#[test]
fn find_min_simple() {
    let mut bst: BinarySearchTree<f64> = BinarySearchTree::new();
    bst.insert(12, 11.12);
    bst.insert(11, 100000.0);
    assert_eq!(bst.find_min(), Ok(11));
    assert_eq!(bst.size, 2);
}

#[test]
fn find_min_more_complex() {
    let mut bst: BinarySearchTree<f64> = BinarySearchTree::new();
    bst.insert(11, 11.11);
    bst.insert(12, 12.12);
    bst.insert(9, 9.99);
    bst.insert(10, 10.0110);
    bst.insert(8, 8.88);
    assert_eq!(bst.find_min(), Ok(8));
    assert_eq!(bst.size, 5);
}

#[test]
fn find_max_simple() {
    let mut bst: BinarySearchTree<f64> = BinarySearchTree::new();
    bst.insert(12, 11.12);
    bst.insert(13, 100000.0);
    assert_eq!(bst.find_max(), Ok(13));
    assert_eq!(bst.size, 2);
}

#[test]
fn find_max_more_complex() {
    let mut bst: BinarySearchTree<f64> = BinarySearchTree::new();
    bst.insert(11, 11.11);
    bst.insert(12, 12.12);
    bst.insert(10, 10.0110);
    bst.insert(16, 16.16);
    bst.insert(15, 15.15);
    bst.insert(17, 17.17);
    bst.insert(14, 14.14);
    assert_eq!(bst.find_max(), Ok(17));
    assert_eq!(bst.size, 7);
}

#[test]
fn find_min_and_max() {
    let mut bst: BinarySearchTree<f64> = BinarySearchTree::new();
    bst.insert(11, 11.11);
    assert_eq!(bst.tree_height(), 0);

    bst.insert(12, 12.12);
    assert_eq!(bst.tree_height(), 1);

    bst.insert(9, 9.99);
    assert_eq!(bst.tree_height(), 1);

    bst.insert(10, 10.0110);
    assert_eq!(bst.tree_height(), 2);

    bst.insert(8, 8.88);
    assert_eq!(bst.tree_height(), 2);

    bst.insert(16, 16.16);
    assert_eq!(bst.tree_height(), 2);

    bst.insert(15, 15.15);
    assert_eq!(bst.tree_height(), 3);

    bst.insert(14, 14.14);
    assert_eq!(bst.tree_height(), 4);

    bst.insert(17, 17.17);
    assert_eq!(bst.tree_height(), 4);

    assert_eq!(bst.find_min(), Ok(8));
    assert_eq!(bst.find_max(), Ok(17));
    assert!(bst.contains(8));
    assert_eq!(bst.size, 9);
}

#[test]
fn find_w_change() {
    let mut bst: BinarySearchTree<u16> = BinarySearchTree::new();
    bst.insert(11, 63);
    assert_eq!(bst.find(11), Some(&63));
    bst.insert(11, 56);
    assert_eq!(bst.find(11), Some(&56));
}

#[test]
fn preorder_test() {
    let mut bst: BinarySearchTree<f32> = BinarySearchTree::new();
    bst.insert(11, 11.11);
    bst.insert(12, 12.12);
    bst.insert(9, 9.99);
    bst.insert(10, 10.0110);
    bst.insert(8, 8.88);
    bst.insert(16, 16.16);
    bst.insert(15, 15.15);
    bst.insert(14, 14.14);
    bst.insert(17, 17.17);

    assert_eq!(bst.preorder_traversal(), vec![11, 9, 8, 10, 12, 16, 15, 14, 17]);
}

#[test]
fn inorder_test() {
    let mut bst: BinarySearchTree<f32> = BinarySearchTree::new();
    bst.insert(11, 11.11);
    bst.insert(12, 12.12);
    bst.insert(9, 9.99);
    bst.insert(10, 10.0110);
    bst.insert(8, 8.88);
    bst.insert(16, 16.16);
    bst.insert(15, 15.15);
    bst.insert(14, 14.14);
    bst.insert(17, 17.17);

    assert_eq!(bst.inorder_traversal(), vec![8, 9, 10, 11, 12, 14, 15, 16, 17]);
}

#[test]
fn postorder_test() {
    let mut bst: BinarySearchTree<f32> = BinarySearchTree::new();
    bst.insert(11, 11.11);
    bst.insert(12, 12.12);
    bst.insert(9, 9.99);
    bst.insert(10, 10.0110);
    bst.insert(8, 8.88);
    bst.insert(16, 16.16);
    bst.insert(15, 15.15);
    bst.insert(14, 14.14);
    bst.insert(17, 17.17);

    assert_eq!(bst.postorder_traversal(), vec![8, 10, 9, 14, 15, 17, 16, 12, 11]);
}

#[test]
fn bfs_test() {
    let mut bst: BinarySearchTree<f32> = BinarySearchTree::new();
    bst.insert(11, 11.11);
    bst.insert(12, 12.12);
    bst.insert(9, 9.99);
    bst.insert(10, 10.0110);
    bst.insert(8, 8.88);
    bst.insert(16, 16.16);
    bst.insert(15, 15.15);
    bst.insert(14, 14.14);
    bst.insert(17, 17.17);

    assert_eq!(bst.bfs_traversal(), vec![11, 9, 12, 8, 10, 16, 15, 17, 14]);
}