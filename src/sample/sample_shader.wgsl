

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    // @location(1) tex_coords: vec2<f32>,
    // @location(2) instance_color: vec4<f32>,
    // @location(3) normal: vec3<f32>,
};



@vertex
fn vs_main(
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
    out.clip_position = /* camera.view_proj * */ model_matrix * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    // out.tex_coords = vertex.tex_coords;
    return out;
}

// Fragment shader

// struct LightDirection {
//     direction: vec3<f32>
// }

// @group(1) @binding(0) var<uniform> light_direction: LightDirection;
// // @group(1) @binding(1) var s_diffuse: sampler;
// // @group(1) @binding(0) var color: vec3<f32>;

// @group(2) @binding(0) var tex_sampler: sampler;
// @group(2) @binding(1) var texture: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.color, 1.0);
    // let texture_color = textureSample(texture, tex_sampler, in.tex_coords);

    // var light: f32 = 1.0;

    // if (length(light_direction.direction) > 0.0) {
    //     let light_dir = normalize(light_direction.direction);
    //     light = max(dot(in.normal, -light_dir), 0.0);
    // }
    
    // return texture_color * in.instance_color * (light * 0.9 + 0.1) * in.instance_color.a;
}