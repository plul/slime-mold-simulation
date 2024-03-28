# UI - Keybindings only gets us so far

## winit event loop - Keybindings

```rust
WindowEvent::KeyboardInput { event, .. } => match event {
    KeyEvent {
        logical_key,
        state
        ..
    } => match logical_key {
      Key::Character(c) if c.as_ref() == "r" && matches!(state, ElementState::Released) => {
          send_command(&ctx.user_input_tx, common::Command::RandomizeParameters);
      }
```

## Actual UI

We need

- Buttons
- Check boxes
- Sliders

## How

- UI on top of winit/wgpu window?
- Separate UI?
