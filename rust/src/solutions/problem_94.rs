use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn tranver(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    if let Some(root) = root {
        let node = root.borrow();
        tranver(&node.left, v);
        v.push(node.val);
        tranver(&node.right, v);
    }
}
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = Vec::<i32>::new();
    tranver(&root, &mut ans);
    ans
}
