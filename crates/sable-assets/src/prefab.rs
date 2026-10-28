use sable_resources::Mesh;

#[derive(Default)]
pub struct Prefab {
    pub nodes: Vec<PrefabNode>,
    pub root_indices: Vec<usize>,
}

pub struct PrefabNode {
    pub name: Option<String>,
    pub mesh: Option<Mesh>,
    pub children: Vec<usize>,
    // transform
}
