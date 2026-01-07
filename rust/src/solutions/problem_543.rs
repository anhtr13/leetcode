use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn recur_543(root: &Option<Rc<RefCell<TreeNode>>>, longest_path: &mut i32) -> i32 {
    if let Some(node) = root {
        let node_br = node.borrow();
        let l = recur_543(&node_br.left, longest_path);
        let r = recur_543(&node_br.right, longest_path);
        *longest_path = (*longest_path).max(l + r);
        return l.max(r) + 1;
    }
    0
}
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    recur_543(&root, &mut res);
    res
}
