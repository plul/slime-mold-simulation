# Trail: Diffuse

```wgsl
var sum: f32 = 0.0;
for (var i = -1; i <= 1; i = i + 1) {
    for (var j = -1; j <= 1; j = j + 1) {
        // Out of bounds detection omitted

        let xi = x + i;
        let yi = y + j;
        let idx = yi * ctx.canvas_width + xi;
        sum = sum + trail_map[idx];
    }
}

let diffused: f32 = sum / 9.0;

trail_map[idx] = diffused;
```

## Model step

![Model](./images/algorithm_diffuse.jpg)
