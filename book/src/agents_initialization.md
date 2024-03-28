# Agents: Initialize buffer

## Agent struct

```rust,noplayground
#[derive(Debug, Copy, Clone)]
struct Agent {
    /// Position
    position: [f32; 2],

    /// Velocity vector
    velocity: [f32; 2],
}
```

## Initialize agents

```rust,noplayground
let agents: Vec<Agent> = (0..number_of_agents)
    .map(|_| Agent::random_start_position())
    .collect();

gpu.queue.write_buffer(
    agents_buffer, // Handle to GPU buffer
    0, // Offset
    bytemuck::cast_slice(&agents)
);
```
