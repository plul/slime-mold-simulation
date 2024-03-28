---
id: Display
section: utility-classes
---import './Display.css'

## Examples

### Inline block

```html
<div class="pf-v5-u-display-inline-block">.pf-v5-u-display-inline-block</div>

```

### Block

```html
<div class="pf-v5-u-display-block">.pf-v5-u-display-block</div>

```

### Flex

```html
<div class="pf-v5-u-display-flex">.pf-v5-u-display-flex</div>

```

### Inline flex

```html
<div class="pf-v5-u-display-inline-flex">.pf-v5-u-display-inline-flex</div>

```

### Grid

```html
<div class="pf-v5-u-display-grid">.pf-v5-u-display-grid</div>

```

### Inline

```html
<div class="pf-v5-u-display-inline">.pf-v5-u-display-inline</div>

```

### Table

```html
<div class="pf-v5-u-display-table">
  <div class="pf-v5-u-display-table-row">
    <div class="pf-v5-u-display-table-cell">table-cell</div>
    <div class="pf-v5-u-display-table-cell">table-cell</div>
    <div class="pf-v5-u-display-table-cell">table-cell</div>
  </div>
  <div class="pf-v5-u-display-table-row">
    <div class="pf-v5-u-display-table-cell">table-cell</div>
    <div class="pf-v5-u-display-table-cell">table-cell</div>
    <div class="pf-v5-u-display-table-cell">table-cell</div>
  </div>
</div>

```

### None

```html
<div class="pf-v5-u-display-none-on-sm">Hidden on sm breakpoint</div>

```

## Documentation

### Overview

[Breakpoint](/developer-resources/global-css-variables#breakpoint-variables-and-class-suffixes) is optional. Breakpoints: base (no breakpoint value), `-on-sm`, `-on-md`, `-on-lg`, and `-on-xl`. **Example .pf-v5-u-display-inline-block-on-lg**

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-u-display-inline-block{-on-[breakpoint]}` | `*` |  Sets display: inline-block |
| `.pf-v5-u-display-block{-on-[breakpoint]}` | `*` |  Sets display: block |
| `.pf-v5-u-display-inline{-on-[breakpoint]}` | `*` |  Sets display: inline |
| `.pf-v5-u-display-flex{-on-[breakpoint]}` | `*` |  Sets display: flex |
| `.pf-v5-u-display-inline-flex{-on-[breakpoint]}` | `*` |  Sets display: inline-flex |
| `.pf-v5-u-display-table{-on-[breakpoint]}` | `*` |  Sets display: table |
| `.pf-v5-u-display-table-row{-on-[breakpoint]}` | `*` |  Sets display: table-row |
| `.pf-v5-u-display-table-cell{-on-[breakpoint]}` | `*` |  Sets display: table-cell |
| `.pf-v5-u-display-none{-on-[breakpoint]}` | `*` |  Sets display: none |
