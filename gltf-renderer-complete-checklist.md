# glTF Renderer Development Checklist

This checklist uses every model in the Khronos glTF Sample Assets repository as a development milestone for a renderer. It is organized from minimal static geometry through core PBR, then extensions, with animation, morphing, and skinning deliberately placed last.

## How to use this checklist

- Select one canonical variant for each model, preferably a binary `.glb` when available.
- Treat compressed or alternative texture variants as additional tests attached to the same model.
- Do not mark a model complete merely because it loads. Require correct rendering, no validation errors, and an approved screenshot.
- Record unsupported required extensions explicitly rather than silently ignoring them.
- Keep a machine-readable feature manifest beside the checklist.

## Completion criteria for every model

- [ ] Asset parses without structural or bounds errors.
- [ ] Required resources resolve correctly.
- [ ] Required features and extensions are detected.
- [ ] Unsupported required extensions fail clearly.
- [ ] Render completes without graphics API validation errors.
- [ ] Output matches an approved reference or golden image.
- [ ] Repeated load/render/unload does not leak resources.

---

## Phase 0: Loader, GLB, and test harness

Build deterministic loading, validation, diagnostics, screenshot capture, and golden-image comparison before feature work begins.

- [x] **`TriangleWithoutIndices`**
  - **Tests:** Minimal scene/node/mesh traversal, POSITION accessors, sequential non-indexed drawing.
- [x] **`Triangle`**
  - **Tests:** Index accessors and indexed triangle drawing.
- [ ] **`Box`**
  - **Tests:** Normals, a basic material, depth testing, face culling, and reusable mesh upload.

---

## Phase 1: Core geometry, accessors, and topology

Complete static core mesh ingestion while rendering with a simple diagnostic shader.

- [ ] **`Cube`**
  - **Tests:** Hard edges, duplicated vertices, flat face normals, and winding.
- [ ] **`BoxInterleaved`**
  - **Tests:** Interleaved attributes, byteStride, accessor offsets, and alignment.
- [ ] **`BoxVertexColors`**
  - **Tests:** COLOR_0 routing and normalized or floating-point vertex colors.
- [ ] **`VertexColorTest`**
  - **Tests:** Vertex color component formats and multiplication with base color.
- [ ] **`SimpleSparseAccessor`**
  - **Tests:** Sparse accessor indices, replacement values, and missing base buffers.
- [ ] **`MeshPrimitiveModes`**
  - **Tests:** All core point, line, strip, fan, and triangle primitive modes.
- [ ] **`PrimitiveModeNormalsTest`**
  - **Tests:** Primitive modes with and without normals and defined fallback shading.
- [ ] **`SimpleMeshes`**
  - **Tests:** Multiple nodes sharing one mesh and multiple vertex attributes.
- [x] **`SimpleMaterial`**
  - **Tests:** Core untextured material factors and primitive-to-material binding.
- [ ] **`NegativeScaleTest`**
  - **Tests:** Mirrored transforms, normal matrices, face winding, and culling.

---

## Phase 2: Scene graphs, transforms, cameras, and resource paths

Harden scene traversal and external-resource handling before introducing texture-heavy rendering.

- [ ] **`MultipleScenes`**
  - **Tests:** Default scene selection and explicit switching between multiple scenes.
- [ ] **`Cameras`**
  - **Tests:** Perspective and orthographic cameras attached to transformed nodes.
- [ ] **`OrientationTest`**
  - **Tests:** glTF coordinate conventions and TRS transform composition.
- [ ] **`BoomBoxWithAxes`**
  - **Tests:** Axis orientation and a recognizable multi-part reference object.
- [ ] **`Box With Spaces`**
  - **Tests:** Percent-encoded resource paths and filenames containing spaces.
- [ ] **`Unicode❤♻Test`**
  - **Tests:** Unicode filenames, object names, material names, and filesystem handling.
- [ ] **`AntiqueCamera`**
  - **Tests:** Deep static hierarchy, many primitives, materials, and local transforms.
- [ ] **`NodePerformanceTest`**
  - **Tests:** High node, mesh, primitive, and texture counts; traversal and allocation performance.
- [ ] **`VirtualCity`**
  - **Tests:** Large static scene traversal, culling, batching, and memory pressure.

---

## Phase 3: Textures, UVs, samplers, tangents, and alpha

Implement the complete core image and sampling path before physically based lighting.

- [ ] **`SimpleTexture`**
  - **Tests:** Image resolution, upload, TEXCOORD_0, and base-color sampling.
- [ ] **`BoxTextured`**
  - **Tests:** Power-of-two base-color texture and ordinary sampler behavior.
- [ ] **`BoxTexturedNonPowerOfTwo`**
  - **Tests:** Non-power-of-two dimensions, mipmaps, filters, and wrap modes.
- [ ] **`TextureCoordinateTest`**
  - **Tests:** UV orientation and correspondence between object and texture coordinates.
- [ ] **`MultiUVTest`**
  - **Tests:** TEXCOORD_0/TEXCOORD_1 and per-texture coordinate-set selection.
- [ ] **`TextureSettingsTest`**
  - **Tests:** Min/mag filters, mip filtering, wrapping, and double-sided materials.
- [ ] **`TextureEncodingTest`**
  - **Tests:** sRGB decoding for color textures and linear treatment of data textures.
- [ ] **`TextureLinearInterpolationTest`**
  - **Tests:** Interpolation in linear space after transfer-function decoding.
- [ ] **`NormalTangentTest`**
  - **Tests:** Normal mapping and generated tangent frames when TANGENT is absent.
- [ ] **`NormalTangentMirrorTest`**
  - **Tests:** Supplied tangents, handedness, and mirrored UV islands.
- [ ] **`TwoSidedPlane`**
  - **Tests:** doubleSided rendering, back-face handling, and normal orientation policy.
- [ ] **`AlphaBlendModeTest`**
  - **Tests:** OPAQUE, MASK, BLEND, alphaCutoff, depth writes, and sorting.
- [ ] **`CompareAlphaCoverage`**
  - **Tests:** Alpha coverage, masked-edge behavior, and texture alpha interpretation.

---

## Phase 4: Core metallic-roughness PBR diagnostics

Implement and calibrate the core glTF metallic-roughness material model and image-based lighting.

- [ ] **`MetalRoughSpheresNoTextures`**
  - **Tests:** Metallic and roughness factors over a controlled grid without textures.
- [ ] **`MetalRoughSpheres`**
  - **Tests:** Packed metallic-roughness textures and PBR response.
- [ ] **`EnvironmentTest`**
  - **Tests:** IBL for dielectric and metallic materials across roughness values.
- [ ] **`CompareBaseColor`**
  - **Tests:** Base-color factor and texture pathways.
- [ ] **`CompareMetallic`**
  - **Tests:** Metallic factor and metallic texture channel.
- [ ] **`CompareRoughness`**
  - **Tests:** Roughness factor, texture channel, and highlight width.
- [ ] **`CompareNormal`**
  - **Tests:** Normal map scale, tangent basis, and lighting response.
- [ ] **`CompareAmbientOcclusion`**
  - **Tests:** Occlusion texture strength and indirect-light application.
- [ ] **`Duck`**
  - **Tests:** Simple textured production asset and baseline end-to-end PBR.
- [ ] **`Avocado`**
  - **Tests:** Organic textured MR-PBR surface.
- [ ] **`BarramundiFish`**
  - **Tests:** Detailed textured MR-PBR surface and normal mapping.
- [ ] **`BoomBox`**
  - **Tests:** Base color, MR, normals, occlusion, and emissive in one asset.
- [ ] **`DamagedHelmet`**
  - **Tests:** Integrated production PBR reference with all major core texture channels.
- [ ] **`Corset`**
  - **Tests:** Fine normal detail, occlusion, and textured fabric-like core material.
- [ ] **`SciFiHelmet`**
  - **Tests:** Detailed normal-mapped hard surface and multiple texture inputs.
- [ ] **`FlightHelmet`**
  - **Tests:** Many materials, primitives, texture sets, and mixed alpha behavior.
- [ ] **`Lantern`**
  - **Tests:** Complex textured static asset; compression variants are deferred.
- [ ] **`WaterBottle`**
  - **Tests:** Core PBR, transparency-related core states, and multi-material integration.
- [ ] **`Suzanne`**
  - **Tests:** Simple recognizable geometry for lighting and material regression checks.

---

## Phase 5: Large core scenes and production hardening

Use complex core assets to validate caching, culling, batching, lifetime management, and repeatable output.

- [ ] **`Sponza`**
  - **Tests:** Large architectural scene, many materials, alpha surfaces, and lighting coverage.
- [ ] **`USDShaderBallForGltf`**
  - **Tests:** High-quality integrated core material and texture validation.

---

## Phase 6: Structural, lighting, and data extensions

Begin extensions with features that minimally disturb the core PBR transport model.

- [ ] **`UnlitTest`**
  - **Tests:** KHR_materials_unlit with base color, textures, vertex color, and alpha.
- [ ] **`TextureTransformTest`**
  - **Tests:** KHR_texture_transform offset, scale, rotation, and texCoord override.
- [ ] **`TextureTransformMultiTest`**
  - **Tests:** Independent texture transforms across several material texture slots.
- [ ] **`MaterialsVariantsShoe`**
  - **Tests:** KHR_materials_variants enumeration, mappings, and runtime switching.
- [ ] **`CarConcept`**
  - **Tests:** Material variants on a complex multi-material object.
- [ ] **`ChronographWatch`**
  - **Tests:** Material variants, dense geometry, and detailed material switching.
- [ ] **`NodeVisibilityTest`**
  - **Tests:** Static KHR_node_visibility evaluation and traversal exclusion.
- [ ] **`XmpMetadataRoundedCube`**
  - **Tests:** KHR_xmp_json_ld parsing, retention, and inspection without rendering effects.
- [ ] **`DirectionalLight`**
  - **Tests:** Directional-light calculations and transformed direction.
- [ ] **`LightsPunctualLamp`**
  - **Tests:** KHR_lights_punctual point and spot lights attached to nodes.
- [ ] **`PointLightIntensityTest`**
  - **Tests:** Punctual-light intensity, color, range, and inverse-square attenuation.
- [ ] **`PlaysetLightTest`**
  - **Tests:** Multiple punctual lights in a complex textured scene.
- [ ] **`SimpleInstancing`**
  - **Tests:** EXT_mesh_gpu_instancing and per-instance translation, rotation, and scale.
- [ ] **`MeshoptCubeTest`**
  - **Tests:** Meshopt-compressed buffer views and reconstruction before accessor decoding.

---

## Phase 7: Material extensions, first tier

Add composable material lobes before framebuffer-dependent transmission features.

- [ ] **`EmissiveStrengthTest`**
  - **Tests:** KHR_materials_emissive_strength and HDR emissive output.
- [ ] **`CompareEmissiveStrength`**
  - **Tests:** Core emissive versus emissive-strength output.
- [ ] **`IORTestGrid`**
  - **Tests:** KHR_materials_ior and dielectric Fresnel response.
- [ ] **`CompareIor`**
  - **Tests:** Default IOR versus extension-controlled IOR.
- [ ] **`SpecularTest`**
  - **Tests:** KHR_materials_specular factor, color, and textures.
- [ ] **`CompareSpecular`**
  - **Tests:** Core dielectric specular versus extension-adjusted response.
- [ ] **`ClearCoatTest`**
  - **Tests:** KHR_materials_clearcoat factor, roughness, and clearcoat normal.
- [ ] **`ClearCoatCarPaint`**
  - **Tests:** Layered clearcoat over a metallic car-paint base.
- [ ] **`ClearcoatWicker`**
  - **Tests:** Clearcoat normal and roughness over a textured substrate.
- [ ] **`CompareClearcoat`**
  - **Tests:** Base material versus clearcoat extension.
- [ ] **`PotOfCoals`**
  - **Tests:** Clearcoat combined with emissive coals and multiple materials.
- [ ] **`SheenCloth`**
  - **Tests:** KHR_materials_sheen on a focused fabric example.
- [ ] **`SheenTestGrid`**
  - **Tests:** Sheen color and roughness across controlled values.
- [ ] **`CompareSheen`**
  - **Tests:** Core material versus sheen-enhanced material.
- [ ] **`SheenChair`**
  - **Tests:** Sheen combined with material variants.
- [ ] **`ChairDamaskPurplegold`**
  - **Tests:** Textured sheen on a detailed fabric asset.
- [ ] **`SheenWoodLeatherSofa`**
  - **Tests:** Sheen and specular across mixed wood and leather materials.
- [ ] **`SpecularSilkPouf`**
  - **Tests:** Combined sheen and specular material extensions.
- [ ] **`GlamVelvetSofa`**
  - **Tests:** Material variants combined with sheen and specular.

---

## Phase 8: Material extensions, advanced reflection

Add direction-dependent and thin-film reflection effects, then validate combinations.

- [ ] **`AnisotropyStrengthTest`**
  - **Tests:** KHR_materials_anisotropy strength.
- [ ] **`AnisotropyRotationTest`**
  - **Tests:** Anisotropy rotation in tangent space.
- [ ] **`AnisotropyDiscTest`**
  - **Tests:** Anisotropy texture and directional highlight response.
- [ ] **`CarbonFibre`**
  - **Tests:** Textured anisotropy and normal mapping on a production material.
- [ ] **`CompareAnisotropy`**
  - **Tests:** Isotropic versus anisotropic reflection.
- [ ] **`IridescenceDielectricSpheres`**
  - **Tests:** KHR_materials_iridescence on dielectric materials.
- [ ] **`IridescenceMetallicSpheres`**
  - **Tests:** Iridescence interacting with metallic materials.
- [ ] **`IridescenceSuzanne`**
  - **Tests:** Iridescence over recognizable curved geometry.
- [ ] **`IridescenceAbalone`**
  - **Tests:** Textured iridescence and film-thickness variation.
- [ ] **`CompareIridescence`**
  - **Tests:** Core material versus thin-film iridescence.
- [ ] **`SpecGlossVsMetalRough`**
  - **Tests:** Legacy KHR_materials_pbrSpecularGlossiness compatibility path.
- [ ] **`TrafficCone`**
  - **Tests:** KHR_materials_retroreflection and light-to-view dependent response.

---

## Phase 9: Transmission, volume, dispersion, and combined optical extensions

Introduce render-graph features requiring scene-color capture, ordering, refraction, and absorption.

- [ ] **`TransmissionTest`**
  - **Tests:** KHR_materials_transmission and scene-color sampling.
- [ ] **`CompareTransmission`**
  - **Tests:** Opaque versus transmissive material behavior.
- [ ] **`TransmissionRoughnessTest`**
  - **Tests:** Rough transmission and interaction with IOR.
- [ ] **`TransmissionThinwallTestGrid`**
  - **Tests:** Thin-wall transmission across a controlled material grid.
- [ ] **`GlassBrokenWindow`**
  - **Tests:** Transmission combined with alpha MASK holes.
- [ ] **`AttenuationTest`**
  - **Tests:** KHR_materials_volume thickness, attenuation, and node scale.
- [ ] **`CompareVolume`**
  - **Tests:** Thin transmission versus volumetric absorption.
- [ ] **`GlassHurricaneCandleHolder`**
  - **Tests:** Transmission and volume on production glass.
- [ ] **`GlassVaseFlowers`**
  - **Tests:** Core alpha blending compared with transmission and volume.
- [ ] **`MosquitoInAmber`**
  - **Tests:** Transmission, volume, IOR, and enclosed geometry.
- [ ] **`ABeautifulGame`**
  - **Tests:** Complex scene using transmission and volume.
- [ ] **`DragonAttenuation`**
  - **Tests:** Variants, transmission, volume, and attenuation.
- [ ] **`TransmissionOrderTest`**
  - **Tests:** Ordering among opaque, masked, blended, and transmissive surfaces.
- [ ] **`StainedGlassLamp`**
  - **Tests:** Colored transparent/transmissive materials and complex overlap.
- [ ] **`DispersionTest`**
  - **Tests:** KHR_materials_dispersion baseline.
- [ ] **`CompareDispersion`**
  - **Tests:** Transmission with and without dispersion.
- [ ] **`DragonDispersion`**
  - **Tests:** Dispersion combined with transmission and volume on complex geometry.
- [ ] **`DiffuseTransmissionTest`**
  - **Tests:** KHR_materials_diffuse_transmission baseline.
- [ ] **`DiffuseTransmissionTeacup`**
  - **Tests:** Textured diffuse transmission on a thin production object.
- [ ] **`MandarinOrange`**
  - **Tests:** Diffuse transmitted light on an organic object.
- [ ] **`ScatteringSkull`**
  - **Tests:** Subsurface-scattering-oriented extension behavior and complex geometry.
- [ ] **`IridescenceLamp`**
  - **Tests:** Iridescence combined with transmission and volume.
- [ ] **`IridescentDishWithOlives`**
  - **Tests:** Transmission, volume, IOR, specular, and iridescent materials.
- [ ] **`SunglassesKhronos`**
  - **Tests:** Iridescent transmissive lenses and multiple materials.
- [ ] **`ToyCar`**
  - **Tests:** Transmission, clearcoat, and sheen in one composite asset.
- [ ] **`AnisotropyBarnLamp`**
  - **Tests:** Anisotropy, clearcoat, emissive strength, transmission, volume, and BasisU.

---

## Phase 10: Core node and morph animation

Implement timeline evaluation only after static core and extension rendering are stable.

- [ ] **`AnimatedTriangle`**
  - **Tests:** Minimal node rotation animation.
- [ ] **`AnimatedCube`**
  - **Tests:** Linear quaternion rotation animation.
- [ ] **`BoxAnimated`**
  - **Tests:** Translation and rotation channels on a static mesh.
- [ ] **`InterpolationTest`**
  - **Tests:** STEP, LINEAR, and CUBICSPLINE interpolation.
- [ ] **`SimpleMorph`**
  - **Tests:** Morph target attributes, weights, and basic morph animation.
- [ ] **`AnimatedMorphCube`**
  - **Tests:** Multiple morph targets driven by animated weights.
- [ ] **`MorphPrimitivesTest`**
  - **Tests:** Consistent morph weights across multiple primitives.
- [ ] **`MorphStressTest`**
  - **Tests:** Evaluation of up to eight morph targets.

---

## Phase 11: Extension property animation

Extend the animation system to typed non-TRS properties only after ordinary clips are correct.

- [ ] **`AnimatedColorsCube`**
  - **Tests:** KHR_animation_pointer targeting material base color.
- [ ] **`AnimationPointerUVs`**
  - **Tests:** Animation pointer targeting texture-transform properties.
- [ ] **`PotOfCoalsAnimationPointer`**
  - **Tests:** Animated material or texture properties for heat-refraction effects.
- [ ] **`CubeVisibility`**
  - **Tests:** KHR_node_visibility animated through KHR_animation_pointer.
- [ ] **`LightVisibility`**
  - **Tests:** Animated visibility on a node carrying a punctual light.

---

## Phase 12: Skinning and full animated scenes

Finish with joint palettes, inverse-bind matrices, weighted deformation, complex clips, and animation combined with advanced rendering.

- [ ] **`SimpleSkin`**
  - **Tests:** Minimal skin object, JOINTS_0, WEIGHTS_0, and inverse-bind matrices.
- [ ] **`RiggedSimple`**
  - **Tests:** Basic skeletal animation and joint matrix evaluation.
- [ ] **`RiggedFigure`**
  - **Tests:** Larger hierarchy and blended joint weights.
- [ ] **`RecursiveSkeletons`**
  - **Tests:** Reused meshes, unusual skeleton roots, and recursive hierarchy cases.
- [ ] **`BrainStem`**
  - **Tests:** Production-style animated skin and complex joint hierarchy.
- [ ] **`CesiumMan`**
  - **Tests:** Textured character skinning and animation.
- [ ] **`Fox`**
  - **Tests:** Multiple named skeletal animation clips and clip switching.
- [ ] **`CesiumMilkTruck`**
  - **Tests:** Multi-node textured scene with transform animation.
- [ ] **`DiffuseTransmissionPlant`**
  - **Tests:** Animation combined with diffuse-transmission materials.
- [ ] **`CommercialRefrigerator`**
  - **Tests:** Final integration of door animation, glass, complex materials, and scene hierarchy.

---

## Repository-wide final acceptance

- [ ] Run loader-only validation across every model and selected variant.
- [ ] Run deterministic screenshot tests across every supported model.
- [ ] Run debug views for normals, tangents, UV sets, mip levels, metallic, roughness, and joint weights where applicable.
- [ ] Run repeated load/render/unload stress tests.
- [ ] Report load time, upload time, frame time, draw calls, triangles, and texture memory.
- [ ] Verify that every required extension is implemented or produces an intentional unsupported result.

## Coverage summary

- **Models covered:** 150
- **Ordering:** static core → core PBR → extensions → animation/morphing → skinning
- **Source inventory:** Khronos glTF Sample Assets `Models` directory and generated model lists.

## Suggested per-model result record

```yaml
asset: Triangle
variant: glTF-Binary/Triangle.glb
phase: 0
status: pending
features: []
extensionsUsed: []
extensionsRequired: []
goldenImage: null
metrics:
  loadTimeMs: null
  uploadTimeMs: null
  frameTimeMs: null
  drawCalls: null
  triangleCount: null
  textureMemoryBytes: null
notes: ''
```
