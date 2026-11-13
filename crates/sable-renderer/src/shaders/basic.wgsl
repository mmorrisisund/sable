struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
    @location(2) normal: vec3f,
    @location(3) tex_coords: vec2f,
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
    @location(1) normal: vec3f,
    @location(2) tex_coords: vec2f,
}

struct CameraUniform {
    view: mat4x4f,
    projection: mat4x4f,
    view_projection: mat4x4f,
    position: vec4f,
}

@group(0) @binding(0) var<uniform> u_camera: CameraUniform;


struct Material {
    base_color: vec4f,
    metallic: f32,
    roughness: f32,
}

@group(1) @binding(0) var<uniform> u_material: Material;
@group(1) @binding(1) var albedo_t: texture_2d<f32>;
@group(1) @binding(2) var albedo_s: sampler;

@vertex
fn vs_main(in: VertexInput) ->  VertexOutput {
    var out: VertexOutput;

    out.position = u_camera.view_projection * vec4f(in.position, 1.0);
    out.color = in.color;
    out.normal = in.normal;
    out.tex_coords = in.tex_coords;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    var albedo = textureSample(albedo_t, albedo_s, in.tex_coords).rgb;
    albedo *= u_material.base_color.rgb;

    return vec4f(albedo, 1.0);
}