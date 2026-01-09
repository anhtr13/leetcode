use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct LRUCacheNode {
    key: i32,
    val: i32,
    front: Option<Rc<RefCell<LRUCacheNode>>>,
    back: Option<Rc<RefCell<LRUCacheNode>>>,
}

impl LRUCacheNode {
    fn new(k: i32, v: i32) -> Self {
        Self {
            key: k,
            val: v,
            front: None,
            back: None,
        }
    }
}

pub struct LRUCache {
    head: Option<Rc<RefCell<LRUCacheNode>>>,
    tail: Option<Rc<RefCell<LRUCacheNode>>>,
    cache: HashMap<i32, Rc<RefCell<LRUCacheNode>>>,
    len: usize,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            head: None,
            tail: None,
            cache: HashMap::<i32, Rc<RefCell<LRUCacheNode>>>::new(),
            len: 0,
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key)
            && let Some(tail) = self.tail.clone()
        {
            if tail.borrow().key != key {
                if let Some(front) = &node.borrow().front {
                    front.borrow_mut().back = node.borrow().back.clone();
                }
                if let Some(back) = &node.borrow().back {
                    back.borrow_mut().front = node.borrow().front.clone();
                }
                if let Some(head) = self.head.clone()
                    && head.borrow().key == key
                {
                    if let Some(new_head) = &head.borrow().back {
                        new_head.borrow_mut().front = None;
                        self.head = Some(new_head.clone());
                    }
                }
                let mut node_bm = node.borrow_mut();
                node_bm.back = None;
                node_bm.front = Some(tail.clone());
                tail.borrow_mut().back = Some(node.clone());
                self.tail = Some(node.clone());
            }
            return node.borrow().val;
        }
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let node = self.cache.get(&key);
        if let Some(node) = node {
            node.borrow_mut().val = value;
            if let Some(tail) = self.tail.clone()
                && tail.borrow().key == key
            {
                return;
            }
            if let Some(front) = &node.borrow().front {
                front.borrow_mut().back = node.borrow().back.clone();
            }
            if let Some(back) = &node.borrow().back {
                back.borrow_mut().front = node.borrow().front.clone();
            }
            if let Some(head) = self.head.clone()
                && head.borrow().key == key
            {
                if let Some(next) = &head.borrow().back {
                    next.borrow_mut().front = None;
                    self.head = Some(next.clone());
                }
            }
            if let Some(tail) = self.tail.clone() {
                let mut node_brm = node.borrow_mut();
                node_brm.val = value;
                node_brm.back = None;
                node_brm.front = Some(tail.clone());
                tail.borrow_mut().back = Some(node.clone());
                self.tail = Some(node.clone());
            }
        } else {
            let new_node = Rc::new(RefCell::new(LRUCacheNode::new(key, value)));
            self.cache.insert(key, new_node.clone());
            if self.len < self.capacity {
                self.len += 1;
                if let Some(tail) = &self.tail {
                    tail.borrow_mut().back = Some(new_node.clone());
                    new_node.borrow_mut().front = Some(tail.clone());
                    self.tail = Some(new_node);
                } else {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            } else {
                if self.capacity == 1
                    && let Some(head) = self.head.clone()
                {
                    let mut head_brm = head.borrow_mut();
                    self.cache.remove(&head_brm.key);
                    head_brm.key = key;
                    head_brm.val = value;
                }
                if let Some(head) = self.head.clone()
                    && let Some(tail) = self.tail.clone()
                    && let mut head_brm = head.borrow_mut()
                    && let Some(new_head) = head_brm.back.clone()
                {
                    self.cache.remove(&head_brm.key);
                    head_brm.back = None;
                    new_head.borrow_mut().front = None;
                    self.head = Some(new_head.clone());
                    tail.borrow_mut().back = Some(new_node.clone());
                    new_node.borrow_mut().front = Some(tail);
                    self.tail = Some(new_node);
                }
            }
        }
    }
}
