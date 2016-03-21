#[derive(Clone, Debug)]
pub struct Node {
    pub value: bool,
    pub checked: bool,
}

impl Node {
    pub fn new(value: bool) -> Node {
        Node {
            value: value,
            checked: false,
        }
    }
}
