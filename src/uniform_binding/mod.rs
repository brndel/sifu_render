
mod uniform_binding;
mod binding_resource;

pub use uniform_binding::*;
pub use binding_resource::*;


pub mod wgpu {
    pub use wgpu::{ShaderStages, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Device};
}
