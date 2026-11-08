use std::borrow::Cow;

use glam::Mat4;
use sable_core::{handle::NodeHandle, transform::Transform};
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

    pub fn update(&mut self) {
        update_heirarchy(&mut self.nodes, &self.roots);
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
            child_node.transform.mark_dirty();
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

    pub transform: Transform,
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}

fn update_heirarchy(nodes: &mut SlotMap<NodeHandle, Node>, roots: &[NodeHandle]) {
    // create stack - (node_h, current_world, changed_b)
    let mut stack = Vec::new();
    // let stack = Vec::<(NodeHandle, Transform, bool)>::new();

    // add roots to stack
    for &root in roots {
        stack.push((root, Mat4::IDENTITY, false));
    }

    // while stack not empty
    while let Some((handle, parent_world, parent_changed)) = stack.pop() {
        // get node and update local_matrix

        let (current_world, world_changed) = {
            let Some(node) = nodes.get_mut(handle) else {
                continue;
            };

            let local_changed = node.transform.update();
            let world_changed = local_changed || parent_changed;

            // if changes then update world matrix
            if world_changed {
                node.transform
                    .set_world_matrix(parent_world * node.transform.local_matrix());
            }

            (node.transform.world_matrix(), world_changed)
        };

        // add childrent to stack
        if let Some(node) = nodes.get(handle) {
            for &child in &node.children {
                stack.push((child, current_world, world_changed));
            }
        }
    }
}
