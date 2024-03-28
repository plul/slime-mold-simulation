# GPU Interfacing: Booleans?

```diff
#[derive(bytemuck::Zeroable)]
#[derive(bytemuck::NoUninit)] // <-- This causes the compilation error
#[repr(C)]
pub struct ShaderParameters {
    pub enable_agent_bounce: bool,

    // etc...
}
```

![Where we are going](./images/where_we_are_going.png)

## Compilation error

```plaintext
   Compiling common v0.1.0
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
  --> (...)/crates/common/src/simulation.rs:89:12
   |
89 | pub struct ShaderParameters {
   |            ^^^^^^^^^^^^^^^^
   |
   = note: source type: `ShaderParameters` (768 bits)
   = note: target type: `TypeWithoutPadding` (744 bits)
```

> cannot transmute between types of different sizes, or dependently-sized types

## We just don't have the technology. Workaound:

```rust,noplayground
    // BOOLEAN! Set it to 0 or 1
    #[builder(default = 1)]
    pub bool_enable_agent_bounce: u32, 
```

## My wishlist for the next generation of Graphics APIs

- Support for CPUs and GPUs to exchange booleans 
