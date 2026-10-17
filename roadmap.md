## Start with a small workspace

Begin with just three crates:

```text
renderer/
├── Cargo.toml
├── crates/
│   ├── renderer-core/   # CPU-side math, mesh and scene types
│   ├── renderer-gpu/    # wgpu device, surface, buffers, pipelines, passes
│   └── renderer-app/    # window, input, event loop, example executable
└── assets/
```

Keep dependencies flowing one way: `renderer-app` uses `renderer-gpu` and `renderer-core`; `renderer-gpu` uses `renderer-core`. Add `renderer-assets` when you start loading files, and an animation crate only if animation grows enough to justify one. Avoid making every future subsystem a crate on day one.

For a first stack, `wgpu` + `winit` + `glam` is a practical, cross-platform choice. Keep GPU types out of core scene and asset data.

## Build in visible, testable milestones

1. **Window and clear color**
   - Create a workspace and a minimal app that opens a window, handles close/resize, and clears the screen.
   - **Test:** run the example; verify resizing and surface recovery after a resize.

2. **Triangle, then cube**
   - Add a pipeline and draw a hard-coded triangle. Then add indexed geometry and a depth buffer to render a rotating cube.
   - **Test:** run a small render example; unit-test any vertex-layout or mesh validation logic.

3. **Camera and scene transforms**
   - Add a camera and CPU-side scene/node types with local transforms and parent-child relationships.
   - **Test:** unit-test transform composition, camera matrices, and invalid scene data without requiring a GPU.

4. **First asset loader**
   - Add `renderer-assets` for file parsing and CPU-side asset data. Start with one format—glTF is a reasonable choice—and load a basic mesh before adding materials and textures.
   - **Test:** parse small fixture files in ordinary tests; check decoded attributes, indices, hierarchy, and useful errors.

5. **Materials and lighting**
   - Add a simple unlit material, then basic lighting, then textures and a more complete metallic-roughness material.
   - **Test:** unit-test material defaults and conversions; use a small set of known models for visual checks.

6. **Multiple passes**
   - Make the frame flow explicit: opaque geometry first, then add one real need at a time, such as shadows or post-processing.
   - **Test:** test CPU-side pass configuration and ordering. Add GPU integration tests only where a compatible test adapter is available.

7. **Animation**
   - Start with deterministic animation sampling, then node transforms, then skinning or morph targets.
   - **Test:** use fixed timestamps and expected values for STEP, LINEAR, and CUBICSPLINE interpolation before relying on visual demos.

## Keep each milestone independently useful

For every step, aim to have:

- a runnable example showing the new behavior;
- unit tests for CPU logic;
- clear validation errors for malformed input or unsupported features;
- a small amount of documentation describing what is supported.

Get the clear-color window and triangle working before designing a general render graph, plugin system, or elaborate resource manager. That gives you an early result on screen while keeping the architecture open for assets, passes, and animation later.
