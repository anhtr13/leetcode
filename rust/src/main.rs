fn main() {
    let str = "Hello world";
    println!("{str}");
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    leetcode_rust::solutions::problem_48::rotate(&mut matrix);
    for row in matrix {
        println!("{row:?}");
    }
}
