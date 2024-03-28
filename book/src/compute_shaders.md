# Simulating on the GPU: Compute shaders

- Vertex shader: Determines where the triangles are
- Fragment shader: Colors the triangles
- **Compute shader: General purpose computing on the GPU**

## Setup: Mirror the types 

Example: Agent structs

### Rust `Agent` struct

```rust,noplayground
#[derive(Debug, Copy, Clone)]
struct Agent {
    position: [f32; 2],
    velocity: [f32; 2],
}
```

### WGSL shader `Agent` struct

```wgsl
struct Agent {
    position: vec2<f32>,
    velocity: vec2<f32>,
}
```

## Compute shader - run one GPU thread per agent

```wgsl
@compute
fn compute_agents(agent_idx: u32) {
    var agent = agents_buffer[agent_idx];

    // 1. Sense

    // 2. Rotate

    // 3. Move

    // 4. Deposit

    // 5. Diffuse

    // 6. Decay
}
```

## `fn compute_agents(agent_idx: u32)`? Not so fast...

- You don't get `agent_idx` that easy
