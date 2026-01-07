use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn count_depth_104(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root) = root {
        let node = root.borrow();
        let child = count_depth_104(&node.left).max(count_depth_104(&node.right));
        return child + 1;
    }
    0
}
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    count_depth_104(&root)
}
