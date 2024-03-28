---
id: Accessibility
section: utility-classes
---## Examples

### Screen reader only

```html
Content available only to screen reader, open inspector to investigate
<span
  class="pf-v5-u-screen-reader"
>This content is intended to be announced by assistive technologies, but not visually presented.</span>

```

### Visible

```html
<span
  class="pf-v5-u-visible"
>This unsets .pf-v5-u-screen-reader and .pf-v5-screen-reader. It will be visible.</span>

```

### Hidden

```html
The text underneath is hidden.
<span class="pf-v5-u-hidden">This text is hidden.</span>

```

## Documentation

### Overview

[Breakpoint](/developer-resources/global-css-variables#breakpoint-variables-and-class-suffixes) is optional. Breakpoints: base (no breakpoint value), `-on-sm`, `-on-md`, `-on-lg`, and `-on-xl`. **Example .pf-v5-u-screen-reader-on-lg**

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-u-screen-reader{-on-[breakpoint]}` | `*` |  Visually hides element, but leaves accessible to assistive technologies |
| `.pf-v5-u-visible{-on-[breakpoint]}` | `*` |  Unsets `.pf-v5-u-screen-reader` and `.pf-v5-screen-reader` |
