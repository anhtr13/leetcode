use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn flat(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_bm = node.borrow_mut();
            let left = node_bm.left.take();
            let right = node_bm.right.take();
            let left_end = Self::flat(&mut left.clone());
            let right_end = Self::flat(&mut right.clone());
            if !left.clone().is_none()
                && let Some(node_left_end) = left_end.clone()
            {
                node_bm.right = left;
                if !right.clone().is_none() {
                    node_left_end.borrow_mut().right = right;
                    return right_end;
                }
                return left_end;
            }
            if !right.is_none() {
                node_bm.right = right;
                return right_end;
            }
            return Some(node.clone());
        }
        None
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::flat(root);
    }
}
