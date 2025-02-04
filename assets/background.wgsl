@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4(position, 0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let color = 0.5 + 0.5 * cos(vec3(position.xy, 1) + vec3(1.0, 1.0, 2.0));
    return vec4(color, 1.0);
}
