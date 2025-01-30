#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TimeUniform {
    time: f32,
}

impl TimeUniform {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }
}
