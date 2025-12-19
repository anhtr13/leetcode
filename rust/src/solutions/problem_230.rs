use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn recur_230(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_br = node.borrow();
            v.push(node_br.val);
            Self::recur_230(&node_br.left, v);
            Self::recur_230(&node_br.right, v);
        }
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut v = Vec::<i32>::new();
        Self::recur_230(&root, &mut v);
        v.sort_unstable_by(|a, b| a.cmp(b));
        v[k as usize - 1]
    }
}
