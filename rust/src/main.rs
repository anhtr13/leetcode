mod data_structures;
mod solutions;

pub struct Solution {}

fn main() {
    let str = "Hello world";
    println!("{str}");

    let v = vec![2, 2, 1, 1];
    let b = Solution::can_partition(v);
    print!("{b}");
}
