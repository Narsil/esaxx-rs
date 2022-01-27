use esaxx_rs::suffix;
use std::time::Instant;

fn main() {
    let string = std::fs::read_to_string("data/eighty.txt").unwrap();

    let start = Instant::now();
    for i in 0..100 {
        let s = suffix(&string).unwrap().iter().count();
    }
    println!("{:?} ", start.elapsed());
}
