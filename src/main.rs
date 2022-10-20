mod lib;
use lib::List;
use rand::{thread_rng, Rng};

fn main() {
    let mut linked = List::new();
    for _ in 1..=10 {
        linked.push(thread_rng().gen_range(1..=100));
    }
    println!("{}", linked);
}