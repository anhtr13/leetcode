use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn recur_226(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_bm = node.borrow_mut();
            let left = Self::recur_226(&node_bm.left);
            let right = Self::recur_226(&node_bm.right);
            node_bm.left = right;
            node_bm.right = left;
            return Some(node.clone());
        }
        None
    }
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recur_226(&root);
        root
    }
}
