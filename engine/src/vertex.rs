#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2D {
    position: [f32; 2],
}

pub fn vertex2d(pos: [f32; 2]) -> Vertex2D {
    Vertex2D {
        position: [pos[0] as f32, pos[1] as f32],
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2DTexture {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub fn vertex2dtexture(pos: [f32; 2], tc: [i8; 2]) -> Vertex2DTexture {
    Vertex2DTexture {
        position: [pos[0] as f32, pos[1] as f32],
        tex_coords: [tc[0] as f32, tc[1] as f32],
    }
}
