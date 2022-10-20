mod lib;
use lib::List;
use rand::{thread_rng, Rng};

fn main() {
    let mut linked = List::new();
    for _ in 1..=10 {
        linked.push(thread_rng().gen_range(1..=100));
    }
    println!("Complete list: {}", linked);
    for _ in 1..=10 {
        println!("Popped: {}", linked.pop().unwrap());
    }
}