use crate::data_structures::linked_list::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = Box::new(ListNode::new(0));
    let mut p = &mut res.next;
    let mut r = 0;
    let (mut l1, mut l2) = (&l1, &l2);
    while let (Some(p1), Some(p2)) = (l1, l2) {
        let mut x = p1.val + p2.val + r;
        if x >= 10 {
            x = x % 10;
            r = 1;
        } else {
            r = 0;
        }
        if let Some(node) = p {
            node.next = Some(Box::new(ListNode::new(x)));
            p = &mut node.next;
        } else {
            *p = Some(Box::new(ListNode::new(x)));
        }
        l1 = &p1.next;
        l2 = &p2.next;
    }
    while let Some(p1) = l1 {
        let mut x = p1.val + r;
        if x >= 10 {
            x = x % 10;
            r = 1;
        } else {
            r = 0;
        }
        if let Some(node) = p {
            node.next = Some(Box::new(ListNode::new(x)));
            p = &mut node.next;
        } else {
            *p = Some(Box::new(ListNode::new(x)));
        }
        l1 = &p1.next;
    }
    while let Some(p2) = l2 {
        let mut x = p2.val + r;
        if x >= 10 {
            x = x % 10;
            r = 1;
        } else {
            r = 0;
        }
        if let Some(node) = p {
            node.next = Some(Box::new(ListNode::new(x)));
            p = &mut node.next;
        } else {
            *p = Some(Box::new(ListNode::new(x)));
        }
        l2 = &p2.next;
    }
    if r > 0 {
        if let Some(node) = p {
            node.next = Some(Box::new(ListNode::new(r)));
        } else {
            *p = Some(Box::new(ListNode::new(r)));
        }
    }
    res.next
}
