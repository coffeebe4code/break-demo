@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4(position, 0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = position.xy / vec2<f32>(1480.0, 1080.0);
    let color = 0.5 + 0.5 * sin(vec3(uv.x * 1.78, uv.y * 1.78, (uv.x + uv.y) * 3.14));
    return vec4(color, 1.0);
}
