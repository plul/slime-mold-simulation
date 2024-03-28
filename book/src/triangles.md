# Baby steps: Draw a Triangle

- wgpu is pretty close to the metal
- ❤️ GPUs love triangles

## How to draw a triangle

1. Tell the GPU where the corners of the triangle should be
2. Tell the GPU what color to apply to the pixels inside the triangle

## WebGPU coordinate system

```
                    +1y
                     │
                     │
                     │
                     │
                     │
                     │
                     │
                     │
-1x ─────────────────┼───────────────➤ +1x
                     │
                     │
                     │
                     │
                     │
                     │
                     │
                     ▼
                    -1y
```

## Corners of our triangle

- P1: (-0.8, -0.4)
- P2: (0.3, -0.05)
- P3: (-0.35, 0.2)

```
                    +1y
                     │
                     │
                     │
                     │
                     │
                     │
              P3     │
                     │
-1x ─────────────────┼────────────────➤ +1x
                     │    P2
                     │
                     │
      P1             │
                     │
                     │
                     │
                     ▼
                    -1y
```
