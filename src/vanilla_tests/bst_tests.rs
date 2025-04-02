use crate::vanilla::binary_search_tree::BinarySearchTree;

#[test]
fn new_bst() {
    let mut bst: BinarySearchTree<i32> = BinarySearchTree::new(None);
    assert!(bst.is_empty());
}

#[test]
fn insert_one_node() {
    let mut bst: BinarySearchTree<String> = BinarySearchTree::new(None);
    bst.insert(12, "Test".to_string());
    assert!(bst.search(12));
}

#[test]
fn insert_many_node() {
    let mut bst: BinarySearchTree<char> = BinarySearchTree::new(None);
    bst.insert(12, 'a');
    assert!(bst.search(12));

    bst.insert(10, 'b');
    assert!(bst.search(10));

    bst.insert(13, 'c');
    assert!(bst.search(13));

    bst.insert(11, 'd');
    assert!(bst.search(11));

    assert!(!bst.search(14));
}