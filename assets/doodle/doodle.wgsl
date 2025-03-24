#import bevy_pbr::mesh_view_bindings   globals
#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::mesh_bindings       mesh
#import bevy_pbr::mesh_view_bindings  view
#import bevy_pbr::view_transformations::position_world_to_clip

// Define shader parameters as uniforms
// If no padding, the shader will not work in the WASM build, because the struct size must be a multiple of 16 bytes
struct DoodleParams {
    noise_snap: f32,
    noise_scale: f32,
    _padding1: f32,
    _padding2: f32,
}


// define vertex input data
struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// define vertex output data
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// Snap time to create a low framerate effect
fn snap(x: f32, snap_value: f32) -> f32 {
    return snap_value * round(x / snap_value);
}

// Random3 function that returns a vec3 of random values
fn random3(co: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        fract(sin(dot(co.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453),
        fract(sin(dot(co.yz, vec2<f32>(12.9898, 78.233))) * 43758.5453),
        fract(sin(dot(co.zx, vec2<f32>(12.9898, 78.233))) * 43758.5453)
    );
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    
    // Use the uniform values instead of hardcoded values
    let noise_snap = params.noise_snap;
    let noise_scale = params.noise_scale;
    
    let time = snap(globals.time, noise_snap);
    
    // Modified: Generate noise centered around 0
    let time_vec = vec3<f32>(time, 0.0, 0.0);
    let noise_raw = random3(vertex.position + time_vec).xyz;
    
    // Convert noise values from [0,1] range to [-0.5,0.5] range to center around 0
    let centered_noise = noise_raw - 0.5;
    
    // Apply noise scale
    let noise = centered_noise * noise_scale;
    
    // Apply displacement to vertex position
    let displaced_position = vertex.position + noise;
    
    // Normal vertex transformation
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    // Convert the displaced position to world space
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(displaced_position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        vertex.instance_index
    );
    
    // Pass UV coordinates
    out.uv = vertex.uv;
    return out;
}


@group(2) @binding(100) var<uniform> params: DoodleParams;
@group(2) @binding(101) var my_texture: texture_2d<f32>;
@group(2) @binding(102) var my_sample: sampler;

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    in: VertexOutput
) -> @location(0) vec4<f32> {
    // Sample texture with original UVs
    let color: vec4<f32> = textureSample(my_texture, my_sample, in.uv);
    
    // If alpha is very low, discard the fragment
    if (color.a < 0.01) {
        discard;
    }
    
    return color;
}
