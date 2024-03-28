# GPU Interfacing

![IPC](./images/architecture_shader_param_sync.png)

## Sending `ShaderParameters` from CPU to GPU

```diff
#[derive(Debug, Clone, Copy, PartialEq, TypedBuilder)]
#[derive(Serialize, Deserialize)]
+
+ // All bytes can safely be set to zero and it will still make sense
+ #[derive(bytemuck::Zeroable)] 
+
+ // No bytes are ever left un-initialized
+ #[derive(bytemuck::NoUninit)] 
+
+ // Reliable data layout
+ #[repr(C)] 
pub struct ShaderParameters {
    /// Sensor distance (in pixels)
    #[builder(default = 9.0)]
    pub sensor_distance: f32,

    // etc...
}
```

allows for safely casting the struct as bytes, and re-building the struct on the GPU shader side

```rust,noplayground
fn write_shader_parameters_to_gpu(state: &State, shader_parameters: &ShaderParameters) {
    // Convert ShaderParameters struct to byte slice
    let bytes: &[u8] = bytemuck::cast_slice(&[shader_parameters]);

    // Write bytes to GPU buffer
    state.gpu.queue.write_buffer(
        &state.resources.shader_parameters.buffer,
        0, // offset
        bytes
    );
}
```
