/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        let new_node = Box::new(TreeNode::new(value));
        match &mut self.root {
            Some(tmp) => {
                if tmp.value == new_node.value {
                } else if tmp.value < new_node.value {
                    if tmp.left.is_none() {
                        tmp.left = Some(new_node);
                    } else if let Some(tmp_left) = &mut tmp.left {
                        tmp_left.insert(new_node.value);
                    }
                } else {
                    if tmp.right.is_none() {
                        tmp.right = Some(new_node);
                    } else if let Some(tmp_right) = &mut tmp.right {
                        tmp_right.insert(new_node.value);
                    }
                }
            }
            None => {
                self.root = Some(new_node);
            }
        }

        return ();
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if self.root.is_none() {
            return false;
        } else if let Some(tmp) = &self.root {
            if tmp.value == value {
                return true;
            } else if tmp.value < value {
                if let Some(tmp_left) = &tmp.left {
                    return tmp_left.search(value);
                }
            } else {
                if let Some(tmp_right) = &tmp.right {
                    return tmp_right.search(value);
                }
            }
        }

        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        let new_node = Box::new(TreeNode::new(value));
        if self.value < new_node.value {
            if self.left.is_none() {
                self.left = Some(new_node);
            } else if let Some(tmp_left) = &mut self.left {
                tmp_left.insert(new_node.value);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(new_node);
            } else if let Some(tmp_right) = &mut self.right {
                tmp_right.insert(new_node.value);
            }
        }
    }

    fn search(&self, value: T) -> bool {
        if self.value == value {
            return true;
        } else if self.value < value {
            if let Some(tmp_left) = &self.left {
                return tmp_left.search(value);
            }
        } else {
            if let Some(tmp_right) = &self.right {
                return tmp_right.search(value);
            }
        }

        return false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        if let Some(tmp) = &bst.root {
            println!("bst = {:#?}", tmp);
        }
        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


