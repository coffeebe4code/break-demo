#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
pub fn vertex(pos: [f32; 2], tc: [i8; 2]) -> Vertex {
    Vertex {
        position: [pos[0] as f32, pos[1] as f32],
        tex_coords: [tc[0] as f32, tc[1] as f32],
    }
}
