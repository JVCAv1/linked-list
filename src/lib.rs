use core::fmt;
use std::mem::replace;

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

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Nil }
    }

    pub fn push(&mut self, item: usize) {
        self.head = Link::Som(Box::new(Node {
            item,
            next: replace(&mut self.head, Link::Nil),
        }))
    }

    pub fn pop(&mut self) -> Option<usize> {
        match replace(&mut self.head, Link::Nil) {
            Link::Nil => None,
            Link::Som(Node) => {
                self.head = Node.next;
                Some(Node.item)
            }
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        while let Link::Som(node) = current {
            write!(f, "{} -> ", node.item)?;
            current = &node.next;
        }
        write!(f, "Nil")
    }
}
