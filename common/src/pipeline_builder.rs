use wgpu::{
    Device, PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor, RenderPipeline,
    ShaderModule,
};

pub fn create_pipeline_layout(device: &Device, label_name: &str) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some(label_name),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    })
}

/// Creates a pipeline layout with the specified bind group layouts.
///
/// # Arguments
///
/// * `device` - A reference to the `wgpu::Device` used to create the pipeline layout.
/// * `label_name` - A string slice that specifies the label for the pipeline layout.
/// * `bind_group_layouts` - A slice of references to `wgpu::BindGroupLayout` objects to be used in the pipeline layout.
/// # Returns
///
/// A `wgpu::PipelineLayout` created with the provided bind group layouts.
pub fn create_pipeline_layout_with_bind_groups(
    device: &Device,
    label_name: &str,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some(label_name),
        bind_group_layouts,
        push_constant_ranges: &[],
    })
}

/// Creates a render pipeline with the specified vertex buffer layouts.
///
/// # Arguments
///
/// * `device` - A reference to the `wgpu::Device` used to create the render pipeline.
/// * `pipeline_layout` - A reference to the `wgpu::PipelineLayout` for the pipeline.
/// * `shader_module` - A reference to the `wgpu::ShaderModule` containing the shader code.
/// * `config` - A reference to the `wgpu::SurfaceConfiguration` for the pipeline.
/// * `pipe_line_label` - A string slice specifying the label for the pipeline.
/// * `vertex_entrypoint` - A string slice specifying the entry point for the vertex shader.
/// * `fragment_entrypoint` - A string slice specifying the entry point for the fragment shader.
/// * `buffers` - A slice of `wgpu::VertexBufferLayout` specifying the vertex buffer layouts.
pub fn create_render_pipeline_with_buffers(
    device: &Device,
    pipeline_layout: &PipelineLayout,
    shader_module: &ShaderModule,
    config: &wgpu::SurfaceConfiguration,
    pipe_line_label: &str,
    vertex_entrypoint: &str,
    fragment_entrypoint: &str,
    buffers: &[wgpu::VertexBufferLayout],
) -> RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(pipe_line_label),
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: Some(vertex_entrypoint),
            buffers,
            // compilation_options: Default::default(),
            compilation_options: PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: Some(fragment_entrypoint),
            targets: &[Some(wgpu::ColorTargetState {
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
    })
}

pub fn create_render_pipeline(
    device: &Device,
    pipeline_layout: &PipelineLayout,
    shader_module: &ShaderModule,
    config: &wgpu::SurfaceConfiguration,
    pipe_line_label: &str,
    vertex_entrypoint: &str,
    fragment_entrypoint: &str,
) -> RenderPipeline {
    create_render_pipeline_with_buffers(
        device,
        pipeline_layout,
        shader_module,
        config,
        pipe_line_label,
        vertex_entrypoint,
        fragment_entrypoint,
        &[],
    )
}
