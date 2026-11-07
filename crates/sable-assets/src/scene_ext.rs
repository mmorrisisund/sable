use sable_core::handle::NodeHandle;
use sable_resources::Scene;

use crate::Prefab;

/// Extensions to Scene to avoid circular dependencies between crates
pub trait SceneExt {
    /// Add prefab nodes to Scene.  Returns a virtual root node to the model
    fn instantiate_prefab(&mut self, prefab: &Prefab) -> NodeHandle;
}

impl SceneExt for Scene {
    fn instantiate_prefab(&mut self, prefab: &Prefab) -> NodeHandle {
        let mut node_map = Vec::new();

        for p_node in &prefab.nodes {
            let handle = self.create_node();

            if let Some(name) = &p_node.name {
                self.set_name(handle, name);
            }

            if let Some(mesh) = &p_node.mesh {
                self.set_mesh(handle, mesh.clone());
            }

            node_map.push(handle);
        }

        for (i, p_node) in prefab.nodes.iter().enumerate() {
            let parent_handle = node_map[i];

            for &child_idx in &p_node.children {
                let child_handle = node_map[child_idx];
                self.attach(child_handle, parent_handle);
            }
        }

        // Create virtual root
        let root_handle = self.create_node();
        self.push_root(root_handle);

        for &root_idx in &prefab.root_indices {
            let handle = node_map[root_idx];
            self.attach(handle, root_handle);
        }

        root_handle
    }
}
