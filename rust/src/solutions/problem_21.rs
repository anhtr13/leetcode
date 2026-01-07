use crate::data_structures::linked_list::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = Box::new(ListNode::new(0));
    let mut p = &mut res.next;
    let (mut l1, mut l2) = (&list1, &list2);
    while let (Some(p1), Some(p2)) = (l1, l2) {
        if p1.val < p2.val {
            if let Some(node) = p {
                node.next = Some(Box::new(ListNode::new(p1.val)));
                p = &mut node.next;
            } else {
                let head = Box::new(ListNode::new(p1.val));
                *p = Some(head);
            }
            l1 = &p1.next;
        } else {
            if let Some(node) = p {
                node.next = Some(Box::new(ListNode::new(p2.val)));
                p = &mut node.next;
            } else {
                let head = Box::new(ListNode::new(p2.val));
                *p = Some(head);
            }
            l2 = &p2.next;
        }
    }
    while let Some(p1) = l1 {
        if let Some(node) = p {
            node.next = Some(Box::new(ListNode::new(p1.val)));
            p = &mut node.next;
        } else {
            let head = Box::new(ListNode::new(p1.val));
            *p = Some(head);
        }
        l1 = &p1.next;
    }
    while let Some(p2) = l2 {
        if let Some(node) = p {
            node.next = Some(Box::new(ListNode::new(p2.val)));
            p = &mut node.next;
        } else {
            let head = Box::new(ListNode::new(p2.val));
            *p = Some(head);
        }
        l2 = &p2.next;
    }
    res.next
}
