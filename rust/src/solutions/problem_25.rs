use crate::data_structures::linked_list::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut v = Vec::<i32>::new();
    let mut p = &head;
    while let Some(node) = p {
        v.push(node.val);
        p = &node.next;
    }
    let n = v.len();
    let k = k as usize;
    let mut head = Box::new(ListNode::new(0));
    let mut p = &mut head.next;
    for j in (k..=n).step_by(k) {
        for i in ((j - k)..j).rev() {
            if let Some(node) = p {
                node.next = Some(Box::new(ListNode::new(v[i])));
                p = &mut node.next;
            } else {
                *p = Some(Box::new(ListNode::new(v[i])));
            }
        }
    }
    for i in ((n / k) * k)..n {
        if let Some(node) = p {
            node.next = Some(Box::new(ListNode::new(v[i])));
            p = &mut node.next;
        } else {
            *p = Some(Box::new(ListNode::new(v[i])));
        }
    }
    head.next
}
