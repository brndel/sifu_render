// struct SampleUniform {
//     opacity: f32,
//     color_a: vec3<f32>,
//     color_b: vec3<f32>,
// }

// struct Camera {
//     view_proj: mat4x4<f32>,
// }

// @group(0) @binding(0) var<uniform> sample: SampleUniform;
// @group(0) @binding(1) var<uniform> camera: Camera;
// @group(0) @binding(2) var texture: texture_2d<f32>;
// @group(0) @binding(3) var tex_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vertex(
    vertex: SampleVertex,
    instance: SampleInstance,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.mat0,
        instance.mat1,
        instance.mat2,
        instance.mat3,
    );

    var out: VertexOutput;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureSample(texture, tex_sampler, in.color.rg);

    return vec4(mix(sample.color_a, sample.color_b, tex.r), sample.opacity);
}