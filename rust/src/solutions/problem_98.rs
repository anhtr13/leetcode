use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn check_bst(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(root) = root {
            let node = root.borrow();
            let n_val = node.val as i64;
            if n_val >= max || n_val <= min {
                return false;
            }
            let l = &node.left;
            let r = &node.right;
            return Self::check_bst(l, min, node.val as i64)
                && Self::check_bst(r, node.val as i64, max);
        }
        true
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_bst(&root, i64::MIN, i64::MAX)
    }
}
