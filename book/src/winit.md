# Create a window

## winit

> winit - Cross-platform window creation and management in Rust

## Create event loop and window:

```rust,noplayground
    // Create event loop
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    // Run event loop
    event_loop.run(|event, event_loop_handle| {

        // Use "Init" event to create window:
        if matches!(event, Event::NewEvents(StartCause::Init)) {
            let window_attributes = winit::window::Window::default_attributes()
                .with_title("Slime Mold Simulation")
                .with_inner_size(window_size)
                .with_min_inner_size(window_size);

            let window = event_loop_handle.create_window(window_attributes).unwrap();
        }

        // ... handle other events

    })
```

## Run the code: Where is the window?

- Quirk: On Linux/Wayland the window does not exist until its contents have been drawn once

So we need to render to it...
