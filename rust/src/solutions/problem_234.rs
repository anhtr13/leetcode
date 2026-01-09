use crate::data_structures::linked_list::ListNode;

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut v = Vec::<i32>::new();
    let mut p = &head;
    while let Some(node) = p {
        v.push(node.val);
        p = &node.next;
    }
    let n = v.len();
    for i in 0..(n / 2) {
        if v[i] != v[n - 1 - i] {
            return false;
        }
    }
    true
}
