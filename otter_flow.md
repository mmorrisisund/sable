# Otter Render Flow
* Window Event
  * [Update](#update)
    * Application Update
    * Engine Update
  * [Render](#render)
  * [End Frame](#end-frame)


## Update
- Get frame timing details (to send to application)
- Allow application to update (game logic, physics, etc.)
- Have engine update
  - Process loading events (handles background thread loading events)
  - Update internal frame timing
  - Allow Scene to Update if it exists
    - Run user-defined scripts
    - Update world matrix (update heirarchy in transform)
    - sync shader defines (updates definitions if environment changes)
    - sync gpu buffers (send environment and lighting data to gpu)
  - Reset input state

## Render
- Calls into "user app" Render function. passing engine and window
  - Engine begin_frame -> frame
  - Frame.render    

### Begin Frame (Engine)
- Get the scene and camera
- Call renderer's begin_frame.  passing scene, cam, assets, timing

#### Renderer.begin_frame()
- This builds the frame to be rendered. GPU Resource preparation
1. [Extract and prepare](#extract-and-prepare) - All scene resources (mesh, geometry, material, etc.)
2. [Generate Commands](#generate-commands) (Renderable items)
3. Feature preparation (Extract and prepare on passes)
  a. Create pipelines, bindgroups (pass specific bg)
4. Return the Frame


### Frame.render()
 - The Beast, creating the entire render graph.  All resources exist by this point

## End Frame
- Call into Renderer end frame


#### Extract and Prepare
- Extract Scene
  - Extracts everything into GPU resources, lights/environment/mesh(renderable item)
  - Lights - get lighting info to update storage buffer
  - Environment - get GPU environment resources
  - RenderItem
    - Collect Meshes
      - loop through nodes to find meshes that can be rendered
      - get node matrix data/aabb
    - Prepare Resources
      - Get mesh, then prepare_mesh
        - Prepare Geometry
          - Get or Create GpuGeometry
          - Prepare vertex/index attributes (create buffers for each)
        - Prepare Material
          - Get or Create GpuMaterial
          - allocate resources
            - create/update material uniform
            - visit textures, create gpu texture resources (TextureBinding: image/sampler | generate mipmaps)
          - rebuild bg if needed

        - Get/Create mesh bind group (2)
          - use Resource builder to create uniform buffer
          - add defines to builder for mesh/geometry (skinning)
          - Generate wgsl and wgpu bg layouts
          - Prepare binding resources
            - For each resource in builder create or update resource (Buffer/Texture)
          - Create bind group (using builder)
          - Save layout/bg/wgsl into binding context and cache it
      - add to list of renderables
- Prepare Environment GPU resources
- Update frame state uniform for latest camera
- Prepare frame globals (camera uniform, light storage buffer, environment maps)
  - Create/Update frame globals bind groups

#### Generate Commands
- Get pipeline settings, frame globals/bg
- For Each renderable item, generate a command
  - get gpu_geo, gpu_mat, mesh bg
  - get/create pipeline
  - Command: mesh bg, geo handle, mat handle, pipeline id