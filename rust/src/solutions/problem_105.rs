use crate::{Solution, data_structures::tree::TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn build_105(
        preorder: &Vec<i32>,
        inorder_idx: &std::collections::HashMap<i32, usize>,
        pre_start: i32,
        in_start: i32,
        in_end: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start >= preorder.len() as i32 || in_start > in_end {
            return None;
        }
        let node_val = preorder[pre_start as usize];
        let in_pos = *inorder_idx.get(&node_val).unwrap_or(&0) as i32;
        if in_pos < in_start || in_pos > in_end {
            return None;
        }
        let mut node = TreeNode::new(node_val);
        node.left = Self::build_105(preorder, inorder_idx, pre_start + 1, in_start, in_pos - 1);
        node.right = Self::build_105(
            preorder,
            inorder_idx,
            pre_start + (in_pos - in_start) + 1,
            in_pos + 1,
            in_end,
        );
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_idx: std::collections::HashMap<i32, usize> = inorder
            .iter()
            .enumerate()
            .map(|(idx, &val)| (val, idx))
            .collect();
        Self::build_105(&preorder, &inorder_idx, 0, 0, inorder.len() as i32 - 1)
    }
}
