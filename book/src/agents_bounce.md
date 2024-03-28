# Agents: Bounce

## Bounce on window borders

```wgsl
// Bounce on left edge
if (agent.position.x < 0.0) {
    agent.velocity.x = abs(agent.velocity.x);
}

// Bounce on right edge
if (agent.position.x > f32(ctx.canvas_width)) {
    agent.velocity.x = -abs(agent.velocity.x);
}

// Bounce on top edge
if (agent.position.y < 0.0) {
    agent.velocity.y = abs(agent.velocity.y);
}

// Bounce on bottom edge
if (agent.position.y > f32(ctx.canvas_height)) {
    agent.velocity.y = -abs(agent.velocity.y);
}
```

[Link to controls](http://127.0.0.1:8080/presentation/particlemodel)
