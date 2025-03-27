use std::sync::Arc;

use sifu_render::{mesh::{MeshInstance as MeshInstance, Vertex}, sample::sample_vertex::{SampleDeriveInstance, SampleDeriveVertex}, Renderer};
use wgpu::{
    Backends, CommandEncoder, CommandEncoderDescriptor, Device, DeviceDescriptor, Features,
    Instance, InstanceDescriptor, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration,
    TextureViewDescriptor, core::instance::RequestAdapterError,
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
    println!("{}", SampleDeriveVertex::shader_struct_str());
    println!("{}", SampleDeriveInstance::shader_struct_str());

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

        Self {
            window,
            instance,
            surface,
            device,
            queue,
        }
    }

    pub fn render<R: RendererCreator>(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("RenderContext::render"),
            });

        let state = R::prepare(&self.device, &self.queue);

        R::render(&mut encoder, state);

        self.queue.submit([encoder.finish()]);
        // output.present();
    }
}

trait RendererCreator {
    type State;

    fn prepare(device: &Device, queue: &Queue) -> Self::State;

    fn render(encoder: &mut CommandEncoder, state: Self::State);
}
