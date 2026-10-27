struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
}

struct Material {
    base_color: vec4f,
    metallic: f32,
    roughness: f32,
}

@group(0) @binding(0) var<uniform> u_material: Material;

@vertex
fn vs_main(in: VertexInput) ->  VertexOutput {
    var out: VertexOutput;

    out.position = vec4f(in.position, 1.0);
    out.color = in.color;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {

    // return vec4f(u_material.base_color);
    return vec4f(in.color, 1.0);
}