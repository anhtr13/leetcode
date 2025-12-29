use std::sync::{Arc, Condvar, Mutex};

pub struct ZeroEvenOdd {
    n: i32,
    cur: Arc<Mutex<i32>>,
    lock: Arc<Mutex<u8>>,
    cv: Condvar,
}

impl ZeroEvenOdd {
    pub fn new(n: i32) -> Self {
        ZeroEvenOdd {
            n: n,
            cur: Arc::new(Mutex::new(1)),
            lock: Arc::new(Mutex::new(0)),
            cv: Condvar::new(),
        }
    }

    // printNumber(x) prints the integer x
    pub fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n {
            let lock = self.lock.clone();
            let cur = self.cur.clone();
            let mut l = self
                .cv
                .wait_while(lock.lock().unwrap(), |l| *l != 0)
                .unwrap();
            print_number(0);
            let c = cur.lock().unwrap();
            *l = if *c % 2 == 1 { 1 } else { 2 };
            self.cv.notify_all();
        }
    }

    pub fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in (1..=self.n).step_by(2) {
            let lock = self.lock.clone();
            let cur = self.cur.clone();
            let mut l = self
                .cv
                .wait_while(lock.lock().unwrap(), |l| *l != 1)
                .unwrap();
            let mut c = cur.lock().unwrap();
            print_number(*c);
            *c += 1;
            *l = 0;
            self.cv.notify_all();
        }
    }

    pub fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in (2..=self.n).step_by(2) {
            let lock = self.lock.clone();
            let cur = self.cur.clone();
            let mut l = self
                .cv
                .wait_while(lock.lock().unwrap(), |l| *l != 2)
                .unwrap();
            let mut c = cur.lock().unwrap();
            print_number(*c);
            *c += 1;
            *l = 0;
            self.cv.notify_all();
        }
    }
}
