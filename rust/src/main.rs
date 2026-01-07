fn main() {
    let str = "Hello world";
    println!("{str}");
    let v = vec![3, 2, 1, 5, 6, 4];
    let x = leetcode_rust::solutions::problem_215::find_kth_largest(v, 2);
    println!("{x}");
}
