use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn tranversal_102(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Vec<i32>>, i: usize) {
        if let Some(root) = root {
            if v.len() == i {
                v.push(Vec::<i32>::new());
            }
            let node = root.borrow();
            v[i].push(node.val);
            Self::tranversal_102(&node.left, v, i + 1);
            Self::tranversal_102(&node.right, v, i + 1);
        }
    }
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        Self::tranversal_102(&root, &mut ans, 0);
        ans
    }
}
