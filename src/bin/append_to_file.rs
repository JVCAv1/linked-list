use linked_list::List;
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut linked = List::new();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("linked_list.txt")
		.expect("File Error!");

    for _ in 1..=10 {
        linked.push(thread_rng().gen_range(1..=100));
    }
	for _ in 1..=10 {
		writeln!(file, "{}", linked.pop().unwrap()).expect("Write Error!");
	}
}
