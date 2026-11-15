Engine to Orchestrate Flow
 - Will hold input state, assets, scene management, and renderer
 - Application will create and own the engine
 - Each frame application will use engine to handle input, update scene, and render to surface
 - gpui will the composite with root view


Camera Stuff
- Scene has generic camera with settings and matrices
- Simplified render camera can be extracted from this
- Camera is just a node (which has a transform)
- Different camera controllers take Input and transform, then modify transform