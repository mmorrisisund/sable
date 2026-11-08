Otter Load Texture
* Determine if source is uri or buffer view
* Load image data and decode
* Create/Save IntermediateTexture
  * Image Data (width, height, bytes)
  * Name, sampler, mip gen bool

Otter Load Material
* Save metal/rough/emissive to uniform
* Add textures in turn to maps: Color, MR, normal, etc.
  * Create cpu Texture and add to asset server (get_or_create_texture)
* Save settings values
* Add to asset server and save to created_materials(loader tracking)

Otter Texture Creation
* Use IntermediateTexture to create image (if needed) and add to assets
* create engine texture (name/image(handle)/sampler/color space
* Add to assets and created_textures (loader tracking map)

Final Material (After procmacro)
* Uniforms: CpuBuffer around a typed uniform
  * Type Uniform from "uniform" fields in user struct
* Settings: Locked MaterialSettings
* Textures: Locked texture maps