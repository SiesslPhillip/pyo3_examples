use pyo3::{PyResult, pyclass, pymethods};

pub struct Leaf {
    value: i64,
    left: Option<Box<Leaf>>,
    right: Option<Box<Leaf>>,
}

#[pyclass]
pub struct BinaryTree {
    root: Option<Box<Leaf>>,
}

impl Leaf {
    fn new(input_value: i64) -> Box<Leaf> {
        Box::new(Leaf {
            value: input_value,
            left: None,
            right: None,
        })
    }
}

fn go_left(input_leaf: &Box<Leaf>, input_value: i64) -> bool {
    match input_leaf.left.as_ref() {
        None => return true,
        Some(leaf) => input_value < leaf.value,
    }
}

fn add_value_to_leaf(input_leaf: &mut Box<Leaf>, input_value: i64) {
    if go_left(input_leaf, input_value) {
        match input_leaf.left.as_mut() {
            None => input_leaf.left = Some(Leaf::new(input_value)),
            Some(left) => add_value_to_leaf(left, input_value),
        }
    } else {
        match input_leaf.right.as_mut() {
            None => input_leaf.right = Some(Leaf::new(input_value)),
            Some(right) => add_value_to_leaf(right, input_value),
        }
    }
}

fn print_leaf(input_leaf: &Option<Box<Leaf>>) {
    match input_leaf {
        None => return,
        Some(leaf) => {
            println!("{}", leaf.value);
            print_leaf(&leaf.left);
            print_leaf(&leaf.right)
        }
    }
}

fn leaf_contains(input_leaf: &Option<Box<Leaf>>, search_value: i64) -> bool {
    match input_leaf {
        None => false,
        Some(leaf) => {
            if leaf.value == search_value {
                true
            } else if go_left(leaf, search_value) {
                leaf_contains(&leaf.left, search_value)
            } else {
                leaf_contains(&leaf.right, search_value)
            }
        }
    }
}

#[pymethods]
impl BinaryTree {
    #[new]
    fn new() -> BinaryTree {
        BinaryTree { root: None }
    }

    fn add_value(&mut self, input_value: i64) {
        match self.root.as_mut() {
            None => self.root = Some(Leaf::new(input_value)),
            Some(root) => add_value_to_leaf(root, input_value),
        }
    }

    fn contains(&self, search_value: i64) -> PyResult<bool> {
        Ok(leaf_contains(&self.root, search_value))
    }

    fn print(&self) {
        print_leaf(&self.root)
    }
}
