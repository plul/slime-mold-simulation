# Fragment Shader re-visited

## Sample from trail map texture

```wgsl
struct TrailMap {
    data: array<f32>,
}

@group(2) @binding(0)
var<storage, read_write> trail_map: TrailMap;

@fragment
fn fragment_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    let x = u32(coord.x);
    let y = u32(coord.y);
    let idx = y * ctx.canvas_width + x;

    // Sample trail map (chemoattractants):
    let v = trail_map.data[idx];

    // Return grascale value (rgba)
    return vec4(v, v, v, 1.0);
}
```

## Note: Do we draw the agents? No!

- We only draw the trail of chemicals that they leave behind

## Turn this thing on...

[Link to controls](http://127.0.0.1:8080/presentation/triangles)
