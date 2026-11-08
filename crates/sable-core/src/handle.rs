use slotmap::new_key_type;

new_key_type! {
    pub struct PrefabHandle;
    pub struct GeometryHandle;
    pub struct MaterialHandle;
    pub struct NodeHandle;
}
