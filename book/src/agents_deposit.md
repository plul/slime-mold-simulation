# Agents: Deposit chemoattractants

```wgsl
// Determine trail map pixel index from agent position
let pixel_x = u32(round(agent.position.x));
let pixel_y = u32(round(agent.position.y));
let pixel_idx = pixel_y * ctx.canvas_width + pixel_x;

// Deposit chemoattractant
trail_map.data[pixel_idx] = 1.0;
```

## Model step

![Model](./images/algorithm_deposit.jpg)

## Turn it on...

[Link to controls](http://127.0.0.1:8080/presentation/particlemodel)
