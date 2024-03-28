---
id: 'Masthead'
section: components
cssPrefix: pf-v5-c-masthead
---## Examples

### Basic

```html
<header class="pf-v5-c-masthead" id="basic-masthead">
  <span class="pf-v5-c-masthead__toggle">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Global navigation"
    >
      <i class="fas fa-bars" aria-hidden="true"></i>
    </button>
  </span>
  <div class="pf-v5-c-masthead__main">
    <a class="pf-v5-c-masthead__brand" href="#">Logo</a>
  </div>
  <div class="pf-v5-c-masthead__content">
    <span>Content</span>
  </div>
</header>

```

### Basic with mixed content

```html
<header class="pf-v5-c-masthead" id="basic-masthead-with-mixed-content">
  <span class="pf-v5-c-masthead__toggle">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Global navigation"
    >
      <i class="fas fa-bars" aria-hidden="true"></i>
    </button>
  </span>
  <div class="pf-v5-c-masthead__main">
    <a class="pf-v5-c-masthead__brand" href="#">Logo</a>
  </div>
  <div class="pf-v5-c-masthead__content">
    <div class="pf-v5-l-flex">
      <span>Content</span>
      <button class="pf-v5-c-button pf-m-primary" type="button">Primary</button>
      <button class="pf-v5-c-button pf-m-secondary" type="button">Secondary</button>
      <button class="pf-v5-c-button pf-m-tertiary" type="button">Tertiary</button>
    </div>
  </div>
</header>

```

### Display inline

```html
<header class="pf-v5-c-masthead pf-m-display-inline" id="inline-masthead">
  <span class="pf-v5-c-masthead__toggle">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Global navigation"
    >
      <i class="fas fa-bars" aria-hidden="true"></i>
    </button>
  </span>
  <div class="pf-v5-c-masthead__main">
    <a class="pf-v5-c-masthead__brand" href="#">Logo</a>
  </div>
  <div class="pf-v5-c-masthead__content">
    <span>Content</span>
  </div>
</header>

```

### Display stack

```html
<header class="pf-v5-c-masthead pf-m-display-stack" id="stack-masthead">
  <span class="pf-v5-c-masthead__toggle">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Global navigation"
    >
      <i class="fas fa-bars" aria-hidden="true"></i>
    </button>
  </span>
  <div class="pf-v5-c-masthead__main">
    <a class="pf-v5-c-masthead__brand" href="#">Logo</a>
  </div>
  <div class="pf-v5-c-masthead__content">
    <span>Content</span>
  </div>
</header>

```

### Display stack, display inline responsive

```html
<header
  class="pf-v5-c-masthead pf-m-display-inline pf-m-display-stack-on-lg pf-m-display-inline-on-2xl"
  id="stack-inline-masthead"
>
  <span class="pf-v5-c-masthead__toggle">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Global navigation"
    >
      <i class="fas fa-bars" aria-hidden="true"></i>
    </button>
  </span>
  <div class="pf-v5-c-masthead__main">
    <a class="pf-v5-c-masthead__brand" href="#">Logo</a>
  </div>
  <div class="pf-v5-c-masthead__content">
    <span>Content</span>
  </div>
</header>

```

### Light variant

```html
<header class="pf-v5-c-masthead pf-m-light" id="light-masthead">
  <span class="pf-v5-c-masthead__toggle">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Global navigation"
    >
      <i class="fas fa-bars" aria-hidden="true"></i>
    </button>
  </span>
  <div class="pf-v5-c-masthead__main">
    <a class="pf-v5-c-masthead__brand" href="#">Logo</a>
  </div>
  <div class="pf-v5-c-masthead__content">
    <span>Content</span>
  </div>
</header>

```

### Light 200 variant

```html
<header class="pf-v5-c-masthead pf-m-light-200" id="light-200-masthead">
  <span class="pf-v5-c-masthead__toggle">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Global navigation"
    >
      <i class="fas fa-bars" aria-hidden="true"></i>
    </button>
  </span>
  <div class="pf-v5-c-masthead__main">
    <a class="pf-v5-c-masthead__brand" href="#">Logo</a>
  </div>
  <div class="pf-v5-c-masthead__content">
    <span>Content</span>
  </div>
</header>

```

### Insets

```html
<header class="pf-v5-c-masthead pf-m-inset-sm" id="inset-masthead">
  <span class="pf-v5-c-masthead__toggle">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Global navigation"
    >
      <i class="fas fa-bars" aria-hidden="true"></i>
    </button>
  </span>
  <div class="pf-v5-c-masthead__main">
    <a class="pf-v5-c-masthead__brand" href="#">Logo</a>
  </div>
  <div class="pf-v5-c-masthead__content">
    <span>Content</span>
  </div>
</header>

```

## Documentation

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-masthead` | `<header>` | Initiates the masthead component. **Required** |
| `.pf-v5-c-masthead__main` | `<div>` | Initiates the masthead main component. **Required** |
| `.pf-v5-c-masthead__toggle` | `<span>` | Initiates the masthead toggle component. |
| `.pf-v5-c-masthead__brand` | `<a>, <div>` | Initiates the masthead content component. |
| `.pf-v5-c-masthead__content` | `<div>` | Initiates the masthead content component. |
| `.pf-m-inset-{none, sm, md, lg, xl, 2xl}{-on-[breakpoint]}` | `.pf-v5-c-masthead` | Modifies masthead horizontal padding at optional [breakpoint](/developer-resources/global-css-variables#breakpoint-variables-and-class-suffixes). |
| `.pf-m-light` | `.pf-v5-c-masthead` |  Modifies a masthead component to have a light theme with a background color of `--pf-v5-global--BackgroundColor--100`. |
| `.pf-m-light-200` | `.pf-v5-c-masthead` |  Modifies a masthead component to have a light theme with a background color of `--pf-v5-global--BackgroundColor--200`. |
