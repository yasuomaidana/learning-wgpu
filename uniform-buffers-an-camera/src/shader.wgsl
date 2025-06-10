struct CameraUniform{
    // The camera's view matrix, which transforms world coordinates to view coordinates.
    view_projection: mat4x4<f32>,
}


// @group(1) means this resource (the CameraUniform uniform buffer) is in bind group 1.
//@binding(0) means it is the first binding in that group.
@group(1) @binding(0)
var<uniform> camera: CameraUniform;


struct VertexInput {
    //This means that when vertex data is passed to the shader, 
    // the value at location 0 in the vertex buffer will be mapped 
    // to the position variable. It is used to match the layout of 
    // vertex attributes between the GPU buffer and the shader.
    @location(0) position: vec3<f32>,
//    @location(1) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    // The @builtin(position) attribute in WGSL marks the clip_position 
    // field as the output position for the vertex shader. 
    // This value determines where the vertex will appear on the 
    // screen after transformation.
    //
    // The GPU uses this field to perform the final transformation
    // and rasterization of the vertex. It must be a vec4<f32>, 
    // where the x, y, z, and w components represent the position 
    // in clip space.
    @builtin(position) clip_position: vec4<f32>,
    //    @location(0) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
//    out.color = model.color;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_projection * vec4<f32>(model.position, 1.0);
    return out;
}

// @group(0) and @binding(0) specify the bind group and binding index for this resource.
// @group(n) assigns the resource to a specific bind group, allowing the shader to access multiple groups of resources.
// @binding(n) assigns the resource to a specific binding within the group, matching the layout defined in the application code.
//
// t_diffuse is a 2D texture resource bound to group 0, binding 0.
// It is used in the fragment shader for sampling the diffuse texture color.
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

// In this WGSL shader context, `@binding(n)` 
// specifies the binding index within a bind group for a resource 
// (like a texture or sampler). It must match the binding index 
// defined in your Rust code when creating the `BindGroupLayout` 
// and `BindGroup`. For example, `@binding(0)` for `t_diffuse` 
// means it corresponds to the resource at binding 0 in the 
// bind group, and `@binding(1)` for `s_diffuse` means 
// it corresponds to binding 1. This ensures the shader 
// accesses the correct GPU resources as set up 
// in your application code.
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
