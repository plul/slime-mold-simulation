# Vertex shader

_Vertex shader:_
Function that computes the coordinates of the vertices (corners) of the triangles to render.

![Vertices](./images/vertices.png)

Image attribution: <https://sotrh.github.io/learn-wgpu/>

## Render pass on the Rust side

Invoke render pass:

```rust,noplayground
let vertices = 0..3;
let instances = 0..1;
render_pass.draw(vertices, instances);
```

### Our WGSL vertex shader:

- Simply outputs a hardcoded triangle:

```wgsl
@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var position: vec2<f32>;
    switch (vertex_index) {
        // Output one of the vertices of the triangle:
        case 0u: { position = vec2<f32>(-0.8, -0.4); }
        case 1u: { position = vec2<f32>(0.3, -0.05); }
        case 2u: { position = vec2<f32>(-0.35, 0.2); }

        default: { position = vec2<f32>(0.0, 0.0); } // This should never happen
    }

    let clip_position = vec4<f32>(position, 0.0, 1.0);
    return clip_position;
}
```
