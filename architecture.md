## General architecture

Keep CPU-side data separate from GPU state, and make crate dependencies flow toward rendering and the app:

```mermaid
flowchart TD
    App["renderer-app<br/>Window, input, event loop"]
    Core["renderer-core<br/>Math, scene, transforms, camera"]
    Assets["renderer-assets<br/>File parsing, CPU asset data"]
    Render["renderer-gpu<br/>GPU resources, pipelines, render passes"]
    Anim["renderer-animation<br/>Animation sampling and playback<br/>(add when needed)"]
    Examples["examples<br/>Small runnable demos"]
    Tests["tests and fixtures<br/>CPU tests, asset fixtures, GPU smoke tests"]

    App --> Core
    App --> Render
    App --> Assets
    App --> Anim
    Render --> Core
    Assets --> Core
    Anim --> Core
    Examples --> App
    Examples --> Core
    Examples --> Assets
    Tests --> Core
    Tests --> Assets
    Tests --> Render
```

**Responsibilities and boundaries**

- **`renderer-core`** owns GPU-independent scene and rendering concepts: transforms, cameras, mesh/material descriptions, and handles or IDs. It should be straightforward to unit-test without opening a window or initializing a GPU.
- **`renderer-assets`** parses files into CPU-side asset data. It reports unsupported or malformed content explicitly and does not create GPU resources.
- **`renderer-gpu`** owns the `wgpu` instance/device/surface, GPU resource uploads, pipelines, and render passes. It consumes scene data but should not own the app’s event loop.
- **`renderer-app`** connects windowing and input to the engine: it runs the frame loop, routes input, and coordinates scene updates and rendering.
- **`renderer-animation`** is optional early on. Add it when animation sampling and playback warrant a separate crate; it should update core scene state, not issue GPU commands.

A typical dependency direction is:

```text
renderer-app → renderer-assets, renderer-animation, renderer-gpu
renderer-assets → renderer-core
renderer-animation → renderer-core
renderer-gpu → renderer-core
```

Avoid circular dependencies. In particular, keep parsed asset data and scene types independent of `wgpu` objects; the renderer can turn CPU data into GPU resources when loading or when a resource is first needed.

## Render cycle

```mermaid
flowchart TD
    Start(["Start frame"])
    Events["Poll window events and input"]
    Resize{"Surface resized or lost?"}
    Reconfigure["Reconfigure surface"]
    Update["Update time, controls, and application state"]
    Animate["Advance animations<br/>(when implemented)"]
    Scene["Update scene transforms and visibility"]
    Extract["Extract visible scene data<br/>camera, draw items, lights"]
    Upload["Upload changed resources<br/>buffers, textures"]
    Acquire["Acquire swapchain image"]
    AcquireOK{"Image acquired?"}
    Retry["Handle timeout, outdated, or lost surface"]
    Record["Encode render commands"]
    Opaque["Opaque geometry pass<br/>color + depth"]
    Shadow["Shadow pass<br/>(when implemented)"]
    Transparent["Transparent pass<br/>(when implemented)"]
    Post["Post-processing pass<br/>(when implemented)"]
    Submit["Submit commands to GPU"]
    Present["Present frame"]
    End(["Next frame"])

    Start --> Events --> Resize
    Resize -- Yes --> Reconfigure --> Update
    Resize -- No --> Update
    Update --> Animate --> Scene --> Extract --> Upload --> Acquire
    Acquire --> AcquireOK
    AcquireOK -- No --> Retry --> End
    AcquireOK -- Yes --> Record --> Opaque
    Opaque --> Shadow --> Transparent --> Post --> Submit --> Present --> End
```

The early version can omit animation, shadows, transparency, and post-processing: update a static scene, extract a few draw items, acquire the surface image, draw a triangle or cube, submit, and present. Add passes when a feature needs them, while keeping the frame’s inputs and outputs explicit.
