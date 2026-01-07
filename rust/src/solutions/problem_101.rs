use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn check_symmetric(l: &Option<Rc<RefCell<TreeNode>>>, r: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if *l == None && *r == None {
        return true;
    }
    if let Some(l) = l
        && let Some(r) = r
    {
        let node_l = l.borrow();
        let node_r = r.borrow();
        if node_l.val != node_r.val {
            return false;
        }
        return check_symmetric(&node_l.left, &node_r.right)
            && check_symmetric(&node_l.right, &node_r.left);
    }
    false
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(root) = root {
        let root = root.borrow();
        return check_symmetric(&root.left, &root.right);
    }
    true
}
