mod data_structures;
mod solutions;

pub struct Solution {}

fn main() {
    let str = "Hello world";
    println!("{str}");

    use crate::solutions::problem_1116;
    let zeo1 = std::sync::Arc::new(problem_1116::ZeroEvenOdd::new(5));
    let zeo2 = zeo1.clone();
    let zeo3 = zeo1.clone();

    let t1 = std::thread::spawn(move || {
        zeo1.zero(|x| {
            println!("{x}");
        });
    });
    let t2 = std::thread::spawn(move || {
        zeo2.even(|x| {
            println!("{x}");
        });
    });
    let t3 = std::thread::spawn(move || {
        zeo3.odd(|x| {
            println!("{x}");
        });
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
