use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn tranver(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(root) = root {
            let node = root.borrow();
            Self::tranver(&node.left, v);
            v.push(node.val);
            Self::tranver(&node.right, v);
        }
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::<i32>::new();
        Self::tranver(&root, &mut ans);
        ans
    }
}
