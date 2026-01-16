use pyo3::{pyclass, pymethods};

pub struct MorseLeaf {
    ch: char,
    left: Option<Box<MorseLeaf>>,
    right: Option<Box<MorseLeaf>>,
}

#[pyclass]
pub struct BinaryMorseTree {
    left: Option<Box<MorseLeaf>>,
    right: Option<Box<MorseLeaf>>,
}

impl MorseLeaf {
    fn new(input_value: char) -> Box<MorseLeaf> {
        Box::new(MorseLeaf {
            ch: input_value,
            left: None,
            right: None,
        })
    }
}

fn go_left(input_leaf: &mut MorseLeaf, input_code: &str) -> bool {
    match input_leaf.left.as_ref() {
        None => true,
        Some(_) => input_code.starts_with("."),
    }
}

fn add_value_to_leaf(node: &mut Option<Box<MorseLeaf>>, value: char, code: &str) {
    let leaf = node.get_or_insert_with(|| {
        Box::new(MorseLeaf {
            ch: '\0',
            left: None,
            right: None,
        })
    });

    if code.is_empty() {
        leaf.ch = value;
        return;
    }

    let rest = &code[1..];

    if go_left(leaf.as_mut(), code) {
        add_value_to_leaf(&mut leaf.left, value, rest);
    } else {
        add_value_to_leaf(&mut leaf.right, value, rest);
    }
}

fn print_leaf(input_leaf: &Option<Box<MorseLeaf>>) {
    match input_leaf {
        None => return,
        Some(leaf) => {
            println!("{}", leaf.ch);
            print_leaf(&leaf.left);
            print_leaf(&leaf.right)
        }
    }
}

// fn leaf_contains(input_leaf:  &Option<Box<Leaf>>, search_value: i64) -> bool {
//     match input_leaf {
//         None => false,
//         Some(leaf) => {
//             if leaf.value == search_value {
//                 true
//             } else if go_left(leaf, search_value) {
//                 leaf_contains(&leaf.left, search_value)
//             } else {
//                 leaf_contains(&leaf.right, search_value)
//             }
//         }
//     }
// }

#[pymethods]
impl BinaryMorseTree {
    #[new]
    fn new() -> BinaryMorseTree {
        BinaryMorseTree {
            left: None,
            right: None,
        }
    }

    fn add_value(&mut self, ch: char, code: String) {
        let code_str: &str = code.as_str();
        match &code_str[..1] {
            "." => add_value_to_leaf(&mut self.left, ch, &code_str[1..]),
            "-" => add_value_to_leaf(&mut self.right, ch, &code_str[1..]),
            _ => {
                println!("Unsupported Code")
            }
        }
    }

    // fn contains(&self, search_value: i64) -> PyResult<bool> {
    //     Ok(leaf_contains(&self.root, search_value))
    // }

    fn print(&self) {
        print_leaf(&self.left);
        print_leaf(&self.right)
    }
}
