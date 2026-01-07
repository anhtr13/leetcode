use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn build_108(nums: &Vec<i32>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if l > r {
        return None;
    }
    let m = (l + r) / 2;
    let mut node = TreeNode::new(nums[m]);
    if l < r {
        if m >= 1 {
            node.left = build_108(nums, l, m - 1);
        }
        if m < nums.len() - 1 {
            node.right = build_108(nums, m + 1, r);
        }
    }
    Some(Rc::new(RefCell::new(node)))
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_108(&nums, 0, nums.len() - 1)
}
