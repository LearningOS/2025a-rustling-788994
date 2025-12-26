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

    // 辅助方法：在当前节点的子树中插入值
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 插入值小于当前节点值 → 插入左子树
            Ordering::Less => {
                if let Some(ref mut left_node) = self.left {
                    left_node.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            // 插入值大于当前节点值 → 插入右子树
            Ordering::Greater => {
                if let Some(ref mut right_node) = self.right {
                    right_node.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            // 相等则不处理（BST 不存储重复值）
            Ordering::Equal => (),
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

    // 插入值到 BST
    fn insert(&mut self, value: T) {
        if let Some(ref mut root_node) = self.root {
            // 根节点存在，调用节点的 insert 方法
            root_node.insert(value);
        } else {
            // 根节点为空，新建根节点
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // 查找值是否存在于 BST 中
    fn search(&self, value: T) -> bool {
        // 从根节点开始查找
        let mut current = &self.root;
        while let Some(ref node) = current {
            match value.cmp(&node.value) {
                // 找到匹配值 → 返回 true
                Ordering::Equal => return true,
                // 查找值更小 → 去左子树找
                Ordering::Less => current = &node.left,
                // 查找值更大 → 去右子树找
                Ordering::Greater => current = &node.right,
            }
        }
        // 遍历完所有节点未找到 → 返回 false
        false
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


