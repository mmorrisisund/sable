# Renderer implementation roadmap

This plan turns [architecture.md](./architecture.md) and the glTF feature checklist in
[renderer.md](./renderer.md) into an incremental implementation for this repository.
Each milestone keeps the previous example working and ends with something runnable;
do not start the next milestone until its build, tests, and visual checkpoint pass.

## Repository shape

Keep the current GPUI application as the window/event-loop host. The root `sable`
package and its `examples/` play the `renderer-app` role from the architecture: GPUI
creates the surface, and `sable-renderer` renders into the supplied `TextureView`.
Do not add a second windowing stack or make the renderer own the GPUI surface.

Grow the workspace only as each responsibility is needed:

```text
sable/
├── Cargo.toml
├── src/
│   └── lib.rs                    # Public facade/re-exports
├── examples/
│   ├── simple.rs                 # Clear-color and surface-lifecycle smoke test
│   ├── triangle.rs               # Renderer API smoke test
│   ├── cube.rs                   # Camera, depth, and scene smoke test
│   └── gltf_viewer.rs            # Loaded-asset visual test (later)
├── crates/
│   ├── sable-renderer/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── renderer.rs       # Frame orchestration and public render entry
│   │       ├── pipeline.rs       # Pipeline creation/configuration
│   │       ├── vertex.rs         # GPU vertex layouts
│   │       ├── mesh.rs           # GPU mesh buffers and draw calls
│   │       ├── depth.rs          # Depth target creation/resizing
│   │       ├── texture.rs        # GPU textures and samplers (when needed)
│   │       ├── pass.rs           # Explicit passes (when needed)
│   │       └── shaders/          # WGSL grouped by shading path
│   ├── sable-core/               # Add at milestone 2; CPU-only data
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── camera.rs
│   │       ├── transform.rs
│   │       ├── mesh.rs
│   │       ├── material.rs
│       │       ├── scene.rs
│       │       ├── lighting.rs           # Add with lighting
│       │       ├── animation.rs          # Add with animation
│       │       └── skin.rs               # Add with skinning
│   └── sable-assets/             # Add at milestone 4; CPU-only file loading
│       ├── src/
│       │   ├── lib.rs
│       │   └── gltf/
│       │       ├── mod.rs
│       │       ├── document.rs
│       │       ├── buffer.rs
│       │       ├── accessor.rs
│       │       ├── mesh.rs
│       │       └── scene.rs
│       └── tests/
│           └── fixtures/
└── tests/                        # Cross-crate tests, if they need a public API
```

Keep the existing workspace glob (`crates/*`) and add internal dependencies only when
the corresponding crate is introduced. `sable-core` must not depend on `wgpu`;
`sable-assets` parses into core types; `sable-renderer` converts core data into GPU
resources. Keep examples small and focused rather than putting renderer implementation
in `examples/simple.rs`.

## Milestones

The API snippets below are staged public-surface sketches: they show what callers
should be able to use after that milestone, not complete implementations to paste in
all at once. Types may gain fields and methods in later milestones. Keep shader,
buffer, pass, parser, and bind-group helpers private unless a concrete application
need justifies exposing them.

### 0. Preserve a known-good clear-color app

- **Files/methods:** Keep `examples/simple.rs` as the GPUI/surface reference. Verify
  the existing `Renderer::new` and `Renderer::render` clear pass in
  `crates/sable-renderer/src/renderer.rs`.
- **Working result:** A GPUI window opens, clears, survives resizing, and presents
  frames. The current `triangle.rs` shader returns the same position for every vertex,
  so it is not yet a valid triangle checkpoint.
- **Gate:** `cargo check --workspace --all-targets`, then run
  `cargo run --example simple`. Keep this example runnable throughout later work.

### 1. Draw a real non-indexed triangle through the library

Implement in this order:

1. Add `crates/sable-renderer/src/shaders/triangle.wgsl` with a vertex entry point
   that emits three distinct clip-space positions and a fragment entry point that
   emits a visible color.
2. Add `vertex.rs` and define the vertex data/layout used by that shader.
3. Refactor `renderer.rs`: create the shader and pipeline, upload the three vertices,
   bind the vertex buffer, and issue the non-indexed draw from `render`.
4. Update `crates/sable-renderer/src/lib.rs` exports as needed and simplify
   `examples/triangle.rs` so its `AppState` delegates rendering to `Renderer`.

- **Working result:** `cargo run --example triangle` displays a colored triangle;
  `simple` still displays the clear-color baseline.
- **Gate:** Workspace check plus a manual visual run. Do not proceed with an empty
  vertex buffer or a shader that returns a constant position.

### 1.1 Public API sketch

These are API targets, not implementation requirements for the current milestone.
Keep GPU implementation types private unless callers need to configure them.

```rust
// crates/sable-renderer/src/lib.rs
pub use renderer::{Renderer, RendererConfig};

// crates/sable-renderer/src/renderer.rs
pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub color_format: wgpu::TextureFormat,
}

pub struct Renderer { /* private GPU state */ }

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: RendererConfig,
    ) -> Self;

    pub fn render(&self, target: &wgpu::TextureView);
}

// src/lib.rs
pub use sable_renderer::{Renderer, RendererConfig};
```

`examples/triangle.rs` should only own GPUI surface/window state, construct
`Renderer`, acquire a view, call `render`, and present it. The shader and vertex
buffer are implementation details, not public API.

### 2. Add CPU scene types, camera, and depth; render a cube

Implement in this order:

1. Add `crates/sable-core` and its workspace dependency.
2. Move the GPU-independent camera concept out of
   `crates/sable-renderer/src/camera.rs` into `sable-core/src/camera.rs`. Add and test
   the view-projection calculation and guard invalid perspective parameters.
3. Add `transform.rs` and `scene.rs` with local transforms and parent-child world
   transform composition; test composition without a GPU.
4. Add `depth.rs` in `sable-renderer`, including creation and resize handling.
5. Add the camera uniform/bind group and depth-enabled pipeline; render a rotating,
   indexed cube in `examples/cube.rs`.

- **Working result:** The cube rotates, nearer faces occlude farther faces, and
  resizing continues to work. Triangle and clear examples remain runnable.
- **Gate:** Core camera/transform tests, workspace check, and visual cube run.

### 2.1 Public API sketch

```rust
// crates/sable-core/src/camera.rs
pub struct Camera { /* view/projection state */ }

impl Camera {
    pub fn perspective(
        position: glam::Vec3,
        target: glam::Vec3,
        fov_y_radians: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Result<Self, CameraError>;

    pub fn view(&self) -> glam::Mat4;
    pub fn projection(&self) -> glam::Mat4;
    pub fn view_projection(&self) -> glam::Mat4;
}

pub enum CameraError {
    InvalidAspectRatio,
    InvalidFieldOfView,
    InvalidClipPlanes,
}

// crates/sable-core/src/transform.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}

impl Transform {
    pub fn matrix(&self) -> glam::Mat4;
    pub fn combine(parent: &Self, local: &Self) -> Self;
}

// crates/sable-core/src/scene.rs
pub struct Node {
    pub transform: Transform,
}

pub struct Scene { /* owned nodes and root IDs */ }
pub struct NodeId(u32);

// crates/sable-core/src/lib.rs
pub use camera::{Camera, CameraError};
pub use scene::{Node, NodeId, Scene};
pub use transform::Transform;

// crates/sable-renderer/src/renderer.rs
impl Renderer {
    pub fn resize(&mut self, width: u32, height: u32);
    pub fn render(&self, camera: &Camera, target: &wgpu::TextureView);
}

// src/lib.rs
pub use sable_core::{Camera, CameraError, Node, NodeId, Scene, Transform};
pub use sable_renderer::{Renderer, RendererConfig};
```

`CameraError` should implement `std::error::Error` and `Display`; construction
validates inputs rather than silently substituting values. Keep `NodeId` opaque in a
full implementation if the scene storage needs to change.

### 3. Make meshes and scene traversal reusable

Implement in this order:

1. Add CPU mesh and material-baseline descriptions to `sable-core/src/mesh.rs` and
   `material.rs`; validate indices and required vertex attributes.
2. Add `sable-renderer/src/mesh.rs` to upload validated mesh data and issue indexed
   or non-indexed draws.
3. Extend `scene.rs` with mesh references and traverse the active scene into explicit
   draw items (world transform, mesh, material).
4. Update `renderer.rs` to render a list of draw items rather than hard-coded
   geometry; keep the single-mesh triangle and cube examples using this path.
5. Add a small multi-node example or extend `cube.rs` to show repeated mesh instances.

- **Working result:** Multiple transformed objects render in one frame and can share
  mesh data without sharing scene transforms.
- **Gate:** CPU tests for invalid indices and world transforms, plus a multi-object
  visual run.

### 3.1 Public API sketch

```rust
// crates/sable-core/src/mesh.rs
pub struct Mesh {
    pub positions: Vec<glam::Vec3>,
    pub normals: Option<Vec<glam::Vec3>>,
    pub indices: Option<Vec<u32>>,
}

impl Mesh {
    pub fn validate(&self) -> Result<(), MeshError>;
}

pub enum MeshError {
    MissingPositions,
    AttributeLengthMismatch,
    IndexOutOfBounds { index: u32, vertex_count: usize },
}

// crates/sable-core/src/material.rs
pub struct Material {
    pub base_color: glam::Vec4,
    pub alpha_mode: AlphaMode,
    pub double_sided: bool,
}

pub enum AlphaMode {
    Opaque,
    Mask { cutoff: f32 },
    Blend,
}

// crates/sable-core/src/scene.rs
pub struct Node {
    transform: Transform,
    mesh: Option<MeshId>,
    material: Option<MaterialId>,
}

impl Node {
    pub fn new(transform: Transform) -> Self;
}

pub struct Scene { /* owns the node graph and root IDs */ }
pub struct NodeId(u32);
pub struct MeshId(u32);
pub struct MaterialId(u32);

impl NodeId {
    pub const fn from_index(index: u32) -> Self;
    pub const fn index(self) -> u32;
}

impl MeshId {
    pub const fn from_index(index: u32) -> Self;
    pub const fn index(self) -> u32;
}

impl MaterialId {
    pub const fn from_index(index: u32) -> Self;
    pub const fn index(self) -> u32;
}

impl Scene {
    pub fn new() -> Self;
    pub fn add_node(&mut self, node: Node) -> NodeId;
    pub fn set_parent(
        &mut self,
        child: NodeId,
        parent: Option<NodeId>,
    ) -> Result<(), SceneError>;
    pub fn set_mesh(&mut self, node: NodeId, mesh: Option<MeshId>)
        -> Result<(), SceneError>;
    pub fn set_material(&mut self, node: NodeId, material: Option<MaterialId>)
        -> Result<(), SceneError>;
    pub fn validate(
        &self,
        meshes: &[Mesh],
        materials: &[Material],
    ) -> Result<(), SceneError>;
}

pub enum SceneError {
    InvalidNode(NodeId),
    MissingMesh(MeshId),
    MissingMaterial(MaterialId),
    ParentCycle,
}

pub struct DrawItem<'a> {
    pub mesh: &'a Mesh,
    pub material: Option<&'a Material>,
    pub world_transform: glam::Mat4,
}

impl Scene {
    pub fn draw_items<'a>(
        &'a self,
        meshes: &'a [Mesh],
        materials: &'a [Material],
    ) -> impl Iterator<Item = DrawItem<'a>>;
}

// crates/sable-renderer/src/renderer.rs
pub struct Frame<'a> {
    pub camera: &'a Camera,
    pub target: &'a wgpu::TextureView,
    pub scene: &'a Scene,
}

impl Renderer {
    pub fn render(&self, frame: &Frame<'_>);
}

// crates/sable-renderer/src/lib.rs
pub use renderer::Frame;

// src/lib.rs
pub use sable_renderer::{Frame, Renderer, RendererConfig};
```

The sketch favors borrowed CPU scene data at the render boundary. GPU mesh caching
and resource handles can be introduced privately without exposing buffer ownership
to application code.

Re-export the intended API from `sable-core/src/lib.rs` and the root `src/lib.rs`;
applications should not need to reach into internal modules to create a scene.

### 4. Load the first glTF geometry on the CPU

Add `sable-assets` only now. Implement in this order:

1. `gltf/document.rs`: parse a document and report malformed/unsupported input as
   explicit errors.
2. `gltf/buffer.rs`: resolve embedded and external buffers with bounds checks.
3. `gltf/accessor.rs`: decode component types, offsets, strides, and sparse overrides.
4. `gltf/mesh.rs`: assemble primitive attributes and optional indices into
   `sable-core` mesh data.
5. `gltf/scene.rs`: resolve the selected scene and node hierarchy into core scene
   data.
6. Add small Khronos fixture assets and tests before wiring loading into
   `examples/gltf_viewer.rs`.

- **Working result:** The viewer loads and renders a basic untextured glTF mesh, while
  parse and bounds errors are visible to the caller rather than silently ignored.
- **Gate:** Fixture tests and a visual run. Initially cover the Phase 1 cases in
  `renderer.md`: non-indexed/indexed triangles, simple materials/meshes, selected
  scenes, boxes and interleaved attributes, URI spaces, vertex colors, sparse
  accessors, primitive modes, negative scale, and graph-size behavior. Add cases in
  small batches; keep the previously supported fixtures passing.

### 4.1 Public API sketch

```rust
// crates/sable-assets/src/lib.rs
pub use gltf::{GltfError, GltfLoadOptions, load_gltf};
pub struct LoadedAsset {
    pub scene: sable_core::Scene,
    pub meshes: Vec<sable_core::Mesh>,
    pub materials: Vec<sable_core::Material>,
}

pub struct GltfLoadOptions {
    pub selected_scene: Option<usize>,
}

pub fn load_gltf(
    path: impl AsRef<std::path::Path>,
    options: &GltfLoadOptions,
) -> Result<LoadedAsset, GltfError>;

pub enum GltfError {
    Io(std::io::Error),
    InvalidDocument(String),
    UnsupportedRequiredExtension(String),
    InvalidBuffer(String),
    InvalidAccessor(String),
}

// src/lib.rs
pub use sable_assets::{GltfError, GltfLoadOptions, LoadedAsset, load_gltf};
```

Keep accessor readers and document internals crate-private: consumers load an asset
and receive validated CPU scene data, not parser implementation details. The error
variants can grow as real parse cases are implemented, but failures must remain
observable.

### 5. Complete static scene and camera behavior

- **Files/methods:** Extend `sable-assets/src/gltf/scene.rs` and `sable-core` camera
  and transform types for perspective/orthographic cameras, glTF coordinate
  conventions, node matrices/TRS, and shared mesh instances. Keep asset parsing
  separate from GPU upload.
- **Working result:** Static multi-node assets, including nested transforms and camera
  selection, render correctly in the viewer.
- **Gate:** Unit tests for camera projections and nested transforms; exercise the
  Phase 2 camera, orientation, hierarchy, and multi-mesh cases from `renderer.md`.

### 5.1 Public API sketch

```rust
// crates/sable-core/src/camera.rs
pub enum Projection {
    Perspective {
        fov_y_radians: f32,
        aspect_ratio: f32,
        near: f32,
        far: Option<f32>,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

impl Camera {
    pub fn from_projection(
        transform: Transform,
        projection: Projection,
    ) -> Result<Self, CameraError>;
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) -> Result<(), CameraError>;
}

// crates/sable-core/src/scene.rs
impl Scene {
    pub fn active_camera(&self) -> Option<NodeId>;
    pub fn world_transform(&self, node: NodeId) -> Option<glam::Mat4>;
}

// crates/sable-assets/src/lib.rs
pub struct LoadedAsset {
    pub scene: sable_core::Scene,
    pub scenes: Vec<sable_core::Scene>,
    pub meshes: Vec<sable_core::Mesh>,
    pub materials: Vec<sable_core::Material>,
}
```

The loader should preserve multiple scenes and choose the glTF default scene unless
`GltfLoadOptions::selected_scene` explicitly selects another. Keep glTF matrix/TRS
decoding in the asset crate and expose normalized core transforms.

### 6. Add textures and complete core material inputs

Implement in this order: extend core material descriptions; parse images, textures,
samplers, UV sets, and texture transforms in `sable-assets`; add GPU texture/sampler
upload and material bind groups in `sable-renderer`; then add shader sampling and
color-space handling. Keep an untextured fallback path working.

- **Working result:** The viewer displays base-color textures and correct sampler/UV
  behavior on at least one fixture.
- **Gate:** Asset fixtures and visual checks for the Phase 3 texture, UV, sampler,
  sRGB, interpolation, and tangent cases in `renderer.md`. Add texture-dependent
  rendering only after the untextured path still works.

### 6.1 Public API sketch

```rust
// crates/sable-core/src/material.rs
pub struct TextureRef {
    pub texture: TextureId,
    pub tex_coord: u32,
}

pub struct PbrMaterial {
    pub base_color_factor: glam::Vec4,
    pub base_color_texture: Option<TextureRef>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
}

pub struct Material {
    pub pbr: PbrMaterial,
    pub alpha_mode: AlphaMode,
    pub double_sided: bool,
}

pub struct TextureId(u32);
pub struct Image { pub bytes: Vec<u8>, pub mime_type: Option<String> }
pub struct Sampler {
    pub wrap_u: WrapMode,
    pub wrap_v: WrapMode,
    pub min_filter: FilterMode,
    pub mag_filter: FilterMode,
}
pub enum WrapMode { ClampToEdge, MirroredRepeat, Repeat }
pub enum FilterMode { Nearest, Linear }

// crates/sable-assets/src/lib.rs
pub struct LoadedAsset {
    pub scene: sable_core::Scene,
    pub meshes: Vec<sable_core::Mesh>,
    pub materials: Vec<sable_core::Material>,
    pub images: Vec<sable_core::Image>,
    pub samplers: Vec<sable_core::Sampler>,
}
```

Keep byte decoding and validation in `sable-assets`; GPU upload and GPU resource
identifiers belong to `sable-renderer` and stay out of the public API.

### 7. Implement calibrated core PBR

- **Files/methods:** Add material factors and light/environment data to core types;
  parse supported glTF core material channels; implement the metallic-roughness
  shader in a dedicated WGSL file; add debug/compare scenes under `examples/` or test
  fixtures.
- **Working result:** A material-factor grid and representative glTF models render
  with stable metallic-roughness shading.
- **Gate:** Validate the Phase 4 PBR models and Phase 5 compare matrix from
  `renderer.md`. Keep material defaults explicit and test CPU-side defaults and
  conversions.

### 7.1 Public API sketch

```rust
// crates/sable-core/src/material.rs
pub struct PbrMaterial {
    pub base_color_factor: glam::Vec4,
    pub base_color_texture: Option<TextureRef>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<TextureRef>,
    pub normal_texture: Option<TextureRef>,
    pub occlusion_texture: Option<TextureRef>,
    pub emissive_factor: glam::Vec3,
    pub emissive_texture: Option<TextureRef>,
}

// crates/sable-core/src/lighting.rs (add when lights are introduced)
pub enum Light {
    Directional { color: glam::Vec3, intensity: f32 },
    Point { color: glam::Vec3, intensity: f32, range: Option<f32> },
    Spot {
        color: glam::Vec3,
        intensity: f32,
        range: Option<f32>,
        inner_cone_radians: f32,
        outer_cone_radians: f32,
    },
}

// crates/sable-renderer/src/renderer.rs
pub struct RenderOptions {
    pub clear_color: wgpu::Color,
}

impl Renderer {
    pub fn render(&self, frame: &Frame<'_>, options: &RenderOptions);
}
```

Treat `RenderOptions` as a small per-frame configuration, not a bag of pipeline
internals. Default PBR factors should be represented by `Default` implementations
and unit-tested.

### 8. Separate frame passes and alpha modes

- **Files/methods:** Extract pass setup from `renderer.rs` into `pass.rs`; establish
  opaque and masked drawing first, then sorted blended drawing, then add shadow or
  post-processing passes only when needed. Keep frame inputs and outputs explicit.
- **Working result:** Opaque, `MASK`, and `BLEND` assets render through the right
  paths without breaking opaque-only scenes.
- **Gate:** Test pass configuration/order and visually verify the Phase 6 alpha and
  emissive cases in `renderer.md`.

### 8.1 Public API sketch

```rust
// crates/sable-renderer/src/renderer.rs
pub struct Frame<'a> {
    pub camera: &'a Camera,
    pub target: &'a wgpu::TextureView,
    pub scene: &'a Scene,
    pub lights: &'a [Light],
}

pub struct RenderOptions {
    pub clear_color: wgpu::Color,
    pub enable_shadows: bool,
}

pub struct Renderer { /* owns GPU pipelines and resources */ }
impl Renderer {
    pub fn render(
        &self,
        frame: &Frame<'_>,
        options: &RenderOptions,
    ) -> Result<(), RenderError>;
}

pub enum RenderError {
    MissingCamera,
    MissingResource(String),
}

// crates/sable-renderer/src/lib.rs
pub use renderer::{Frame, RenderError, RenderOptions, Renderer, RendererConfig};
```

Expose scene/frame intent, not individual pass scheduling. `RenderError` should only
report recoverable invalid frame/resource conditions; GPU validation remains visible
through the `wgpu` error reporting path.

### 9. Add optional glTF extensions incrementally

- **Files/methods:** Keep extension parsing in the relevant `sable-assets/src/gltf/`
  module and extension-specific material/light data in core. Add renderer shader or
  pass support only for an extension that has parsed data and a fixture. Implement
  one extension at a time; unsupported required extensions must return a clear error.
- **Working result:** Every newly supported fixture renders while all core fixtures
  remain passing.
- **Gate:** Work through the Phase 7 groups in `renderer.md` (punctual lighting,
  texture transforms/optimization, material variants, specialized shading, and
  transmission/volume). Treat optional extensions as explicit scope, not a prerequisite
  for core glTF rendering.

### 9.1 Public API sketch

```rust
// crates/sable-assets/src/lib.rs
pub struct GltfLoadOptions {
    pub selected_scene: Option<usize>,
    pub extensions: ExtensionPolicy,
}

pub enum ExtensionPolicy {
    CoreOnly,
    Enable(Vec<GltfExtension>),
}

pub enum GltfExtension {
    PunctualLights,
    TextureTransform,
    MaterialVariants,
    MaterialsUnlit,
    MaterialsClearcoat,
    MaterialsTransmission,
    MaterialsVolume,
    MeshoptCompression,
}

// crates/sable-core/src/material.rs
pub struct MaterialExtensions {
    pub unlit: bool,
    pub clearcoat: Option<Clearcoat>,
    pub transmission: Option<Transmission>,
    pub volume: Option<Volume>,
}

pub struct Clearcoat { pub factor: f32, pub roughness_factor: f32 }
pub struct Transmission { pub factor: f32 }
pub struct Volume { pub thickness_factor: f32, pub attenuation_color: glam::Vec3 }
```

Only add an extension enum variant when it has an actual parser/test path. A required
but disabled or unsupported extension returns `GltfError::UnsupportedRequiredExtension`;
optional unsupported data follows documented loader policy and is never silently
treated as fully supported.

### 10. Add animation, morph targets, and skinning

Implement in this order:

1. Add CPU animation data and deterministic STEP, LINEAR, and CUBICSPLINE sampling;
   test fixed timestamps before connecting it to the frame loop.
2. Apply animation tracks to node transforms and material/visibility properties.
3. Add morph-target data and GPU deformation.
4. Add skin/joint data, inverse bind matrices, and GPU skinning.
5. Extend `gltf_viewer.rs` with a selectable/looping animation.

- **Working result:** Animated node transforms work before morphing; morph fixtures
  work before skinned fixtures. Static rendering remains available.
- **Gate:** Complete the Phase 8 interpolation/morph/node-animation cases, then Phase 9
  skinning cases from `renderer.md`, keeping deterministic CPU tests alongside visual
  checks.

### 10.1 Public API sketch

```rust
// crates/sable-core/src/animation.rs
pub enum Interpolation { Step, Linear, CubicSpline }

pub enum AnimationTarget {
    Translation(NodeId),
    Rotation(NodeId),
    Scale(NodeId),
    Weights(NodeId),
    Visibility(NodeId),
}

pub struct AnimationSampler {
    pub times: Vec<f32>,
    pub interpolation: Interpolation,
    pub values: AnimationValues,
}

pub enum AnimationValues {
    Vec3(Vec<glam::Vec3>),
    Rotation(Vec<glam::Quat>),
    Weights(Vec<Vec<f32>>),
    Visibility(Vec<bool>),
}

pub struct AnimationChannel {
    pub sampler: usize,
    pub target: AnimationTarget,
}

pub struct AnimationClip {
    pub samplers: Vec<AnimationSampler>,
    pub channels: Vec<AnimationChannel>,
}

impl AnimationClip {
    pub fn sample(&self, time: f32, scene: &mut Scene) -> Result<(), AnimationError>;
}

pub enum AnimationError {
    InvalidTime,
    InvalidSampler(usize),
    ValueTypeMismatch,
}

// crates/sable-core/src/mesh.rs
pub struct MorphTarget {
    pub positions: Option<Vec<glam::Vec3>>,
    pub normals: Option<Vec<glam::Vec3>>,
}

// crates/sable-core/src/skin.rs
pub struct Skin {
    pub joints: Vec<NodeId>,
    pub inverse_bind_matrices: Vec<glam::Mat4>,
}

// crates/sable-assets/src/lib.rs
pub struct LoadedAsset {
    pub scene: sable_core::Scene,
    pub meshes: Vec<sable_core::Mesh>,
    pub materials: Vec<sable_core::Material>,
    pub animations: Vec<sable_core::AnimationClip>,
    pub skins: Vec<sable_core::Skin>,
}
```

Keep sampling deterministic and CPU-side. GPU skinning/morph implementation details
remain private to `sable-renderer`; the app selects a clip and advances its time.

## Checkpoint discipline

At the end of every milestone:

1. Run `cargo check --workspace --all-targets` and the targeted crate tests (use
   `cargo test -p sable-core` or `cargo test -p sable-assets` once those crates exist).
2. Run the milestone's example and inspect the actual rendered output; compilation
   alone does not prove a GPU feature works.
3. Keep earlier examples and fixtures working, and document the supported subset and
   known limitations before expanding the next feature group.

The Khronos model names and feature-specific test ideas live in
[renderer.md](./renderer.md); this file defines the implementation order and the
working checkpoint required before moving through that checklist.
