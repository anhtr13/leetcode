use std::sync::{Arc, Condvar, Mutex};

pub struct DiningPhilosophers {
    cv: Condvar,
    folks: Arc<Mutex<[bool; 5]>>,
}

impl DiningPhilosophers {
    pub fn new() -> Self {
        DiningPhilosophers {
            cv: Condvar::new(),
            folks: Arc::new(Mutex::new([true; 5])),
        }
    }

    // Callbacks are like LeetCode: each used exactly once
    pub fn wants_to_eat<F1, F2, F3, F4, F5>(
        &self,
        philosopher: i32,
        pick_left_fork: F1,
        pick_right_fork: F2,
        eat: F3,
        put_left_fork: F4,
        put_right_fork: F5,
    ) where
        F1: FnOnce(),
        F2: FnOnce(),
        F3: FnOnce(),
        F4: FnOnce(),
        F5: FnOnce(),
    {
        // TODO: implement your dining philosophers solution here
        // You can translate your C++ logic into Rust inside this function.
        let left = philosopher as usize;
        let right = ((philosopher + 1) % 5) as usize;
        let folks = self.folks.clone();

        let mut f = self
            .cv
            .wait_while(folks.lock().unwrap(), |f| !(f[left] && f[right]))
            .unwrap();

        (*f)[left] = false;
        (*f)[right] = false;
        drop(f);
        self.cv.notify_all();

        pick_left_fork();
        pick_right_fork();
        eat();
        put_left_fork();
        put_right_fork();

        let mut f = folks.lock().unwrap();
        (*f)[left] = true;
        (*f)[right] = true;
        drop(f);
        self.cv.notify_all();
    }
}
