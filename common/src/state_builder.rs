use pollster::FutureExt;
use wgpu::{
    Adapter, Color, CommandEncoder, Device, Instance, PresentMode, Queue, RenderPass, Surface,
    SurfaceCapabilities, TextureView,
};
use winit::dpi::PhysicalSize;

/// Creates a `SurfaceConfiguration` for a given physical size and surface capabilities.
///
/// # Arguments
///
/// * `size` - The physical size of the surface in pixels.
/// * `capabilities` - The capabilities of the surface, including supported formats and alpha modes.
///
/// # Returns
///
/// A `wgpu::SurfaceConfiguration` object configured with the provided size and capabilities.
pub fn create_surface_config(
    size: PhysicalSize<u32>,
    capabilities: SurfaceCapabilities,
) -> wgpu::SurfaceConfiguration {
    let surface_format = capabilities
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(capabilities.formats[0]);

    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::AutoNoVsync,
        alpha_mode: capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

/// Creates a `wgpu::Device` and `wgpu::Queue` from the given `Adapter`.
///
/// # Arguments
///
/// * `adapter` - A reference to the `wgpu::Adapter` from which to request the device.
///
/// # Returns
///
/// A tuple containing the created `Device` and `Queue`.
///
/// # Panics
///
/// Panics if device creation fails.
pub fn create_device(adapter: &Adapter) -> (Device, Queue) {
    adapter
        .request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: None,
            memory_hints: Default::default(),
            trace: Default::default(),
        })
        .block_on()
        .unwrap()
}

/// Creates a `wgpu::Adapter` for the given `Instance` and `Surface`.
///
/// # Arguments
///
/// * `instance` - The `wgpu::Instance` used to request the adapter.
/// * `surface` - The `wgpu::Surface` that the adapter must be compatible with.
///
/// # Returns
///
/// A `wgpu::Adapter` suitable for rendering to the provided surface.
///
/// # Panics
///
/// Panics if no suitable adapter is found.
pub fn create_adapter(instance: Instance, surface: &Surface) -> Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            // power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        // Look for wasm compatible adapter we shouldn't use block_on
        .block_on()
        .unwrap()
}

/// Creates a `RenderPass` object which records a single render pass.
///
/// # Arguments
///
/// * `encoder` - A mutable reference to the `CommandEncoder` which will record the commands.
/// * `view` - A reference to the `TextureView` that the render pass will render to.
/// * `color` - The `Color` to clear the render target with.
///
/// # Returns
///
/// * `RenderPass<'b>` - A `RenderPass` object that records the commands for a single render pass.
///
/// # Lifetimes
///
/// * `'b` - The lifetime of the `RenderPass`, which must be within the lifetime of the `CommandEncoder`.

pub fn create_render_pass<'b>(
    encoder: &'b mut CommandEncoder,
    view: &TextureView,
    color: Color,
) -> RenderPass<'b> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    })
}

// Here we can add the WASM specific code
/// Creates a new `wgpu::Instance` using the primary backend.
/// 
/// # Returns
/// 
/// A `wgpu::Instance` initialized with the primary backend.
pub fn create_gpu_instance() -> Instance {
    Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    })
}
