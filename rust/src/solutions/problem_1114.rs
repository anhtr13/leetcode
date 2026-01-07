use std::sync::{Arc, Condvar, Mutex};

pub struct Foo {
    count: Arc<Mutex<u8>>,
    cond: Condvar,
}

impl Foo {
    pub fn new() -> Self {
        Foo {
            count: Arc::new(Mutex::new(1)),
            cond: Condvar::new(),
        }
    }

    pub fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        // Do not change this line
        print_first();
        let count = self.count.clone();
        *count.lock().unwrap() = 2;
        self.cond.notify_all();
    }

    pub fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        let count = self.count.clone();
        let mut c = count.lock().unwrap();
        while *c != 2 {
            c = self.cond.wait(c).unwrap();
        }
        // Do not change this line
        print_second();
        *c = 3;
        self.cond.notify_all();
    }

    pub fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        let count = self.count.clone();
        let mut c = count.lock().unwrap();
        while *c != 3 {
            c = self.cond.wait(c).unwrap();
        }
        // Do not change this line
        print_third();
    }
}
