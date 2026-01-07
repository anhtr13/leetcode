use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn recur_437(
    root: &Option<Rc<RefCell<TreeNode>>>,
    count_table: &mut std::collections::HashMap<i64, u16>,
    target: i64,
    cur_sum: i64,
) -> i64 {
    if let Some(node) = root {
        let node_br = node.borrow();
        let sum = cur_sum + node_br.val as i64;
        let mut count = 0;
        if let Some(x) = count_table.get(&(sum - target))
            && *x > 0
        {
            count += *x as i64;
        }
        count_table.entry(sum).and_modify(|x| *x += 1).or_insert(1);
        let l = recur_437(&node_br.left, count_table, target, sum);
        let r = recur_437(&node_br.right, count_table, target, sum);
        count_table.entry(sum).and_modify(|x| *x -= 1);
        return count + l + r;
    }
    0
}
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    let mut count_table = std::collections::HashMap::<i64, u16>::new();
    count_table.insert(0, 1);
    recur_437(&root, &mut count_table, target_sum as i64, 0) as i32
}
