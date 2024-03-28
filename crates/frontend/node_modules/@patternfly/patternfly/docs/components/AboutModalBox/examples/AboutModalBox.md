---
id: About modal
section: components
cssPrefix: pf-v5-c-about-modal-box
---## Examples

### Basic

```html isFullscreen
<div
  class="pf-v5-c-about-modal-box"
  style="--pf-v5-c-about-modal-box--BackgroundImage: url(/assets/images/pfbg-icon.svg)"
>
  <div class="pf-v5-c-about-modal-box__brand">
    <img
      class="pf-v5-c-about-modal-box__brand-image"
      src="/assets/images/pf_mini_logo_white.svg"
      alt="PatternFly brand logo"
    />
  </div>
  <div class="pf-v5-c-about-modal-box__close">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Close dialog"
    >
      <i class="fas fa-times" aria-hidden="true"></i>
    </button>
  </div>
  <div class="pf-v5-c-about-modal-box__header">
    <h1 class="pf-v5-c-title pf-m-4xl" id="about-modal-title">Product name</h1>
  </div>
  <div class="pf-v5-c-about-modal-box__content">
    <div class="pf-v5-c-about-modal-box__body">content</div>
    <p
      class="pf-v5-c-about-modal-box__strapline"
    >Trademark and copyright information here</p>
  </div>
</div>

```

## Documentation

In order to add a background image, set the `--pf-v5-c-about-modal-box--BackgroundImage` CSS variable to the path of the image. For example: `--pf-v5-c-about-modal-box--BackgroundImage: url(custom/path/image.jpg);`

### Accessibility

| Attribute | Applies to | Outcome |
| -- | -- | -- |
| `aria-label="Close Dialog"` | `.pf-v5-c-modal-box__close .pf-v5-c-button` | Provides an accessible name for the close button as it uses an icon instead of text. **Required** |

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-about-modal-box` |  `<div>`, `<article>`  |  Initiates a modal box. |
| `.pf-v5-c-about-modal-box__brand` |  `<div>` |  Initiates a modal box brand cell. |
| `.pf-v5-c-about-modal-box__brand-image` |  `<img>` |  Initiates a modal box brand image. |
| `.pf-v5-c-about-modal-box__close` |  `<div>` |  Initiates a modal box close cell. |
| `.pf-v5-c-about-modal-box__header` |  `<div>`, `<header>` |  Initiates a modal box header cell. |
| `.pf-v5-c-about-modal-box__content` |  `<div>` |  Initiates a modal box content cell. |
| `.pf-v5-c-about-modal-box__body` |  `<div>` |  Initiates a modal box body cell. |
| `.pf-v5-c-about-modal-box__strapline` |  `<p>` |  Initiates a modal box strapline cell. |
| `--pf-v5-c-about-modal-box--BackgroundImage` |  `.pf-v5-c-about-modal-box` |  Sets the background image for the about modal. |
