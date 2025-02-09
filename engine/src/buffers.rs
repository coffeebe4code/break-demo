use crate::context::Context;

pub fn index_buffer(context: &Context, name: &str, indices: &[u16]) -> wgpu::Buffer {
    let idx_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(&format!("index buffer: {}", name)),
        size: (indices.len() * std::mem::size_of::<u16>()) as u64,
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    context
        .queue
        .write_buffer(&idx_buffer, 0, bytemuck::cast_slice(indices));
    return idx_buffer;
}
