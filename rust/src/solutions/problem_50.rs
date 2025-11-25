use crate::Solution;

impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        if n == i32::MIN {
            x = x * x;
            n = n / 2;
        }
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }
        if n == 0 {
            return 1.0;
        }
        if n % 2 == 1 {
            return x * Self::my_pow(x * x, n / 2);
        }
        return Self::my_pow(x * x, n / 2);
    }
}
