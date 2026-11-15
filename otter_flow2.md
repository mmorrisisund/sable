## Window Event (AppRunner Functions)
Update -> Render (User App func [AppHandler impl]) -> End Frame (Engine)
* Calling into Render on User App just forward the egnine render call (Begin Frame)

## Update
App Update -> Engine Update -> Reset Input

## Render (Engine)
Begin Frame (engine) -> (Get Scene/Cam)--> Renderer.Begin Frame (return Frame)

### Render (Renderer)
Gpu Resources -> Commands






RedrawRequest -> Engine.begin_frame() -> (Self)Renderer.begin_frame() -> Extract & Prepare -> Generate Commands
              -> Frame.Render

### Extract & Prepare
Extract Scene -> Prepare Environment -> Prepare Globals

### Extract Scene (Render Items - Meshes)
Create Geometry -> Create Material -> Create Bind Group -> Wrap into renderable







## App Flow
Load Model(-> Prefab) --> Instantiate Prefab (add to scene/get info for ui)