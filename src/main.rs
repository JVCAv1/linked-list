mod lib;
use lib::List;
use rand::random;

fn main() {
    let mut linked = List::new();
    for _ in 1..=10 {
        linked.push(random())
    }
    println!("{:?}", linked.pop()); //TODO
}