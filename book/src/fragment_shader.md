# Fragment shader

_Fragment shader:_
Function that computes the color of each fragment (pixel) inside the triangles on screen.

## Paint the triangles red

```wgsl
@fragment
fn fragment_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    // Output a solid red color
    return vec4(1.0, 0.0, 0.0, 1.0);
}
```
