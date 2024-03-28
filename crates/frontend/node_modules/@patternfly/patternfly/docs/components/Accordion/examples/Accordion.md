---
id: Accordion
section: components
cssPrefix: pf-v5-c-accordion
---## Examples

### Fluid

```html
<div class="pf-v5-c-accordion">
  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item one</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item two</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item three</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle pf-m-expanded"
      type="button"
      aria-expanded="true"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item four</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content pf-m-expanded">
    <div
      class="pf-v5-c-accordion__expandable-content-body"
    >Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis molestie lorem lacinia dolor aliquet faucibus. Suspendisse gravida imperdiet accumsan. Aenean auctor lorem justo, vitae tincidunt enim blandit vel. Aenean quis tempus dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item five</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>
</div>

```

### Fixed

```html
<div class="pf-v5-c-accordion">
  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item one</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content pf-m-fixed" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle pf-m-expanded"
      type="button"
      aria-expanded="true"
      id="accordion-fixed-item-two-toggle"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item two</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div
    class="pf-v5-c-accordion__expandable-content pf-m-expanded pf-m-fixed"
    role="region"
    tabindex="0"
    aria-labelledby="accordion-fixed-item-two-toggle"
  >
    <div
      class="pf-v5-c-accordion__expandable-content-body"
    >Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis molestie lorem lacinia dolor aliquet faucibus. Suspendisse gravida imperdiet accumsan. Aenean auctor lorem justo, vitae tincidunt enim blandit vel. Aenean quis tempus dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
    <div
      class="pf-v5-c-accordion__expandable-content-body"
    >Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis molestie lorem lacinia dolor aliquet faucibus. Suspendisse gravida imperdiet accumsan. Aenean auctor lorem justo, vitae tincidunt enim blandit vel. Aenean quis tempus dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
    <div
      class="pf-v5-c-accordion__expandable-content-body"
    >Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis molestie lorem lacinia dolor aliquet faucibus. Suspendisse gravida imperdiet accumsan. Aenean auctor lorem justo, vitae tincidunt enim blandit vel. Aenean quis tempus dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item three</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content pf-m-fixed" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item four</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content pf-m-fixed" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item five</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content pf-m-fixed" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>
</div>

```

### Definition list

```html
<dl class="pf-v5-c-accordion">
  <dt>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item one</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </dt>
  <dd class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </dd>

  <dt>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item two</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </dt>
  <dd class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </dd>

  <dt>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item three</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </dt>
  <dd class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </dd>

  <dt>
    <button
      class="pf-v5-c-accordion__toggle pf-m-expanded"
      type="button"
      aria-expanded="true"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item four</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </dt>
  <dd class="pf-v5-c-accordion__expandable-content pf-m-expanded">
    <div
      class="pf-v5-c-accordion__expandable-content-body"
    >Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis molestie lorem lacinia dolor aliquet faucibus. Suspendisse gravida imperdiet accumsan. Aenean auctor lorem justo, vitae tincidunt enim blandit vel. Aenean quis tempus dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
  </dd>

  <dt>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item five</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </dt>
  <dd class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </dd>
</dl>

```

### Bordered

```html
<div class="pf-v5-c-accordion pf-m-bordered">
  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item one</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle pf-m-expanded"
      type="button"
      aria-expanded="true"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item two</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content pf-m-expanded">
    <div class="pf-v5-c-accordion__expandable-content-body">
      <a href="#">Lorem ipsum</a> dolor sit amet, consectetur adipiscing elit. Duis molestie lorem lacinia dolor aliquet faucibus. Suspendisse gravida imperdiet accumsan. Aenean auctor lorem justo, vitae tincidunt enim blandit vel. Aenean quis tempus dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    </div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item three</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item four</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item five</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>
</div>

```

### Large bordered

```html
<div class="pf-v5-c-accordion pf-m-display-lg pf-m-bordered">
  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item one</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle pf-m-expanded"
      type="button"
      aria-expanded="true"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item two</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content pf-m-expanded">
    <div
      class="pf-v5-c-accordion__expandable-content-body"
    >Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis molestie lorem lacinia dolor aliquet faucibus. Suspendisse gravida imperdiet accumsan. Aenean auctor lorem justo, vitae tincidunt enim blandit vel. Aenean quis tempus dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item three</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle pf-m-expanded"
      type="button"
      aria-expanded="true"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item four</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content pf-m-expanded">
    <div
      class="pf-v5-c-accordion__expandable-content-body"
    >Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis molestie lorem lacinia dolor aliquet faucibus. Suspendisse gravida imperdiet accumsan. Aenean auctor lorem justo, vitae tincidunt enim blandit vel. Aenean quis tempus dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
    <div class="pf-v5-c-accordion__expandable-content-body">
      <button
        class="pf-v5-c-button pf-m-link pf-m-inline pf-m-display-lg"
        type="button"
      >
        Call to action
        <span class="pf-v5-c-button__icon pf-m-end">
          <i class="fas fa-arrow-right" aria-hidden="true"></i>
        </span>
      </button>
    </div>
  </div>

  <h3>
    <button
      class="pf-v5-c-accordion__toggle"
      type="button"
      aria-expanded="false"
    >
      <span class="pf-v5-c-accordion__toggle-text">Item five</span>

      <span class="pf-v5-c-accordion__toggle-icon">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
    </button>
  </h3>
  <div class="pf-v5-c-accordion__expandable-content" hidden>
    <div class="pf-v5-c-accordion__expandable-content-body">This text is hidden</div>
  </div>
</div>

```

## Documentation

### Overview

There are two variations to build the accordion component:
One way uses `<div>` and `<h1 - h6>` tags to build the component.
In these examples `.pf-v5-c-accordion` uses `<div>`, `.pf-v5-c-accordion__toggle` uses `<h3><button>`, and `.pf-v5-c-accordion__expandable-content` uses `<div>`. The heading level that you use should fit within the rest of the headings outlined on your page.

Another variation is using the definition list:
In these examples `.pf-v5-c-accordion` uses `<dl>`, `.pf-v5-c-accordion__toggle` uses `<dt><button>`, and `.pf-v5-c-accordion__expandable-content` uses `<dd>`.

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-accordion` | `<div>`, `<dl>` | Initiates an accordion component. **Required**|
| `.pf-v5-c-accordion__toggle` | `<h1-h6><button>`, `<dt><button>` | Initiates a toggle in the accordion. **Required** |
| `.pf-v5-c-accordion__toggle-text` | `<span>` | Initiates the text inside the toggle. **Required** |
| `.pf-v5-c-accordion__toggle-icon` | `<span>` | Initiates the toggle icon wrapper. **Required** |
| `.pf-v5-c-accordion__expandable-content` | `<div>`, `<dd>` | Initiates expandable content. **Must be paired with a button** |
| `.pf-v5-c-accordion__expandable-content-body` | `<div>` | Initiates expandable content body. **Required** |
| `.pf-m-bordered` | `.pf-v5-c-accordion` | Modifies the accordion to add borders between items. |
| `.pf-m-display-lg` | `.pf-v5-c-accordion` | Modifies the accordion for large display styling. This variation is for marketing/web use cases. |
| `.pf-m-expanded` | `.pf-v5-c-accordion__toggle`, `.pf-v5-c-accordion__expandable-content` | Modifies the accordion button and expandable content for the expanded state. |
| `.pf-m-fixed` | `.pf-v5-c-accordion__expandable-content` | Modifies the expandable content for the fixed state. |
