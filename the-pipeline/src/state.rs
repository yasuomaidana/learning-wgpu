use common::state_builder::{create_adapter, create_device, create_gpu_instance, create_render_pass, create_surface_config};
use std::sync::Arc;
use wgpu::{Color, Device, PipelineCompilationOptions, Queue, RenderPipeline, Surface};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;
use common::event_handler::pressure_event_handler::handle_pressure_event;

pub struct State<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    blue: f64,
    window: Arc<Window>,
    // Pipeline
    render_pipeline: RenderPipeline,
}

impl<'a> State<'a> {
    pub fn new(window: Window) -> State<'a> {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        // Instance is used to create surfaces and adapters
        let instance = create_gpu_instance();

        // The surface is the "window" that we will render the
        // graphics to
        // It is the part of the window that we draw to.
        let surface = instance
            .create_surface(window_arc.clone())
            .expect("Failed to create surface");

        // Adapter is a handle for our actual graphics card
        let adapter = create_adapter(instance, &surface);

        // Device is the handle to the GPU. Responsible for the creation of most rendering and compute resources.
        // Queue is the handle to the command queue. Responsible for submitting commands to the GPU.
        let (device, queue) = create_device(&adapter);

        // SurfaceCapabilities are the capabilities of the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let config = create_surface_config(size, surface_caps);
        surface.configure(&device, &config);


        // long way
        // let shader = device.create_shader_module(ShaderModuleDescriptor {
        //     label: Some("Shader"),
        //     source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        // });
        
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = create_pipeline_layout(&device, "Render Pipeline Layout");

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor{
                label:Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    // compilation_options: Default::default(),
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    compilation_options: PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            }
        );
        

        Self {
            surface,
            device,
            queue,
            config,
            size,
            blue: 0.0,
            window: window_arc,
            render_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;

        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self
            .surface
            .get_current_texture()
            .expect("Failed to get texture");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            // println!("Blue: {}", self.blue);
            let mut render_pass = create_render_pass(
                &mut encoder,
                &view,
                Color {
                    r: 0.1,
                    g: 0.2,
                    b: self.blue,
                    a: 1.0,
                },
            );
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3,0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Handles input events and returns a boolean indicating whether the event has been fully processed.
    ///
    /// If the method returns `true`, the main loop won't process the event any further.
    ///
    /// # Arguments
    ///
    /// * `event` - A reference to the `WindowEvent` that needs to be processed.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the event has been fully processed, `false` otherwise.
    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::TouchpadPressure { pressure, .. } => {
                self.blue = handle_pressure_event(*pressure, self.blue);
                return true;
            }
            _ => {}
        }
        false
    }

    pub fn update(&mut self) {
        // Update the state of the application
        self.render().unwrap();
    }
}
