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
        if let Some(root) = &mut self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if let Some(root) = &self.root {
            return Self::search_node(root, &value);
        }
        false
    }

    fn search_node(node: &Box<TreeNode<T>>, value: &T) -> bool {
        match value.cmp(&node.value) {
            Ordering::Equal => true,
            Ordering::Less => {
                if let Some(left) = &node.left {
                    Self::search_node(left, value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(right) = &node.right {
                    Self::search_node(right, value)
                } else {
                    false
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {
                // Value already exists, do nothing or replace
                // Here we choose to do nothing for duplicates
                return;
            }
            Ordering::Less => {
                // Insert to the left
                if let Some(left) = &mut self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                // Insert to the right
                if let Some(right) = &mut self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
        }
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


