---
id: Badge
section: components
cssPrefix: pf-v5-c-badge
---## Examples

### Read

```html
<span class="pf-v5-c-badge pf-m-read">7</span>
<span class="pf-v5-c-badge pf-m-read">24</span>
<span class="pf-v5-c-badge pf-m-read">240</span>
<span class="pf-v5-c-badge pf-m-read">999+</span>

```

### Unread

```html
<span class="pf-v5-c-badge pf-m-unread">
  7
  <span class="pf-v5-screen-reader">unread messages</span>
</span>
<span class="pf-v5-c-badge pf-m-unread">
  24
  <span class="pf-v5-screen-reader">unread messages</span>
</span>
<span class="pf-v5-c-badge pf-m-unread">
  240
  <span class="pf-v5-screen-reader">unread messages</span>
</span>
<span class="pf-v5-c-badge pf-m-unread">
  999+
  <span class="pf-v5-screen-reader">unread messages</span>
</span>

```

## Documentation

### Overview

Always add a modifier class. Never use the class `.pf-v5-c-badge` on its own.

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-badge` | `<span>` | Initiates a badge. **Always use with a modifier class.** |
| `.pf-m-read` | `.pf-v5-c-badge` | Applies read badge styling. |
| `.pf-m-unread` | `.pf-v5-c-badge` | Applies unread badge styling. |
