use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    item: usize,
    next: Link,
}

enum Link {
    Nil,
    Som(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Nil }
    }

    pub fn push(&mut self, item: usize) {
        self.head = Link::Som(Box::new(Node {
            item,
            next: mem::replace(&mut self.head, Link::Nil),
        }))
    }

    pub fn pop(&mut self) -> Option<usize> {
        unimplemented!()
    }
}
