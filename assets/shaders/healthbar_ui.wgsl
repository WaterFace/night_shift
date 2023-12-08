#import bevy_ui::ui_vertex_output::UiVertexOutput;

struct HealthbarMaterial {
    filled_color: vec4<f32>,
    empty_color: vec4<f32>,
    fraction: f32,
}

@group(1) @binding(0) var<uniform> material: HealthbarMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    return select(material.empty_color, material.filled_color, in.uv[0] <= material.fraction);
}