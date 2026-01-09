use crate::data_structures::linked_list::ListNode;

fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = Box::new(ListNode::new(0));
    let mut p = &mut res.next;
    let mut p1 = &l1;
    let mut p2 = &l2;
    while let Some(h1) = p1
        && let Some(h2) = p2
    {
        let mut val = h1.val;
        if h1.val < h2.val {
            p1 = &h1.next;
        } else {
            val = h2.val;
            p2 = &h2.next;
        }
        if let Some(head) = p {
            head.next = Some(Box::new(ListNode::new(val)));
            p = &mut head.next;
        } else {
            *p = Some(Box::new(ListNode::new(val)));
        }
    }
    while let Some(h1) = p1 {
        if let Some(head) = p {
            head.next = Some(Box::new(ListNode::new(h1.val)));
            p = &mut head.next;
        } else {
            *p = Some(Box::new(ListNode::new(h1.val)));
        }
        p1 = &h1.next;
    }
    while let Some(h2) = p2 {
        if let Some(head) = p {
            head.next = Some(Box::new(ListNode::new(h2.val)));
            p = &mut head.next;
        } else {
            *p = Some(Box::new(ListNode::new(h2.val)));
        }
        p2 = &h2.next;
    }
    res.next
}

fn divide_and_conquer(
    lists: &mut Vec<Option<Box<ListNode>>>,
    l: usize,
    r: usize,
) -> Option<Box<ListNode>> {
    if l == r {
        return lists[l].take();
    }
    if l + 1 == r {
        return merge(lists[l].take(), lists[r].take());
    }
    let m = (l + r) / 2;
    return merge(
        divide_and_conquer(lists, l, m),
        divide_and_conquer(lists, m + 1, r),
    );
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let n = lists.len();
    if n == 0 {
        return None;
    }
    let mut lists = lists.clone();
    return divide_and_conquer(&mut lists, 0, n - 1);
}
