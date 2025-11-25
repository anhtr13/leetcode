use crate::Solution;
use crate::data_structures::linked_list::ListNode;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        let mut p = &head;
        while let Some(node) = p {
            let mut x = Box::new(ListNode::new(node.val));
            x.next = res;
            res = Some(x);
            p = &node.next;
        }
        res
    }
}
