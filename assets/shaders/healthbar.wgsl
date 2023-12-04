#import bevy_pbr::forward_io::VertexOutput;

@group(1) @binding(0) var<uniform> fraction: f32;
@group(1) @binding(1) var<uniform> filled_color: vec4<f32>;
@group(1) @binding(2) var<uniform> empty_color: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return select(empty_color, filled_color, mesh.uv[0] <= fraction);
}