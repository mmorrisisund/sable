use std::borrow::Cow;

use sable_core::handle::NodeHandle;
use slotmap::{SecondaryMap, SlotMap, SparseSecondaryMap};

use crate::Mesh;

#[derive(Debug, Default)]
pub struct Scene {
    nodes: SlotMap<NodeHandle, Node>,
    roots: Vec<NodeHandle>,

    names: SecondaryMap<NodeHandle, Cow<'static, str>>,

    meshes: SparseSecondaryMap<NodeHandle, Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }
}

// Node Management API
impl Scene {
    pub fn nodes(&self) -> Vec<NodeHandle> {
        self.nodes.keys().collect::<Vec<_>>()
    }

    pub fn create_node(&mut self) -> NodeHandle {
        self.nodes.insert(Node::new())
    }

    pub fn get_node(&self, handle: NodeHandle) -> Option<&Node> {
        self.nodes.get(handle)
    }

    pub fn get_node_mut(&mut self, handle: NodeHandle) -> Option<&mut Node> {
        self.nodes.get_mut(handle)
    }

    pub fn attach(&mut self, child: NodeHandle, parent: NodeHandle) {
        if child == parent {
            log::warn!("cannot attach node to itself");
            return;
        }

        // remove from existing parent or root
        if let Some(node) = self.get_node_mut(child) {
            if let Some(parent_handle) = node.parent {
                if let Some(parent_node) = self.get_node_mut(parent_handle) {
                    parent_node.children.retain(|h| *h != child)
                }
            } else {
                self.roots.retain(|h| *h != child);
            }
        }

        // attach to parent or root if parent does not exist (send alert)
        if let Some(parent_node) = self.get_node_mut(parent) {
            parent_node.children.push(child);
        } else {
            log::error!("parent node not found, adding as root");
            self.push_root(child);
        }

        // udpate child
        if let Some(child_node) = self.get_node_mut(child) {
            child_node.parent = Some(parent);
            log::warn!("update transform or mark dirty");
        }
    }

    pub fn remove(&mut self, _child: NodeHandle, _parent: NodeHandle) {
        // find subtree of nodes to remove, starting at child->down.

        // remove from parent or roots

        // remove components and lastly handle
    }

    pub fn push_root(&mut self, handle: NodeHandle) {
        self.roots.push(handle);
    }
}

// Component Management API
impl Scene {
    pub fn get_mesh(&self, handle: NodeHandle) -> Option<&Mesh> {
        self.meshes.get(handle)
    }

    pub fn set_name(&mut self, handle: NodeHandle, name: &str) {
        self.names.insert(handle, Cow::Owned(name.to_string()));
    }

    pub fn set_mesh(&mut self, handle: NodeHandle, mesh: Mesh) {
        self.meshes.insert(handle, mesh);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    parent: Option<NodeHandle>,
    children: Vec<NodeHandle>,
    // transform
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}
