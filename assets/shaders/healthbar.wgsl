#import bevy_pbr::forward_io::VertexOutput;

struct HealthbarMaterial {
    filled_color: vec4<f32>,
    empty_color: vec4<f32>,
    fraction: f32,
}

@group(1) @binding(0) var<uniform> material: HealthbarMaterial;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return select(material.empty_color, material.filled_color, mesh.uv[0] <= material.fraction);
}