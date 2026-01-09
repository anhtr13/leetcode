use crate::data_structures::linked_list::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut p = ListNode::new(-1);
    p.next = head;
    let mut p = Box::new(p);
    let mut fast = p.clone();
    for _ in 0..n {
        if let Some(node) = fast.next {
            fast = node;
        }
    }
    let mut slow = p.as_mut();
    while let Some(f) = fast.next {
        fast = f;
        slow = slow.next.as_mut().unwrap();
    }
    let remove_node = slow.next.as_mut().unwrap();
    slow.next = remove_node.next.take();
    p.next
}
