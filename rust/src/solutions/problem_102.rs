use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn tranversal_102(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Vec<i32>>, i: usize) {
    if let Some(root) = root {
        if v.len() == i {
            v.push(Vec::<i32>::new());
        }
        let node = root.borrow();
        v[i].push(node.val);
        tranversal_102(&node.left, v, i + 1);
        tranversal_102(&node.right, v, i + 1);
    }
}
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = Vec::<Vec<i32>>::new();
    tranversal_102(&root, &mut ans, 0);
    ans
}
