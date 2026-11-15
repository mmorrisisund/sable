use sable_core::{handle::NodeHandle, transform::Transform};

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub(crate) parent: Option<NodeHandle>,
    pub(crate) children: Vec<NodeHandle>,

    pub transform: Transform,
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}
