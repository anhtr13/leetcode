use std::sync::{Arc, Condvar, Mutex};

pub struct FooBar {
    n: usize,
    flag: Arc<Mutex<bool>>,
    cond: Condvar,
}

impl FooBar {
    pub fn new(n: usize) -> Self {
        FooBar {
            n: n,
            flag: Arc::new(Mutex::new(true)),
            cond: Condvar::new(),
        }
    }

    pub fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let flag = self.flag.clone();
            let mut f = self.cond.wait_while(flag.lock().unwrap(), |f| !*f).unwrap();

            // printFoo() outputs "foo". Do not change or remove this line.
            print_foo();

            *f = false;
            self.cond.notify_all();
        }
    }

    pub fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let flag = self.flag.clone();
            let mut f = self.cond.wait_while(flag.lock().unwrap(), |f| *f).unwrap();

            // printBar() outputs "bar". Do not change or remove this line.
            print_bar();

            *f = true;
            self.cond.notify_all();
        }
    }
}
