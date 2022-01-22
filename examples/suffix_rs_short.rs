use esaxx_rs::suffix_rs;
use std::time::Instant;

fn main() {
    let string = std::fs::read_to_string("data/eighty.txt").unwrap();

    let start = Instant::now();
    let s = suffix_rs(&string).unwrap().iter().count();
    println!("{:?} {:?}", s, start.elapsed());
}
