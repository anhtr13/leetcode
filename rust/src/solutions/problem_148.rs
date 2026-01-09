use crate::data_structures::linked_list::ListNode;

pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = Vec::<i32>::new();
    let mut p = &head;
    while let Some(node) = p {
        vals.push(node.val);
        p = &node.next;
    }
    vals.sort_unstable();
    let mut p = &mut head;
    let mut i = 0;
    while let Some(node) = p {
        node.val = vals[i];
        p = &mut node.next;
        i += 1;
    }
    head
}
