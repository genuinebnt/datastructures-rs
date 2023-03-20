#[derive(Debug)]
struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            elem: value,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push_back(&mut self, elem: i32) {
        let mut curr_node = self.head.as_mut();
        while let Some(boxed_node) = curr_node {
            if boxed_node.next.is_none() {
                boxed_node.next = Some(Box::new(Node::new(elem)));
                return;
            }
            curr_node = boxed_node.next.as_mut();
        }
    }

    pub fn push_front(&mut self, elem: i32) {
        let mut new_node = Box::new(Node::new(elem));

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            new_node.next = self.head.take();
            self.head = Some(new_node);
        }
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
    }
}

impl Iterator for List {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}
