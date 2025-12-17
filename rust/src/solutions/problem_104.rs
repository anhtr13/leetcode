use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn count_depth_104(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let node = root.borrow();
            let child = Self::count_depth_104(&node.left).max(Self::count_depth_104(&node.right));
            return child + 1;
        }
        0
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_depth_104(&root)
    }
}
