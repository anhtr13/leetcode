use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn recur_236(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
    res: &mut Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if let Some(node) = root {
        let node_br = node.borrow();
        let flag = node_br.val == p || node_br.val == q;
        let l = recur_236(&node_br.left, p, q, res);
        let r = recur_236(&node_br.right, p, q, res);
        if (l && r) || (flag && (l || r)) {
            *res = Some(node.clone());
        }
        return flag || l || r;
    }
    false
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(p) = p
        && let Some(q) = q
    {
        let mut res: Option<Rc<RefCell<TreeNode>>> = None;
        recur_236(&root, p.borrow().val, q.borrow().val, &mut res);
        return res;
    }
    None
}
