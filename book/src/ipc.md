# Inter Process Communication

![IPC](./images/architecture_ws.png)

## Sending `ShaderParameters` over WebSocket

```diff
#[derive(Debug, Clone, Copy, PartialEq, TypedBuilder)]
+ #[derive(Serialize, Deserialize)]
pub struct ShaderParameters {
    /// Sensor distance (in pixels)
    #[builder(default = 9.0)]
    pub sensor_distance: f32,

    // etc...
}
```

allows for 

```rust,noplayground
impl ShaderParameters {
    /// Serializes (to JSON)
    pub fn ser(&self) -> String {
        serde_json::to_string(&self).expect("Failed serialization")
    }

    /// Deserializes (from JSON)
    pub fn deser(text: &str) -> Self {
        serde_json::from_str::<Self>(&text).expect("Failed deserialization")
    }
}
```
