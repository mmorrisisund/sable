# 🛠️ glTF Feature Progression Checklist (Part 1: Core Engine)

A checkable development roadmap isolating core glTF 2.0 specifications from the official Khronos `Models-testing.md` registry.

---

## 🟩 Phase 1: Core Geometry, Accessors, & Scene Layout

Isolate raw JSON parsing, binary buffer reading, and primitive assembly. Render everything using a basic flat or unlit pass-through shader without lighting pipelines.

- [x] **Triangle Without Indices**
  - _Tests:_ Raw sequential draw pipeline. Validates structural mapping and binary buffer extraction without an index map or material.
- [ ] **Triangle**
  - _Tests:_ Basic indexed rendering loop. Verifies target element array buffers.
- [ ] **Simple Material**
  - _Tests:_ Core untextured material color factors. Verifies that constant properties like baseline color coefficients apply cleanly before sampling pipelines exist.
- [ ] **Simple Meshes**
  - _Tests:_ Multiple nodes instancing the exact same mesh with a shared indexed primitive without a material.
- [ ] **Multiple Scenes**
  - _Tests:_ Target scene resolution logic. Forces the engine to check and load the designated root JSON `scene` pointer array index rather than blindly assuming scene `0`.
- [ ] **Box**
  - _Tests:_ Multi-attribute alignment. Ensures interleaved or independent position arrays align with explicit `POSITION` and `NORMAL` coordinate streams.
- [ ] **Box with interleaved position and normal attributes**
  - _Tests:_ Directly stresses parsing strides when attributes share a single buffer view.
- [ ] **Box With Spaces**
  - _Tests:_ URI decoding. Ensures your engine correctly parses asset filenames utilizing encoded spaces (`%20`).
- [ ] **Cube**
  - _Tests:_ Flat shading equations. Validates un-smoothed face boundaries where vertices duplicate explicitly across hard edges.
- [ ] **Box Vertex Colors**
  - _Tests:_ Secondary vertex data routing. Stream evaluation for floating-point or normalized byte vectors inside the `COLOR_0` attribute slot.
- [ ] **Simple Sparse Accessor**
  - _Tests:_ Data array overriding. Requires your parser to identify and dynamically overwrite base buffer blocks using `sparse` coordinate key-value pairings.
- [ ] **Unicode❤♻Test**
  - _Tests:_ String decoding. Validates filesystem robustness against folders, materials, and nodes named with multi-byte international characters.
- [ ] **Mesh Primitive Modes**
  - _Tests:_ Primitive topologies. Verifies rendering support for all valid glTF geometric types (Points, Lines, Line Loops, Triangles, etc.).
- [ ] **Primitive Mode Normals Test**
  - _Tests:_ Normal attribute fallback states. Compares the behavior of rendering points, lines, and triangles with and without explicit normal data.
- [ ] **Negative Scale Test**
  - _Tests:_ Backface culling updates. Validates vertex winding order and culling alterations when nodes have negative scaling multipliers.
- [ ] **NodePerformanceTest**
  - _Tests:_ Graph scaling limits. Stress-tests your engine graph parsing high counts of nodes, primitives, and memory descriptors.

## 🟨 Phase 2: Core Hierarchies & Cameras

Establish proper viewing projections and deep scene graph node tree inheritance using static primitives.

- [ ] **Cameras**
  - _Tests:_ Projection pipelines. Tests the instantiation and toggle logic between perspective and orthographic camera node definitions.
- [ ] **Orientation Test**
  - _Tests:_ Spatial coordinate spaces. Enforces strict compliance with glTF's right-handed Y-up spatial matrix layout.
- [ ] **Buggy**
  - _Tests:_ Deep node matrix concatenation. Combines long chains of nested local Translation, Rotation, and Scale (TRS) properties into correct world space coordinates.
- [ ] **Cesium Milk Truck**
  - _Tests:_ Complex assembly reuse. Assures that multiple sub-mesh descriptors instantiate and position simultaneously across deep hierarchies.
- [ ] **Antique Camera**
  - _Tests:_ Asset complexity benchmarking. High-complexity static asset validating deep node mapping and material slot assignments on structural layouts.
- [ ] **Virtual City**
  - _Tests:_ Scene graph stress testing. Handles massive, highly cluttered static node counts concurrently.

## 🟧 Phase 3: Core Texture Management & Sampler Properties

Configure texture map indexing, coordinate space rules, and GPU sampling parameters.

- [ ] **Simple Texture**
  - _Tests:_ Base image loading. Pulls image source paths and maps texture files directly onto active material shaders.
- [ ] **Texture Coordinate Test**
  - _Tests:_ Texture mapping space. Validates UV orientations to confirm that texture files sample accurately without being vertically inverted along the V-axis.
- [ ] **Texture Settings Test**
  - _Tests:_ Target sampler configurations. Exercises UV address wrapping rules (`CLAMP_TO_EDGE`, `REPEAT`) alongside minification/magnification filters and double-sided material flags.
- [ ] **Texture Encoding Test**
  - _Tests:_ Color space math. Ensures the engine samples colors using accurate transfer functions (sRGB decoding transformations).
- [ ] **Texture Linear Interpolation Test**
  - _Tests:_ Texel filtering execution order. Validates that linear filtering computations execute _after_ sRGB color decoding loops.
- [ ] **MultiUV Test**
  - _Tests:_ Multiple concurrent coordinate streams. Validates shaders mapping independent texture layers via both `TEXCOORD_0` and `TEXCOORD_1`.
- [ ] **Normal-Tangent Test**
  - _Tests:_ Handedness derivation equations. Checks the engine's capability to automatically calculate and generate vertex tangent data vectors for normal maps.
- [ ] **Normal-Tangent Mirror Test**
  - _Tests:_ Supplied tangent vector tracking. Tests an engine's ability to precisely read supplied custom tangent vectors directly from normal maps.

## 🟥 Phase 4: Foundational Core PBR Calibration & Diagnostics

Implement real-time physically based shading equations using core glTF 2.0 metallic-roughness pathways.

- [ ] **Metal-Rough Spheres**
  - _Tests:_ Core PBR execution. Isolates microfacet reflections across a programmatic layout grid of uniform roughness and metallic factor values.
- [ ] **Metal-Rough Spheres (textureless)**
  - _Tests:_ Core PBR factors validation. Verifies the microfacet math pipeline handling factor calculations over standard texturing rules.
- [ ] **Environment Test**
  - _Tests:_ Image-Based Lighting (IBL). Maps a dielectric and metal material matrix used to fine-tune specular reflections against environment lighting setups.
- [ ] **Corset**
  - _Tests:_ Detailed material baking resolution. Combines fine-weave normal maps with packed Ambient Occlusion data to test occlusion shadowing inside tight fabric wrinkles.
- [ ] **Damaged Helmet**
  - _Tests:_ Single-object master PBR shader loop. Integrates Base Color, Metallic-Roughness, Normal maps, Emissive highlights, and Occlusion into one master fragment routine.
- [ ] **FlightHelmet**
  - _Tests:_ Multi-material pipeline queue handling. Forces the engine to draw dozens of independent opaque, alpha-masked, and blended surfaces simultaneously without state breaking.
- [ ] **Avocado**
  - _Tests:_ Baseline PBR asset validation using standard core channels.
- [ ] **Barramundi Fish**
  - _Tests:_ Baseline PBR asset validation using standard core channels.
- [ ] **Duck**
  - _Tests:_ Baseline PBR asset validation using standard core channels.
- [ ] **Lantern**
  - _Tests:_ Baseline PBR asset validation using standard core channels.
- [ ] **SciFi Helmet**
  - _Tests:_ Baseline PBR asset validation using standard core channels.
- [ ] **Water Bottle**
  - _Tests:_ Baseline PBR asset validation using standard core channels.

## 🗂️ Phase 5: Core Diagnostic "Compare" Matrix

Utilize core-only reference charts to fine-tune shader calculations against a zeroed baseline.

- [ ] **Compare Normal**
  - _Tests:_ Normal map illumination accuracy.
- [ ] **Compare Metallic**
  - _Tests:_ Clear-cut isolation of metallic surfaces versus dielectric layers.
- [ ] **Compare Roughness**
  - _Tests:_ Accuracy of microfacet specular dispersion cones.
- [ ] **Compare Base Color**
  - _Tests:_ Isolates pure unlit diffuse surface values from ambient properties.
- [ ] **Compare Ambient Occlusion**
  - _Tests:_ Gauges rendering differences between raw shaders vs. maps running occlusion shadowing.

## 🟦 Phase 6: Core Alpha Blending & Materials Pipelines

Manage draw-order pipelines for non-opaque surfaces and core self-illumination.

- [ ] **Alpha Blend Mode Test**
  - _Tests:_ Opacity state branches. Separates geometry objects directly down three explicit runtime paths: `OPAQUE`, alpha-cutoff `MASK`, and back-to-front sorted `BLEND` render passes.
- [ ] **Boom Box**
  - _Tests:_ Self-illumination. Verifies that emissive maps correctly drive self-illumination independent of ambient light shading.
- [ ] **Boom Box with Axes**
  - _Tests:_ Emissive color textures glowing independently alongside basic spatial orientation axes.

# 🛠️ glTF Feature Progression Checklist (Part 2: Advanced Capabilities)

The remainder of the roadmap focusing on official Khronos material/data extensions, dynamic vertex morph transformations, and hierarchical skeletal deformation loops.

---

## 🚀 Phase 7: Advanced glTF Extensions Matrix

Implement specialized optical properties, lighting parameters, and configuration masks provided by official Khronos extensions.

### 💡 Punctual Lighting Extensions

- [ ] **Directional Light**
  - _Tests:_ Core directional analytical lighting math equations.
- [ ] **Lamp with Punctual Lights**
  - _Tests:_ Parses and injects punctual point and spot lighting sources (`KHR_lights_punctual`).
- [ ] **Point Light Intensity Test**
  - _Tests:_ Specifically isolates point light output intensity math against custom lamp colors (`KHR_lights_punctual`).

### 🗺️ Texture & Data Optimization Extensions

- [ ] **Texture Transform Test**
  - _Tests:_ Parses and implements UV coordinate transformations (offset, scale, rotation) on the BaseColor map (`KHR_texture_transform`).
- [ ] **Texture Transform Multi Test**
  - _Tests:_ Expands texture transformations concurrently across multiple channel streams (`KHR_texture_transform`).
- [ ] **Xmp Metadata Rounded Cube**
  - _Tests:_ Parses non-visual document structure data via the `KHR_xmp_json_ld` specification block.
- [ ] **Meshopt Cube Test**
  - _Tests:_ Evaluates binary stream decompression handling when working with data packed with `KHR_meshopt_compression`.

### 🛡️ Swapping & Material Variations Extensions

- [ ] **MaterialsVariantsShoe**
  - _Tests:_ Dynamic material swapping. Exercises the engine's capability to dynamically substitute texture selections and color coefficients on identical primitives at runtime (`KHR_materials_variants`).

### 🎨 Specialized Shading Extensions

- [ ] **Unlit Test**
  - _Tests:_ Implements unlit constant surface shader pathways (`KHR_materials_unlit`).
- [ ] **Emissive Strength Test**
  - _Tests:_ Adds exposure control tracking for high-intensity glow channels (`KHR_materials_emissive_strength`).
- [ ] **Specular Test**
  - _Tests:_ Fine-tunes specialized specular color and lighting parameters (`KHR_materials_specular`).
- [ ] **Compare Specular**
  - _Tests:_ Standard validation grid checking dielectric and specular highlight multipliers (`KHR_materials_specular`).
- [ ] **Clear Coat Car Paint**
  - _Tests:_ Dual-layer specular calculations rendering high-gloss coats over metals (`KHR_materials_clearcoat`).
- [ ] **KHR_materials_clearcoat Test**
  - _Tests:_ Core technical verification loop for clearcoat parameters (`KHR_materials_clearcoat`).
- [ ] **Clearcoat Wicker**
  - _Tests:_ Checks clearcoat layers passing cleanly over complex, highly-wrinkled surface topologies (`KHR_materials_clearcoat`).
- [ ] **Anisotropy Disc Test**
  - _Tests:_ Evaluates directional microfacet reflections to render brushed metals or satin fabrics (`KHR_materials_anisotropy`).
- [ ] **Anisotropy Rotation Test**
  - _Tests:_ Evaluates directional microfacet reflections to render brushed metals or satin fabrics (`KHR_materials_anisotropy`).
- [ ] **Anisotropy Strength Test**
  - _Tests:_ Evaluates directional microfacet reflections to render brushed metals or satin fabrics (`KHR_materials_anisotropy`).
- [ ] **Compare Sheen**
  - _Tests:_ Validates soft backscattering effects on fabrics (`KHR_materials_sheen`).
- [ ] **Compare Iridescence**
  - _Tests:_ Validates thin-film interference equations generating rainbow-like reflective effects (`KHR_materials_iridescence`).
- [ ] **Iridescence Dielectric Spheres**
  - _Tests:_ Validates thin-film interference equations generating rainbow-like reflective effects (`KHR_materials_iridescence`).
- [ ] **Iridescence Metallic Spheres**
  - _Tests:_ Validates thin-film interference equations generating rainbow-like reflective effects (`KHR_materials_iridescence`).
- [ ] **Iridescence Suzanne**
  - _Tests:_ Validates thin-film interference equations generating rainbow-like reflective effects (`KHR_materials_iridescence`).

### 💧 Transmission, Volume, & Refraction Extensions

- [ ] **Compare Transmission**
  - _Tests:_ True light transmission pathways through thin shells, bypassing standard alpha fades (`KHR_materials_transmission`).
- [ ] **Transmission Test**
  - _Tests:_ True light transmission pathways through thin shells, bypassing standard alpha fades (`KHR_materials_transmission`).
- [ ] **Transmission Roughness Test**
  - _Tests:_ Evaluates how surface roughness blurs background light transmission passing _through_ a mesh (`KHR_materials_transmission`).
- [ ] **Compare Volume**
  - _Tests:_ Thickness and absorption modeling using Beer-Lambert laws (`KHR_materials_volume`).
- [ ] **Attenuation Test**
  - _Tests:_ Thickness and absorption modeling using Beer-Lambert laws (`KHR_materials_volume`).
- [ ] **MandarinOrange**
  - _Tests:_ Evaluates subsurface scattering effects where light transmits through translucent materials (`KHR_materials_diffuse_transmission`).
- [ ] **Skull**
  - _Tests:_ Advanced diagnostic mesh tracking heavy subsurface volume scatter calculations (`KHR_materials_volume` and `KHR_materials_diffuse_transmission`).
- [ ] **Compare Dispersion**
  - _Tests:_ Splits transmissive light paths into distinct rainbow components based on light wavelengths (`KHR_materials_dispersion`).
- [ ] **Dispersion Test**
  - _Tests:_ Splits transmissive light paths into distinct rainbow components based on light wavelengths (`KHR_materials_dispersion`).
- [ ] **Transmission Order Test**
  - _Tests:_ Depth sorting pipelines. Stress-tests your engine's capability to accurately draw `BLEND`, `MASK`, and opaque assets located inside, in front of, or behind transmissive volumes (`KHR_materials_transmission` & `KHR_materials_volume`).

## 🟪 Phase 8: Vertex Morphing & Timeline Node Animations

Introduce time-based alterations to independent nodes and vertex attributes (incorporating core animation pipelines and core-adjacent animation extensions).

- [ ] **Simple Morph**
  - _Tests:_ Core shape-key interpolation. Blends vertex position vectors toward a targeted displacement map using weight vectors.
- [ ] **Morph-Primitives Test**
  - _Tests:_ Target uniform application. Validates that a unified morph target weight applies correctly across separate underlying primitives.
- [ ] **Morph Stress Test**
  - _Tests:_ Pipeline capacity checking. Tests engine limits by running up to 8 independent morph target blend streams simultaneously.
- [ ] **Animated Morph Cube**
  - _Tests:_ Dynamic morph targets. Drives multiple morph target channels over real-time loops using time-stamped keyframe trackers.
- [ ] **Triangle (Animated)**
  - _Tests:_ Base property alterations. Tests trivial runtime rotation modifications targeting node properties directly.
- [ ] **Animated Colors Cube**
  - _Tests:_ Parameter tracking pointers. Updates material base colors dynamically via the uniform animation subsystem (`KHR_animation_pointer`).
- [ ] **AnimationPointerUVs**
  - _Tests:_ Parameter tracking pointers. Animates texture coordinate transforms dynamically via animation pointers (`KHR_animation_pointer`).
- [ ] **NodeVisibilityTest**
  - _Tests:_ Visibility constraints. Controls active graph elements via visibility toggles driven over runtime animation timelines (`KHR_node_visibility`).
- [ ] **CubeVisibility**
  - _Tests:_ Visibility constraints. Controls active graph elements via visibility toggles driven over runtime animation timelines (`KHR_node_visibility`).
- [ ] **LightVisibility**
  - _Tests:_ Visibility constraints. Controls active graph elements via visibility toggles driven over runtime animation timelines (`KHR_node_visibility`).
- [ ] **Box Animated**
  - _Tests:_ Base TRS node transformations. Tests translation and rotation keyframe tracks moving a mesh without an active skeleton or skinning pass.
- [ ] **Interpolation Test**
  - _Tests:_ Timeline math calibration. Compares three timeline curves side-by-side: `STEP`, `LINEAR`, and mathematical `CUBICSPLINE`.

## 🟫 Phase 9: Matrix Skinning & Skeletal Rigging (Final Milestone)

Combine running timeline animation tracks with deep hierarchical parent-child transforms to execute vertex skinning.

- [ ] **Simple Skin**
  - _Tests:_ Base skeletal construction. Sets up a basic node armature framework and transforms vertex arrays via uniform `inverseBindMatrices`.
- [ ] **Rigged Simple**
  - _Tests:_ Joint map resolution. Evaluates multiple bone nodes interacting smoothly across a mesh to verify base `JOINTS_0` lookups.
- [ ] **Rigged Figure**
  - _Tests:_ Weight influence distribution. Validates cross-joint weight blending using the floating-point `WEIGHTS_0` attribute stream.
- [ ] **Recursive Skeletons**
  - _Tests:_ Hierarchy depth. Stress-tests unusual vertex skinning implementations where shared meshes run recursive skeletal hierarchies.
- [ ] **BrainStem**
  - _Tests:_ Production asset stability. Combines complex skin meshes with concurrent structural animation loops.
- [ ] **Cesium Man**
  - _Tests:_ Production asset performance. High-complexity character animation tracking deep bone armatures running on continuous looped timelines.
- [ ] **Fox**
  - _Tests:_ The definitive benchmark. Processes complex hierarchical bone structures, multi-mesh skinned displacements, and concurrent timeline samplers seamlessly.
