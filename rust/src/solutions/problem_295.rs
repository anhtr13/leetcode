use std::cmp::Reverse;

pub struct MedianFinder {
    max_heap: std::collections::BinaryHeap<i32>, // smaller part, in desc order
    min_heap: std::collections::BinaryHeap<Reverse<i32>>, // greatter part, in asc order
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            max_heap: std::collections::BinaryHeap::new(),
            min_heap: std::collections::BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.min_heap.push(Reverse(num));
        if self.min_heap.len() == self.max_heap.len()
            && let Some(a) = self.max_heap.pop()
            && let Some(Reverse(i)) = self.min_heap.pop()
        {
            if a > i {
                self.max_heap.push(i);
                self.min_heap.push(Reverse(a));
            } else {
                self.max_heap.push(a);
                self.min_heap.push(Reverse(i));
            }
        } else if let Some(Reverse(i)) = self.min_heap.pop() {
            self.max_heap.push(i);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len()
            && let Some(a) = self.max_heap.peek()
        {
            return *a as f64;
        }
        if let Some(a) = self.max_heap.peek()
            && let Some(Reverse(i)) = self.min_heap.peek()
        {
            return ((a + i) as f64) / 2.0;
        }
        0.0
    }
}
