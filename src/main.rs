use std::sync::Arc;

use cgmath::{Matrix4, SquareMatrix, Vector3};
use sifu_render::{
    GpuBuffer,
    mesh::{MeshInstance, Vertex},
    sample::{
        sample_shader,
        sample_vertex::{SampleInstance, SampleUniform, SampleVertex},
    },
};
use wgpu::{
    Adapter, Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BlendState, Color, ColorTargetState, ColorWrites, CommandEncoder, CommandEncoderDescriptor, CompareFunction, DepthBiasState, DepthStencilState, Device, DeviceDescriptor, Extent3d, Face, Features, FragmentState, FrontFace, IndexFormat, Instance, InstanceDescriptor, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor, ShaderStages, StencilState, Surface, SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, VertexState
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

#[derive(Default)]
struct App {
    ctx: Option<RenderContext>,
}

fn main() {
    println!("{}", SampleVertex::shader_struct_str());
    println!("{}", SampleInstance::shader_struct_str());

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    // let start_time = Instant::now();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();

        self.ctx = Some(RenderContext::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(ctx) = &mut self.ctx else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                // ctx.render::<SampleSceneRenderer>();
                ctx.render();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => (),
        }
    }
}

struct SampleSceneRenderer;

impl RendererCreator for SampleSceneRenderer {
    type State = ();

    fn prepare(device: &Device, queue: &Queue) -> Self::State {
        ()
    }

    fn render(encoder: &mut CommandEncoder, state: Self::State) {
        println!("render sample!");
    }
}

struct RenderContext {
    instance: Instance,
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
}

impl RenderContext {
    pub fn new(window: Window) -> Self {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let window = Arc::new(window);

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: Some("device descriptor"),
                required_features: Features::empty(),
                ..Default::default()
            },
            None,
        ))
        .unwrap();

        Self::configure_surface(&surface, &device, &adapter, &window);

        Self {
            window,
            instance,
            surface,
            device,
            queue,
        }
    }

    fn configure_surface(surface: &Surface, device: &Device, adapter: &Adapter, window: &Window) {
        let caps = surface.get_capabilities(adapter);
        let format = caps.formats.iter().filter(|format| format.is_srgb()).next().cloned().unwrap_or(caps.formats[0]);

        let size = window.inner_size();

        surface.configure(
            device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format,
                width: size.width,
                height: size.height,
                present_mode: caps.present_modes[0],
                desired_maximum_frame_latency: 0,
                alpha_mode: caps.alpha_modes[0],
                view_formats: Vec::new(),
            },
        );
    }

    pub fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let buffer = GpuBuffer::uniform(
            &self.device,
            SampleUniform {
                value: 2.0,
                color: Vector3::new(1.0, 0.1, 0.8),
            },
        );

        let depth_texture = self.device.create_texture(&TextureDescriptor {
            label: Some("Depth texture"),
            size: Extent3d {
                width: output.texture.width(),
                height: output.texture.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth24Plus,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        let bind_group_layout = self
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Foo BindGroupLayout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Foo BindGroup"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.binding()),
            }],
        });

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Foo PipelineLayout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader_module = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("ShaderModule"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(sample_shader())),
        });

        let render_pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Foo RenderPipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: None,//Some("vert"),
                    compilation_options: Default::default(),
                    buffers: &[SampleVertex::LAYOUT, SampleInstance::LAYOUT],
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(DepthStencilState {
                    format: depth_texture.format(),
                    depth_write_enabled: true,
                    depth_compare: CompareFunction::LessEqual,
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                }),
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: None,//Some("frag"),
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState {
                        format: output.texture.format(),
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            });

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("RenderContext::render"),
            });

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Foo RenderPass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Discard,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(&render_pipeline);

        pass.set_bind_group(0, &bind_group, &[]);

        let mesh = SampleVertex::sample_mesh(&self.device);

        let instances = GpuBuffer::instances(
            &self.device,
            [SampleInstance {
                mat: Matrix4::identity(),
            }],
        );

        mesh.draw(&instances, &mut pass);

        drop(pass);

        self.queue.submit([encoder.finish()]);
        output.present();
    }
}

trait RendererCreator {
    type State;

    fn prepare(device: &Device, queue: &Queue) -> Self::State;

    fn render(encoder: &mut CommandEncoder, state: Self::State);
}
