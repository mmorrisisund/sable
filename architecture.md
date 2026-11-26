Engine to Orchestrate Flow
 - Will hold input state, assets, scene management, and renderer
 - Application will create and own the engine
 - Each frame the application will use engine to handle input, update scene, and render to surface
 - gpui will the composite with root view


Camera Stuff
- Scene has generic camera with settings and matrices
- Simplified render camera can be extracted from this
- Camera is just a node (which has a transform)
  - Basically used to hold view/projection matrices (mostly)
- Different camera controllers take Input and transform, then modify transform


Rendering flow
- Engine runs the flow
- Renderer holds everything that needs managed. resource managers, shader/pipeline caches, passes
- Rendering starts with begin_frame in Renderer.  The extracts gpu resources, pass prep, create renderables (commands).
  - Returns a Frame to conduct the actual drawing
- Frame.render gets everything ready and calls passes
- end_frame for cleanup.  not in use yet

Otter uses begin_frame to create renderable data (via RenderFrame(frame mgmt) ExtractedScene(renderable data)).
  - Frame manager has extract_and_prepare cycle
  - Frame data and scene data used to create render commands
    - Commands are everything needed to render a single object
    - mesh bg, geometry, material, pipeline(id) 
  - Then passes are configured
  - Lastly, Frame returned. references to basically everything gpu resources, caches, passes

Frame Manager extract
 - Pull rendering data from Scene
 - Global Prepare - upload camera/scene/light uniforms (Global BindGroup(0))
   - Gpu Binding on resource manager

Scene Extraction
 - Collect active meshes
   - based on __visible__ scene meshes
   - get mesh data: node matrix, world_matrix, prev_world?, world_aabb eventually
 - Each active mesh
   - Prepare Mesh (geo/mat) to get mesh bg
   - Create Renderable (active mesh + geo/mat resources, few other things)
     - world mat, world aabb, mesh bg, geo/mat handles (not gpu versions)

Prepare Mesh
 - Creates BindGroup 2
 - Prepare Geo/Mat
   - afs
 - Get mesh bindgroup, or create new one if changed
 - Returns binding context: layout, bg, wgsl

Prepare Geo
  - get from cache if exists (stored in rm)
  - prepare attrs
  - if structure new or changed build gpu_geo

Build Gpu_Geo
  - generate layout stuff
  - get list of gpu_bufs based on layout
  - GpuGeo: layout, vert bufs, idx buf, draw ranges

Prepare Gpu_Mesh
  - used to generate mat bind group
  - lots of stuff like gpu_geo (bg, layouts, etc.)