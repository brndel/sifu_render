mod mesh;
mod vertex;
mod mesh_instance;

pub use mesh::Mesh;

pub use sifu_render_derive::Vertex;
pub use vertex::Vertex;
pub use sifu_render_derive::MeshInstance;
pub use mesh_instance::MeshInstance;

pub mod wgpu {
    pub use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
}
