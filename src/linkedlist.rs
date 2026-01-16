use pyo3::{pyclass, pymethods};

pub struct Node {
    value: i64,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(input_value: i64) -> Box<Node> {
        Box::new(Node {
            value: input_value,
            next: None,
        })
    }
}

#[pyclass]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

#[pymethods]
impl LinkedList {
    #[new]
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    fn prepend(&mut self, input_value: i64) {
        let mut new_node: Box<Node> = Node::new(input_value);
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    fn append(&mut self, input_value: i64) {
        match self.head.as_mut() {
            None => {
                self.head = Some(Node::new(input_value));
            }
            Some(head) => {
                let mut next_node: &mut Node = head.as_mut();
                while let Some(ref mut next) = next_node.next {
                    next_node = next.as_mut()
                }
                next_node.next = Some(Node::new(input_value));
            }
        }
    }

    fn print(&mut self) {
        match self.head.as_mut() {
            None => {
                println!("Empty list")
            }
            Some(head) => {
                let mut next_node: &mut Node = head.as_mut();
                loop {
                    println!("{}", next_node.value);
                    match next_node.next.as_mut() {
                        None => break,
                        Some(next) => {
                            next_node = next.as_mut();
                        }
                    }
                }
            }
        }
    }

    fn dispose(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut boxed) = cur {
            cur = boxed.next.take()
        }
    }
}
