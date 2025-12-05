mod data_structures;
mod solutions;

pub struct Solution {}

fn main() {
    let mut s = String::from("Hello");
    let str = &mut s;
    str.push_str(" world!");
    println!("{s}");

    let v = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let res = Solution::search_matrix(v, 3);
    println!("{res}");
}
