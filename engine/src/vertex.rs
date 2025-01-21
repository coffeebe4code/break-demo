#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
pub fn vertex(pos: [f32; 3], tc: [i8; 2]) -> Vertex {
    Vertex {
        position: [pos[0] as f32, pos[1] as f32, pos[2] as f32],
        tex_coords: [tc[0] as f32, tc[1] as f32],
    }
}
