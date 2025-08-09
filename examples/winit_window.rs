#![allow(unused)]

use std::{sync::Arc, time::Instant};

use cgmath::{Basis3, Deg, Matrix4, One, Vector2, Vector3};
use sifu_render::shader::Shader;
use sifu_render::UniformExt;
use sifu_render::{
    GpuBuffer,
    sample::{
        camera_uniform::{Camera, FooUniformsDerive},
        sample_shader,
        sample_vertex::{SampleInstance, SampleUniform, SampleVertex},
    },
    texture::{Color, DepthStencilPixel, ImageTexture, PixelFormat, RenderTexture},
    uniform_binding::UniformBinding,
};
use wgpu::{
    Adapter, Backends, BindGroup, BindGroupDescriptor, BindGroupLayoutDescriptor, BlendState, ColorTargetState, ColorWrites, CommandEncoder, CommandEncoderDescriptor, CompareFunction, DepthBiasState, DepthStencilState, Device, DeviceDescriptor, Features, FragmentState, FrontFace, Instance, InstanceDescriptor, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, SamplerDescriptor, StencilFaceState, StencilState, Surface, TextureFormat, TextureView
};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalSize, Size},
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

#[derive(Default)]
struct App {
    ctx: Option<RenderContext>,
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    // let start_time = Instant::now();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_min_inner_size(Size::Physical(PhysicalSize::new(64, 32))),
            )
            .unwrap();
        window.focus_window();

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
            WindowEvent::Resized(new_size) => {
                let size = Vector2::new(new_size.width, new_size.height);

                ctx.resize_surface(size);
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(ctx) = &mut self.ctx else {
            return;
        };

        ctx.window.request_redraw();
    }
}

struct RenderContext {
    instance: Instance,
    window: Arc<Window>,
    adapter: Adapter,
    surface: Surface<'static>,
    depth_texture: RenderTexture<DepthStencilPixel, true>,
    multisample_texture: RenderTexture<(), true>,
    mesh_texture: ImageTexture,
    device: Device,
    queue: Queue,
    start_time: Instant,
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
        ))
        .unwrap();

        let window_size = window.inner_size();
        let window_size = Vector2::new(window_size.width, window_size.height);

        Self::configure_surface(&surface, &device, &adapter, &window);

        let surface_format = Self::get_format(&surface, &adapter);

        let depth_texture = RenderTexture::new(&device, window_size);
        let multisample_texture = RenderTexture::new_format(&device, window_size, surface_format);

        let mesh_texture =
            ImageTexture::new_procedural(&device, &queue, Vector2::new(16, 16), |pos| {
                if (pos.x + pos.y) % 2 == 0 {
                    Color::BLACK
                } else {
                    Color::WHITE
                }
            });

        Self {
            window,
            instance,
            adapter,
            surface,
            depth_texture,
            multisample_texture,
            mesh_texture,
            device,
            queue,
            start_time: Instant::now(),
        }
    }

    fn configure_surface(surface: &Surface, device: &Device, adapter: &Adapter, window: &Window) {
        let size = window.inner_size();

        let config = surface
            .get_default_config(adapter, size.width, size.height)
            .unwrap();

        surface.configure(device, &config);
    }

    fn get_format(surface: &Surface, adapter: &Adapter) -> TextureFormat {
        surface.get_capabilities(adapter).formats[0]
    }

    fn resize_surface(&mut self, size: Vector2<u32>) {
        self.surface.configure(
            &self.device,
            &self
                .surface
                .get_default_config(&self.adapter, size.x, size.y)
                .unwrap(),
        );
        self.depth_texture.resize(&self.device, size);
        self.multisample_texture.resize(&self.device, size);
    }

    pub fn render(&mut self) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let output = RenderTexture::from_surface_texture(surface_texture);

        // Create uniforms and bind groups

        let sample_uniform = SampleUniform {
            opacity: 0.9,
            color_a: Vector3::new(1.0, 0.0, 0.5),
            color_b: Vector3::new(0.2, 0.9, 0.4),
        }
        .buffer(&self.device);

        let sample_uniform2 = GpuBuffer::uniform(
            &self.device,
            SampleUniform {
                opacity: 0.0,
                color_a: Vector3::new(0.4, 0.1, 1.0),
                color_b: Vector3::new(0.1, 0.2, 0.6),
            },
        );

        let camera = Camera {
            position: Vector3::new(0.0, 0.0, 2.0),
            rotation: Basis3::one(),
            fovy: Deg(90.0).into(),
            screen_size: output.size().cast().unwrap(),
        }
        .uniform()
        .buffer(&self.device);

        let sampler = self.device.create_sampler(&SamplerDescriptor::default());

        let foo_uniforms = FooUniformsDerive {
            sample: &sample_uniform,
            camera: &camera,
            texture: &self.mesh_texture,
            tex_sampler: &sampler,
        };

        let foo_uniforms2 = FooUniformsDerive {
            sample: &sample_uniform2,
            camera: &camera,
            texture: &self.mesh_texture,
            tex_sampler: &sampler,
        };

        let bind_group_layout = self
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Foo BindGroupLayout"),
                entries: FooUniformsDerive::LAYOUT,
            });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Foo BindGroup"),
            layout: &bind_group_layout,
            entries: &foo_uniforms.binding_entries(),
        });

        let bind_group2 = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Foo BindGroup2"),
            layout: &bind_group_layout,
            entries: &foo_uniforms2.binding_entries(),
        });

        // Create pipeline layout

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Foo PipelineLayout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = sample_shader(&self.device);

        let render_pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Foo RenderPipeline"),
                layout: Some(&pipeline_layout),
                vertex: shader.vertex_state(),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None, // Some(Face::Back),
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(DepthStencilState {
                    format: DepthStencilPixel::FORMAT,
                    depth_write_enabled: true,
                    depth_compare: CompareFunction::LessEqual,
                    stencil: StencilState {
                        front: StencilFaceState {
                            compare: CompareFunction::GreaterEqual,
                            fail_op: wgpu::StencilOperation::Keep,
                            depth_fail_op: wgpu::StencilOperation::Keep,
                            pass_op: wgpu::StencilOperation::Replace,
                        },
                        back: StencilFaceState {
                            compare: CompareFunction::GreaterEqual,
                            fail_op: wgpu::StencilOperation::Keep,
                            depth_fail_op: wgpu::StencilOperation::Keep,
                            pass_op: wgpu::StencilOperation::Replace,
                        },
                        read_mask: !0,
                        write_mask: !0,
                    },
                    bias: DepthBiasState::default(),
                }),
                multisample: MultisampleState {
                    count: 4,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(FragmentState {
                    module: &shader.module(),
                    entry_point: Some(Shader::<(), ()>::ENTRY_POINT_FRAGMENT),
                    targets: &[Some(ColorTargetState {
                        format: output.format(),
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                    compilation_options: PipelineCompilationOptions::default()
                }),
                multiview: None,
                cache: None,
            });

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("RenderContext::render"),
            });

        let target = RenderTarget::MultisampleResolve {
            multisample_color: &self.multisample_texture,
            resolve_color: &output,
            depth: Some(&self.depth_texture),
        };

        Self::render_pass(
            &self.device,
            &mut encoder,
            target,
            &render_pipeline,
            &bind_group,
            &bind_group2,
            self.start_time,
        );

        self.queue.submit([encoder.finish()]);
        output.present();
    }

    fn render_pass(
        device: &Device,
        encoder: &mut CommandEncoder,
        target: RenderTarget<()>,
        pipeline: &RenderPipeline,
        bind_group: &BindGroup,
        bind_group2: &BindGroup,
        start_time: Instant,
    ) {
        let mesh = SampleVertex::sample_mesh(device);

        let time = Instant::now();
        let secs = time.duration_since(start_time).as_secs_f32();

        let instance = GpuBuffer::instances(
            device,
            [SampleInstance {
                mat: Matrix4::from_angle_y(Deg(secs * 180.0)),
            }],
        );

        let instance2 = GpuBuffer::instances(
            device,
            [SampleInstance {
                mat: Matrix4::from_translation(Vector3::new(0.0, 0.0, -5.0))
                    * Matrix4::from_angle_y(Deg(secs * -90.0)),
            }],
        );

        let instance3 = GpuBuffer::instances(
            device,
            [SampleInstance {
                mat: Matrix4::from_translation(Vector3::new(0.0, (secs * 0.1).sin() * 15.0, -50.0))
                    * Matrix4::from_scale(20.0)
                    * Matrix4::from_angle_y(Deg(secs * 10.0)),
            }],
        );

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Foo RenderPass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: target.view(),
                resolve_target: target.resolve_target(),
                ops: Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: target.depth().map(|view| RenderPassDepthStencilAttachment {
                view,
                depth_ops: Some(Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Discard,
                }),
                stencil_ops: Some(Operations {
                    load: wgpu::LoadOp::Clear(0),
                    store: wgpu::StoreOp::Discard,
                }),
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(pipeline);



        // draw big mesh in the far back
        // pass.set_stencil_reference(2);
        pass.set_bind_group(0, bind_group, &[]);
        mesh.draw(&instance3, &mut pass);

        // draw blue mesh in the back
        pass.set_stencil_reference(2);
        pass.set_bind_group(0, bind_group2, &[]);
        mesh.draw(&instance2, &mut pass);

        // draw rotating front mesh
        pass.set_stencil_reference(1);
        pass.set_bind_group(0, bind_group, &[]);
        mesh.draw(&instance, &mut pass);
    }
}

enum RenderTarget<'a, P> {
    NonMultisample {
        color: &'a RenderTexture<P, false>,
        depth: Option<&'a RenderTexture<DepthStencilPixel, false>>,
    },

    Multisample {
        color: &'a RenderTexture<P, true>,
        depth: Option<&'a RenderTexture<DepthStencilPixel, true>>,
    },

    MultisampleResolve {
        multisample_color: &'a RenderTexture<P, true>,
        resolve_color: &'a RenderTexture<P, false>,
        depth: Option<&'a RenderTexture<DepthStencilPixel, true>>,
    },
}

impl<'a, P> RenderTarget<'a, P> {
    fn is_multisampled(&self) -> bool {
        match self {
            RenderTarget::NonMultisample { .. } => false,
            RenderTarget::Multisample { .. } => true,
            RenderTarget::MultisampleResolve { .. } => false,
        }
    }

    fn view(&self) -> &TextureView {
        match self {
            RenderTarget::NonMultisample { color, .. } => color.view(),
            RenderTarget::Multisample { color, .. } => color.view(),
            RenderTarget::MultisampleResolve {
                multisample_color, ..
            } => multisample_color.view(),
        }
    }

    fn resolve_target(&self) -> Option<&TextureView> {
        match self {
            RenderTarget::NonMultisample { .. } => None,
            RenderTarget::Multisample { .. } => None,
            RenderTarget::MultisampleResolve { resolve_color, .. } => Some(resolve_color.view()),
        }
    }

    fn depth(&self) -> Option<&TextureView> {
        match self {
            RenderTarget::NonMultisample {
                depth: Some(depth), ..
            } => Some(depth.view()),
            RenderTarget::Multisample {
                depth: Some(depth), ..
            } => Some(depth.view()),
            RenderTarget::MultisampleResolve {
                depth: Some(depth), ..
            } => Some(depth.view()),
            _ => None,
        }
    }
}
