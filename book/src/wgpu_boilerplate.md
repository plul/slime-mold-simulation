# wgpu boilerplate

## Access to GPU

```rust,noplayground
// Instance of wgpu
let instance = wgpu::Instance::default();

// Handle to physical GPU
let adapter = request_adapter(&instance, None).await;

// Connection to GPU and GPU command queue
let (device, queue) = request_device(&adapter).await;
```

## Load shaders source

```rust,noplayground
let shader_source = wgpu::ShaderSource::Wgsl(include_str!("../shaders.wgsl").into());

let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("Shader Module"),
    source: shader_source,
});
```

## Set up render pipeline

```rust,noplayground
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(&pipeline_layout),
    vertex: wgpu::VertexState {
        module: &shader_module,
        entry_point: "vertex_main",
        buffers: &[],
    },
    fragment: Some(wgpu::FragmentState {
        module: &shader_module,
        entry_point: "fragment_main",
        targets: &[Some(wgpu::ColorTargetState {
            format: surface_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })],
    }),
    primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: None,
        ..Default::default()
    },
    depth_stencil: None,
    multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
    },
    multiview: None,
});
```

## Create render passes

```rust,noplayground
let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    label: Some("Render Pass"),
    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::default()),
            store: wgpu::StoreOp::Store,
        },
    })],
    depth_stencil_attachment: None,
    timestamp_writes: None,
    occlusion_query_set: None,
});

render_pass.set_pipeline(&render_pipeline);
```
