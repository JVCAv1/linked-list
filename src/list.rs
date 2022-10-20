pub struct List {
    head: Link,
}

struct Node {
    item: usize,
    next: Link,
}

enum Link {
    Nil,
    Some(Box<Node>),
}