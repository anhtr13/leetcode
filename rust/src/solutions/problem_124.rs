use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::data_structures::tree::TreeNode;

impl Solution {
    fn find_path_sum(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(root) = root {
            let node = root.borrow();
            let l = Self::find_path_sum(&node.left, ans).max(0);
            let r = Self::find_path_sum(&node.right, ans).max(0);
            *ans = (*ans).max(l + r + node.val);
            return l.max(r) + node.val;
        }
        0
    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = std::i32::MIN;
        Self::find_path_sum(&root, &mut ans);
        ans
    }
}
