use std::cell::RefCell;
use std::collections::HashMap;
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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut counts = HashMap::new();
        let mut max_count = 0;
        let mut modes = Vec::new();
        Self::count_nodes(&root, &mut counts, &mut max_count, &mut modes);
        modes
    }

    pub fn count_nodes(
        root: &Option<Rc<RefCell<TreeNode>>>,
        counts: &mut HashMap<i32, i32>,
        max_count: &mut i32,
        modes: &mut Vec<i32>,
    ) {
        let root = match root {
            Some(r) => r.borrow(),
            None => {
                return;
            }
        };

        let count = *counts.get(&root.val).unwrap_or(&0) + 1;
        counts.insert(root.val, count);

        if count == *max_count {
            modes.push(root.val);
        } else if count > *max_count {
            modes.clear();
            modes.push(root.val);
            *max_count = count;
        }

        Self::count_nodes(&root.left, counts, max_count, modes);
        Self::count_nodes(&root.right, counts, max_count, modes);
    }
}

fn main() {
    let node_one = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    }));
    let node_two = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    }));
    let node_three = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    }));
    let node_four = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    }));
    let node_five = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    }));

    node_one.borrow_mut().left = Some(Rc::clone(&node_four));
    node_one.borrow_mut().right = Some(Rc::clone(&node_two));
    node_two.borrow_mut().left = Some(Rc::clone(&node_three));
    node_two.borrow_mut().right = Some(Rc::clone(&node_five));

    assert_eq!(vec![2,3], Solution::find_mode(Some(node_one)));
}
