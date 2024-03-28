# Shader Parameters

```rust, noplayground
#[derive(Debug, Clone, Copy, PartialEq, TypedBuilder)]
pub struct ShaderParameters {
    /// Sensor distance (in pixels)
    #[builder(default = 9.0)]
    pub sensor_distance: f32,

    /// Number of active agents
    #[builder(default = 500_000)]
    pub number_of_active_agents: u32,

    // etc...
}
```
