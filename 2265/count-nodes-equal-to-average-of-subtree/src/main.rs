use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution {}

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Self::post_order_traversal(&root, &mut result);
        result
    }

    fn post_order_traversal(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32) {
        if let Some(node) = node {
            let (left_sum, left_count) = Self::post_order_traversal(&node.borrow().left, result);
            let (right_sum, right_count) = Self::post_order_traversal(&node.borrow().right, result);
            let current_val = node.borrow().val;
            let count = left_count + right_count + 1;
            let sum = left_sum + right_sum + current_val;
            let average = sum / count;
            if average == current_val {
                *result += 1;
            }
            return (sum, count);
        }
        (0, 0)
    }
}
fn main() {
    let node_4 = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    }));
    let node_8 = Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: None,
        right: None,
    }));
    let node_5 = Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: None,
    }));
    let node_0 = Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: None,
        right: None,
    }));
    let node_1 = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    }));
    let node_6 = Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: None,
        right: None,
    }));
    (*node_4).borrow_mut().left = Some(Rc::clone(&node_8));
    (*node_4).borrow_mut().right = Some(Rc::clone(&node_5));
    (*node_8).borrow_mut().left = Some(Rc::clone(&node_0));
    (*node_8).borrow_mut().right = Some(Rc::clone(&node_1));
    (*node_5).borrow_mut().right = Some(Rc::clone(&node_6));
    assert_eq!(Solution::average_of_subtree(Some(node_4)), 5);
}
