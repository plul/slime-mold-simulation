# Trail: Decay

## Example: 5 % evaporation

5 % of the chemical signal evaporates per iteration:

```wgsl
trail_map[idx] = trail_map[idx] * 0.95;
```

## Model step

![Model](./images/algorithm_decay.jpg)
