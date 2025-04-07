use std::{borrow::Cow, marker::PhantomData};

use wgpu::{CommandEncoder, Device, FragmentState, RenderPass, RenderPassDescriptor, RenderPipelineDescriptor, ShaderModel, ShaderModuleDescriptor, VertexState};

pub struct Renderer<'a> {
    encoder: &'a mut CommandEncoder,
}

pub struct Pass<'a, R: Render> {
    pass: RenderPass<'a>,
    _phantom: PhantomData<R>,
}

impl<'a> Renderer<'a> {
    pub fn new(encoder: &'a mut CommandEncoder) -> Self {
        Self { encoder }
    }

    pub fn begin_pass<R: Render>(&mut self) -> Pass<R> {
        let desc = RenderPassDescriptor {
            label: todo!(),
            color_attachments: todo!(),
            depth_stencil_attachment: todo!(),
            timestamp_writes: todo!(),
            occlusion_query_set: todo!(),
        };

        let mut pass = self.encoder.begin_render_pass(&desc);

        Pass {
            pass,
            _phantom: PhantomData,
        }
    }
}

impl<'a, R: Render> Pass<'a, R> {
    fn set_pipeline(&self, device: &Device) {

        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("<wgsl source code>"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed("hey")),
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("test render pipeline"),
            layout: todo!(),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: todo!(),
            },
            primitive: todo!(),
            depth_stencil: todo!(),
            multisample: todo!(),
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: todo!(),
            }),
            multiview: todo!(),
            cache: todo!(),
        });
    }

    fn render(&mut self, mesh: Mesh<R::Vertex>, instances: R::Instance) {   
    }
}

struct Mesh<V> {
    vertices: V,
}

pub trait Render {
    type Vertex;

    type Instance;

    fn shader(&self) -> Shader;
}

struct Shader {}
