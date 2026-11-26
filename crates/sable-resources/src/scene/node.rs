use sable_core::{handle::NodeHandle, transform::Transform};

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) parent: Option<NodeHandle>,
    pub(crate) children: Vec<NodeHandle>,

    pub transform: Transform,
    pub visible: bool,
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            transform: Transform::default(),
            visible: true,
        }
    }
}
