use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn recur_119(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>, level: usize) {
        if let Some(node) = root {
            let node_bm = node.borrow();
            if res.len() == level {
                res.push(node_bm.val);
            }
            Self::recur_119(&node_bm.right, res, level + 1);
            Self::recur_119(&node_bm.left, res, level + 1);
        }
    }

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::<i32>::new();
        Self::recur_119(&root, &mut res, 0);
        res
    }
}
