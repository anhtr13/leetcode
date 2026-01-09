use crate::data_structures::linked_list::ListNode;

pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut p = &mut head;
    while let Some(first) = p {
        p = &mut first.next;
        if let Some(second) = p {
            let x = second.val;
            second.val = first.val;
            first.val = x;
            p = &mut second.next;
        } else {
            break;
        }
    }
    head
}
