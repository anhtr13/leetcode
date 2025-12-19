mod data_structures;
mod solutions;

pub struct Solution {}

fn main() {
    let str = "Hello world";
    println!("{str}");
    let b = Solution::word_break(
        "dogs".to_string(),
        vec!["dog".to_string(), "s".to_string(), "gs".to_string()],
    );
    println!("{b}");
}
