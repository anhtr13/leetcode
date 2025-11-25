use crate::Solution;
use crate::data_structures::linked_list::ListNode;

impl Solution {
    fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut p = head;
        let mut res = Vec::<i32>::new();
        while let Some(node) = p {
            res.push(node.val);
            p = &node.next;
        }
        res
    }
    pub fn add_two_numbers_2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut v1 = Self::list_to_vec(&l1);
        let mut v2 = Self::list_to_vec(&l2);
        if v1.len() > v2.len() {
            (v1, v2) = (v2, v1);
        }
        v1.reverse();
        v2.reverse();
        let mut v = Vec::<i32>::with_capacity(v2.len() + 1);
        let mut r = 0;
        for i in 0..v1.len() {
            let mut x = v1[i] + v2[i] + r;
            if x >= 10 {
                r = 1;
                x = x % 10;
            } else {
                r = 0;
            }
            v.push(x);
        }
        for i in v1.len()..v2.len() {
            let mut x = v2[i] + r;
            if x >= 10 {
                r = 1;
                x = x % 10;
            } else {
                r = 0;
            }
            v.push(x);
        }
        if r == 1 {
            v.push(1);
        }
        let mut res = Box::new(ListNode::new(0));
        let mut p = &mut res.next;
        for x in v.iter().rev() {
            if let Some(node) = p {
                node.next = Some(Box::new(ListNode::new(*x)));
                p = &mut node.next;
            } else {
                let head = Box::new(ListNode::new(*x));
                *p = Some(head);
            }
        }
        res.next
    }
}
