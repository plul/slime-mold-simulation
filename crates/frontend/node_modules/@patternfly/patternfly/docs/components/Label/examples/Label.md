---
id: Label
section: components
cssPrefix: ['pf-v5-c-label','pf-v5-c-label-group']
---import './Label.css'

## Examples

### Filled

```html
<span class="pf-v5-c-label">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Grey</span>
  </span>
</span>

<span class="pf-v5-c-label">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Grey icon</span>
  </span>
</span>

<span class="pf-v5-c-label">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Grey removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="default-removable-button"
      aria-label="Remove"
      aria-labelledby="default-removable-button default-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Grey icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="default-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="default-icon-removable-button default-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Grey link</span>
  </a>
</span>

<span class="pf-v5-c-label">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Grey link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="default-link-removable-button"
      aria-label="Remove"
      aria-labelledby="default-link-removable-button default-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Grey label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Grey label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="default-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="default-truncated-with-icon-button default-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Blue</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Blue icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Blue removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="blue-removable-button"
      aria-label="Remove"
      aria-labelledby="blue-removable-button blue-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Blue icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="blue-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="blue-icon-removable-button blue-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-blue">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Blue link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-blue">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Blue link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="blue-link-removable-button"
      aria-label="Remove"
      aria-labelledby="blue-link-removable-button blue-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Blue label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Blue label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="blue-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="blue-truncated-with-icon-button blue-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Green</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Green icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Green removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="green-removable-button"
      aria-label="Remove"
      aria-labelledby="green-removable-button green-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Green icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="green-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="green-icon-removable-button green-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-green">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Green link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-green">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Green link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="green-link-removable-button"
      aria-label="Remove"
      aria-labelledby="green-link-removable-button green-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Green label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Green label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="green-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="green-truncated-with-icon-button green-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Orange</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Orange icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Orange removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="orange-removable-button"
      aria-label="Remove"
      aria-labelledby="orange-removable-button orange-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Orange icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="orange-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="orange-icon-removable-button orange-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-orange">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Orange link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-orange">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Orange link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="orange-link-removable-button"
      aria-label="Remove"
      aria-labelledby="orange-link-removable-button orange-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Orange label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Orange label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="orange-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="orange-truncated-with-icon-button orange-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Red</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Red icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Red removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="red-removable-button"
      aria-label="Remove"
      aria-labelledby="red-removable-button red-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Red icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="red-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="red-icon-removable-button red-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-red">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Red link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-red">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Red link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="red-link-removable-button"
      aria-label="Remove"
      aria-labelledby="red-link-removable-button red-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Red label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Red label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="red-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="red-truncated-with-icon-button red-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Purple</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Purple icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Purple removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="purple-removable-button"
      aria-label="Remove"
      aria-labelledby="purple-removable-button purple-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Purple icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="purple-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="purple-icon-removable-button purple-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-purple">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Purple link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-purple">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Purple link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="purple-link-removable-button"
      aria-label="Remove"
      aria-labelledby="purple-link-removable-button purple-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Purple label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Purple label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="purple-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="purple-truncated-with-icon-button purple-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Cyan</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Cyan icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Cyan removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="cyan-removable-button"
      aria-label="Remove"
      aria-labelledby="cyan-removable-button cyan-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Cyan icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="cyan-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="cyan-icon-removable-button cyan-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-cyan">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Cyan link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-cyan">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Cyan link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="cyan-link-removable-button"
      aria-label="Remove"
      aria-labelledby="cyan-link-removable-button cyan-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Cyan label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Cyan label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="cyan-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="cyan-truncated-with-icon-button cyan-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />
<span class="pf-v5-c-label pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Gold</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Gold icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Gold removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="gold-removable-button"
      aria-label="Remove"
      aria-labelledby="gold-removable-button gold-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Gold icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="gold-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="gold-icon-removable-button gold-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-gold">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Gold link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-gold">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Gold link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="gold-link-removable-button"
      aria-label="Remove"
      aria-labelledby="gold-link-removable-button gold-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Gold label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Gold label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="gold-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="gold-truncated-with-icon-button gold-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

```

### Outline

```html
<span class="pf-v5-c-label pf-m-outline">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Grey</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Grey icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Grey removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="grey-outline-removable-button"
      aria-label="Remove"
      aria-labelledby="grey-outline-removable-button grey-outline-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Grey icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="grey-outline-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="grey-outline-icon-removable-button grey-outline-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Grey link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-outline">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Grey link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="grey-outline-link-removable-button"
      aria-label="Remove"
      aria-labelledby="grey-outline-link-removable-button grey-outline-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Grey label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Grey label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="grey-outline-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="grey-outline-truncated-with-icon-button grey-outline-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-outline pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Blue</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Blue icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Blue removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="blue-outline-removable-button"
      aria-label="Remove"
      aria-labelledby="blue-outline-removable-button blue-outline-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Blue icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="blue-outline-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="blue-outline-icon-removable-button blue-outline-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-blue">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Blue link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-blue">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Blue link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="blue-outline-link-removable-button"
      aria-label="Remove"
      aria-labelledby="blue-outline-link-removable-button blue-outline-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Blue label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-blue">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Blue label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="blue-outline-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="blue-outline-truncated-with-icon-button blue-outline-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-outline pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Green</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Green icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Green removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="green-outline-removable-button"
      aria-label="Remove"
      aria-labelledby="green-outline-removable-button green-outline-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Green icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="green-outline-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="green-outline-icon-removable-button green-outline-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-green">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Green link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-green">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Green link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="green-outline-link-removable-button"
      aria-label="Remove"
      aria-labelledby="green-outline-link-removable-button green-outline-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Green label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-green">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Green label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="green-outline-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="green-outline-truncated-with-icon-button green-outline-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-outline pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Orange</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Orange icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Orange removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="orange-outline-removable-button"
      aria-label="Remove"
      aria-labelledby="orange-outline-removable-button orange-outline-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Orange icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="orange-outline-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="orange-outline-icon-removable-button orange-outline-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-orange">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Orange link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-orange">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Orange link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="orange-outline-link-removable-button"
      aria-label="Remove"
      aria-labelledby="orange-outline-link-removable-button orange-outline-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Orange label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-orange">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Orange label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="orange-outline-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="orange-outline-truncated-with-icon-button orange-outline-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-outline pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Red</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Red icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Red removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="red-outline-removable-button"
      aria-label="Remove"
      aria-labelledby="red-outline-removable-button red-outline-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Red icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="red-outline-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="red-outline-icon-removable-button red-outline-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-red">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Red link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-red">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Red link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="red-outline-link-removable-button"
      aria-label="Remove"
      aria-labelledby="red-outline-link-removable-button red-outline-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Red label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-red">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Red label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="red-outline-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="red-outline-truncated-with-icon-button red-outline-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-outline pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Purple</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Purple icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Purple removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="purple-outline-removable-button"
      aria-label="Remove"
      aria-labelledby="purple-outline-removable-button purple-outline-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Purple icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="purple-outline-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="purple-outline-icon-removable-button purple-outline-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-purple">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Purple link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-purple">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Purple link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="purple-outline-link-removable-button"
      aria-label="Remove"
      aria-labelledby="purple-outline-link-removable-button purple-outline-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Purple label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-purple">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Purple label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="purple-outline-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="purple-outline-truncated-with-icon-button purple-outline-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-outline pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Cyan</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Cyan icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Cyan removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="cyan-outline-removable-button"
      aria-label="Remove"
      aria-labelledby="cyan-outline-removable-button cyan-outline-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Cyan icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="cyan-outline-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="cyan-outline-icon-removable-button cyan-outline-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-cyan">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Cyan link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-cyan">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Cyan link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="cyan-outline-link-removable-button"
      aria-label="Remove"
      aria-labelledby="cyan-outline-link-removable-button cyan-outline-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Cyan label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-cyan">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Cyan label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="cyan-outline-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="cyan-outline-truncated-with-icon-button cyan-outline-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<br />
<br />

<span class="pf-v5-c-label pf-m-outline pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Gold</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Gold icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Gold removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="gold-outline-removable-button"
      aria-label="Remove"
      aria-labelledby="gold-outline-removable-button gold-outline-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Gold icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="gold-outline-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="gold-outline-icon-removable-button gold-outline-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-gold">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Gold link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-gold">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Gold link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="gold-outline-link-removable-button"
      aria-label="Remove"
      aria-labelledby="gold-outline-link-removable-button gold-outline-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Gold label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-outline pf-m-gold">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Gold label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="gold-outline-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="gold-outline-truncated-with-icon-button gold-outline-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

```

### Compact

```html
<span class="pf-v5-c-label pf-m-compact">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Compact</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-compact">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Compact icon</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-compact">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Compact removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="compact-removable-button"
      aria-label="Remove"
      aria-labelledby="compact-removable-button compact-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-compact">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span class="pf-v5-c-label__text">Compact icon removable</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="compact-icon-removable-button"
      aria-label="Remove"
      aria-labelledby="compact-icon-removable-button compact-icon-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-compact">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Compact link</span>
  </a>
</span>

<span class="pf-v5-c-label pf-m-compact">
  <a class="pf-v5-c-label__content" href="#">
    <span class="pf-v5-c-label__text">Compact link removable</span>
  </a>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="compact-link-removable-button"
      aria-label="Remove"
      aria-labelledby="compact-link-removable-button compact-link-removable-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-compact">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 28ch"
    >Compact label, max-width truncation customization</span>
  </span>
</span>

<span class="pf-v5-c-label pf-m-compact">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__icon">
      <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
    </span>
    <span
      class="pf-v5-c-label__text"
      style="--pf-v5-c-label__text--MaxWidth: 38ch"
    >Compact label with icon and set max-width truncation customization</span>
  </span>
  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="compact-truncated-with-icon-button"
      aria-label="Remove"
      aria-labelledby="compact-truncated-with-icon-button compact-truncated-with-icon-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

```

### Overflow

This style of label is used to indicate overflow within a label group.

```html
<button class="pf-v5-c-label pf-m-overflow" type="button">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Overflow</span>
  </span>
</button>

```

### Editable

**Note: Editable label behavior must be handled with JavaScript.**

*   `.pf-v5-c-label__editable-text` onClick event should:
    *   Set `.pf-m-editable-active` on `.pf-v5-c-label`
    *   Change `.pf-v5-c-label__editable-text`from a button to an input
*   Return keypress, when content is editable, should:
    *   Be captured to prevent line wrapping and save updates to label text
    *   Remove `.pf-m-editable-active` from `.pf-v5-c-label`
*   Esc keypress, when content is editable, should:
    *   Undo any update to label text
    *   Remove `.pf-m-editable-active` from `.pf-v5-c-label`
    *   Change `.pf-v5-c-label__editable-text` back to a button

```html isBeta
<span class="pf-v5-c-label pf-m-editable pf-m-blue">
  <button
    class="pf-v5-c-label__content"
    id="editable-label-editable-content"
    aria-label="Editable text"
  >
    <span class="pf-v5-c-label__text">Editable label</span>
  </button>

  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="editable-label-button"
      aria-label="Remove"
      aria-labelledby="editable-label-button editable-label-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span class="pf-v5-c-label pf-m-editable pf-m-editable-active pf-m-blue">
  <input
    class="pf-v5-c-label__content"
    id="editable-label-active-editable-content"
    type="text"
    value="Editable active"
    aria-label="Editable text"
  />
</span>

<span class="pf-v5-c-label pf-m-compact pf-m-editable pf-m-blue">
  <button
    class="pf-v5-c-label__content"
    id="compact-editable-label-editable-content"
    aria-label="Editable text"
  >
    <span class="pf-v5-c-label__text">Compact editable label</span>
  </button>

  <span class="pf-v5-c-label__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      id="compact-editable-label-button"
      aria-label="Remove"
      aria-labelledby="compact-editable-label-button compact-editable-label-text"
    >
      <i class="fas fa-times fa-fw" aria-hidden="true"></i>
    </button>
  </span>
</span>

<span
  class="pf-v5-c-label pf-m-compact pf-m-editable pf-m-editable-active pf-m-blue"
>
  <input
    class="pf-v5-c-label__content"
    id="compact-editable-label-active-editable-content"
    type="text"
    value="Compact editable active"
    aria-label="Editable text"
  />
</span>

```

### Add label

This style of label is used to add new labels to a label group.

```html isBeta
<button class="pf-v5-c-label pf-m-add" type="button">
  <span class="pf-v5-c-label__content">
    <span class="pf-v5-c-label__text">Add Label</span>
  </span>
</button>

```

### Basic label group

Use a label group to display multiple labels at once.

```html
<div class="pf-v5-c-label-group">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
    </ul>
  </div>
</div>

```

### Label group with overflow

```html
<div class="pf-v5-c-label-group">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <button class="pf-v5-c-label pf-m-overflow pf-m-blue" type="button">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">3 more</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Label group with overflow expanded

```html
<div class="pf-v5-c-label-group">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-cyan">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 4</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-purple">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 5</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <button class="pf-v5-c-label pf-m-overflow" type="button">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">3 less</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Editable label group with add button

The contents of a label group can be modified by removing labels or adding new ones using the Add button.

```html
<div class="pf-v5-c-label-group">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <button class="pf-v5-c-label pf-m-add" type="button">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Add label</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Label group with category label

```html
<div class="pf-v5-c-label-group pf-m-category">
  <div class="pf-v5-c-label-group__main">
    <span
      class="pf-v5-c-label-group__label"
      aria-hidden="true"
      id="label-group-category-label"
    >Group label</span>
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-labelledby="label-group-category-label"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
    </ul>
  </div>
</div>

```

### Label group with removable categories

```html
<div class="pf-v5-c-label-group pf-m-category">
  <div class="pf-v5-c-label-group__main">
    <span
      class="pf-v5-c-label-group__label"
      aria-hidden="true"
      id="label-group-category-removable-label"
    >Group label</span>
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-labelledby="label-group-category-removable-label"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-cyan">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 4</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-orange">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 5</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-red">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 6</span>
          </span>
        </span>
      </li>
    </ul>
  </div>
  <div class="pf-v5-c-label-group__close">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-labelledby="label-group-category-removable-button label-group-category-removable-label"
      aria-label="Close label group"
      id="label-group-category-removable-button"
    >
      <i class="fas fa-times-circle" aria-hidden="true"></i>
    </button>
  </div>
</div>

```

### Vertical label group

```html
<div class="pf-v5-c-label-group pf-m-vertical">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
    </ul>
  </div>
</div>

```

### Vertical label group with overflow

```html
<div class="pf-v5-c-label-group pf-m-vertical">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <button class="pf-v5-c-label pf-m-overflow" type="button">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">3 more</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Vertical label group with overflow expanded

```html
<div class="pf-v5-c-label-group pf-m-vertical">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-cyan">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 4</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-purple">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 5</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <button class="pf-v5-c-label pf-m-overflow" type="button">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">3 less</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Vertical label group with category

```html
<div class="pf-v5-c-label-group pf-m-vertical pf-m-category">
  <div class="pf-v5-c-label-group__main">
    <span
      class="pf-v5-c-label-group__label"
      aria-hidden="true"
      id="label-group-vertical-category-label"
    >Group label</span>
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-labelledby="label-group-vertical-category-label"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
    </ul>
  </div>
</div>

```

### Vertical label group with removable category

```html
<div class="pf-v5-c-label-group pf-m-vertical pf-m-category">
  <div class="pf-v5-c-label-group__main">
    <span
      class="pf-v5-c-label-group__label"
      aria-hidden="true"
      id="label-group-vertical-category-removable-label"
    >Group label</span>
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-labelledby="label-group-vertical-category-removable-label"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
    </ul>
  </div>
  <div class="pf-v5-c-label-group__close">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-labelledby="label-group-vertical-category-removable-button label-group-vertical-category-removable-label"
      aria-label="Close label group"
      id="label-group-vertical-category-removable-button"
    >
      <i class="fas fa-times-circle" aria-hidden="true"></i>
    </button>
  </div>
</div>

```

In addition to the JavaScript management of [editable labels](/components/label#editable), dynamic label groups also need:

*   `.pf-v5-c-label-group.pf-m-editable` onClick event should (excluding labels within) set focus on `.pf-v5-c-label-group__textarea`

### Editable labels, dynamic label group

```html isBeta
<div class="pf-v5-c-label-group pf-m-editable">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-editable pf-m-blue">
          <button
            class="pf-v5-c-label__content"
            id="editable-labels-editable-group-example-editable-label-editable-1-editable-content"
            aria-label="Editable text"
          >
            <span class="pf-v5-c-label__text">Editable label 1</span>
          </button>

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="editable-labels-editable-group-example-editable-label-editable-1-button"
              aria-label="Remove"
              aria-labelledby="editable-labels-editable-group-example-editable-label-editable-1-button editable-labels-editable-group-example-editable-label-editable-1-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-editable pf-m-blue">
          <button
            class="pf-v5-c-label__content"
            id="editable-labels-editable-group-example-editable-label-editable-2-editable-content"
            aria-label="Editable text"
          >
            <span class="pf-v5-c-label__text">Editable label 2</span>
          </button>

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="editable-labels-editable-group-example-editable-label-editable-2-button"
              aria-label="Remove"
              aria-labelledby="editable-labels-editable-group-example-editable-label-editable-2-button editable-labels-editable-group-example-editable-label-editable-2-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-editable pf-m-blue">
          <button
            class="pf-v5-c-label__content"
            id="editable-labels-editable-group-example-editable-label-editable-3-editable-content"
            aria-label="Editable text"
          >
            <span class="pf-v5-c-label__text">Editable label 3</span>
          </button>

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="editable-labels-editable-group-example-editable-label-editable-3-button"
              aria-label="Remove"
              aria-labelledby="editable-labels-editable-group-example-editable-label-editable-3-button editable-labels-editable-group-example-editable-label-editable-3-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item pf-m-textarea">
        <textarea
          class="pf-v5-c-label-group__textarea"
          rows="1"
          tabindex="0"
          aria-label="New label"
        ></textarea>
      </li>
    </ul>
  </div>
</div>

```

### Editable labels, label active, dynamic label group

```html isBeta
<div class="pf-v5-c-label-group pf-m-editable">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-editable pf-m-blue">
          <button
            class="pf-v5-c-label__content"
            id="editable-labels-label-active-editable-group-example-editable-label-editable-1-editable-content"
            aria-label="Editable text"
          >
            <span class="pf-v5-c-label__text">Editable label 1</span>
          </button>

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="editable-labels-label-active-editable-group-example-editable-label-editable-1-button"
              aria-label="Remove"
              aria-labelledby="editable-labels-label-active-editable-group-example-editable-label-editable-1-button editable-labels-label-active-editable-group-example-editable-label-editable-1-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-editable pf-m-blue">
          <button
            class="pf-v5-c-label__content"
            id="editable-labels-label-active-editable-group-example-editable-label-editable-2-editable-content"
            aria-label="Editable text"
          >
            <span class="pf-v5-c-label__text">Editable label 2</span>
          </button>

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="editable-labels-label-active-editable-group-example-editable-label-editable-2-button"
              aria-label="Remove"
              aria-labelledby="editable-labels-label-active-editable-group-example-editable-label-editable-2-button editable-labels-label-active-editable-group-example-editable-label-editable-2-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span
          class="pf-v5-c-label pf-m-editable pf-m-editable-active pf-m-blue"
        >
          <input
            class="pf-v5-c-label__content"
            id="editable-labels-label-active-editable-group-example-editable-label-editable-3-editable-content"
            type="text"
            value="Editable label 3, active"
            aria-label="Editable text"
          />

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="editable-labels-label-active-editable-group-example-editable-label-editable-3-button"
              aria-label="Remove"
              aria-labelledby="editable-labels-label-active-editable-group-example-editable-label-editable-3-button editable-labels-label-active-editable-group-example-editable-label-editable-3-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item pf-m-textarea">
        <textarea
          class="pf-v5-c-label-group__textarea"
          rows="1"
          tabindex="0"
          aria-label="New label"
        ></textarea>
      </li>
    </ul>
  </div>
</div>

```

### Static labels, dynamic label group

```html
<div class="pf-v5-c-label-group pf-m-editable">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Static label 1</span>
          </span>
          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="static-labels-dynamic-label-group-example-editable-label-static-1-button"
              aria-label="Remove"
              aria-labelledby="static-labels-dynamic-label-group-example-editable-label-static-1-button static-labels-dynamic-label-group-example-editable-label-static-1-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Static label 2</span>
          </span>
          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="static-labels-dynamic-label-group-example-editable-label-static-2-button"
              aria-label="Remove"
              aria-labelledby="static-labels-dynamic-label-group-example-editable-label-static-2-button static-labels-dynamic-label-group-example-editable-label-static-2-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Static label 3</span>
          </span>
          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="static-labels-dynamic-label-group-example-editable-label-static-3-button"
              aria-label="Remove"
              aria-labelledby="static-labels-dynamic-label-group-example-editable-label-static-3-button static-labels-dynamic-label-group-example-editable-label-static-3-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item pf-m-textarea">
        <textarea
          class="pf-v5-c-label-group__textarea"
          rows="1"
          tabindex="0"
          aria-label="New label"
        ></textarea>
      </li>
    </ul>
  </div>
</div>

```

### Mixed labels (static / editable), dynamic label group

```html isBeta
<div class="pf-v5-c-label-group pf-m-editable">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Static label 1</span>
          </span>
          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="mixed-labels-dynamic-label-group-example-editable-label-static-1-button"
              aria-label="Remove"
              aria-labelledby="mixed-labels-dynamic-label-group-example-editable-label-static-1-button mixed-labels-dynamic-label-group-example-editable-label-static-1-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">Static label 2</span>
          </span>
          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="mixed-labels-dynamic-label-group-example-editable-label-static-2-button"
              aria-label="Remove"
              aria-labelledby="mixed-labels-dynamic-label-group-example-editable-label-static-2-button mixed-labels-dynamic-label-group-example-editable-label-static-2-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-editable pf-m-blue">
          <button
            class="pf-v5-c-label__content"
            id="mixed-labels-dynamic-label-group-example-editable-label-dynamic-1-editable-content"
            aria-label="Editable text"
          >
            <span class="pf-v5-c-label__text">Dynamic, editable label 1</span>
          </button>

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="mixed-labels-dynamic-label-group-example-editable-label-dynamic-1-button"
              aria-label="Remove"
              aria-labelledby="mixed-labels-dynamic-label-group-example-editable-label-dynamic-1-button mixed-labels-dynamic-label-group-example-editable-label-dynamic-1-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-editable pf-m-blue">
          <button
            class="pf-v5-c-label__content"
            id="mixed-labels-dynamic-label-group-example-editable-label-dynamic-2-editable-content"
            aria-label="Editable text"
          >
            <span class="pf-v5-c-label__text">Dynamic, editable label 2</span>
          </button>

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="mixed-labels-dynamic-label-group-example-editable-label-dynamic-2-button"
              aria-label="Remove"
              aria-labelledby="mixed-labels-dynamic-label-group-example-editable-label-dynamic-2-button mixed-labels-dynamic-label-group-example-editable-label-dynamic-2-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-editable pf-m-blue">
          <button
            class="pf-v5-c-label__content"
            id="mixed-labels-dynamic-label-group-example-editable-label-dynamic-3-editable-content"
            aria-label="Editable text"
          >
            <span class="pf-v5-c-label__text">Dynamic, editable label 3</span>
          </button>

          <span class="pf-v5-c-label__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              id="mixed-labels-dynamic-label-group-example-editable-label-dynamic-3-button"
              aria-label="Remove"
              aria-labelledby="mixed-labels-dynamic-label-group-example-editable-label-dynamic-3-button mixed-labels-dynamic-label-group-example-editable-label-dynamic-3-text"
            >
              <i class="fas fa-times fa-fw" aria-hidden="true"></i>
            </button>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item pf-m-textarea">
        <textarea
          class="pf-v5-c-label-group__textarea"
          rows="1"
          tabindex="0"
          aria-label="New label"
        ></textarea>
      </li>
    </ul>
  </div>
</div>

```

### Label group with compact labels

```html
<div class="pf-v5-c-label-group">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
    </ul>
  </div>
</div>

```

### Label group with compact labels and overflow

```html
<div class="pf-v5-c-label-group">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <button class="pf-v5-c-label pf-m-compact pf-m-overflow" type="button">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__text">3 more</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Vertical label group with compact labels

```html
<div class="pf-v5-c-label-group pf-m-vertical">
  <div class="pf-v5-c-label-group__main">
    <ul
      class="pf-v5-c-label-group__list"
      role="list"
      aria-label="Group of labels"
    >
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 1</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact pf-m-blue">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 2</span>
          </span>
        </span>
      </li>
      <li class="pf-v5-c-label-group__list-item">
        <span class="pf-v5-c-label pf-m-compact pf-m-green">
          <span class="pf-v5-c-label__content">
            <span class="pf-v5-c-label__icon">
              <i class="fas fa-fw fa-info-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-label__text">Label 3</span>
          </span>
        </span>
      </li>
    </ul>
  </div>
</div>

```

## Documentation

### Label usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-label` | `<span>`, `<button>` | Initiates a label. Without a color modifier, the label's default style is grey. Use a color modifier to change the label color. Use a `<button>` if the label is an overflow label used in the label group. **Required** |
| `.pf-v5-c-label__content` | `<span>`, `<a>`, `<button>` | Creates a content wrapper. Use as an `<a>` element if the label serves as a link. Use a `<button>` if the label serves as an action. **Required** |
| `.pf-v5-c-label__icon` | `<span>` | Initiates a label icon. |
| `.pf-v5-c-label__text` | `<span>` | Initiates label text. **Required** |
| `.pf-v5-c-label__editable-text` | `<button>`, `<input>` | Initiates editable label text. See the [editable](#editable) example for details about handling behavior in Javascript.|
| `.pf-v5-c-label__actions` | `<span>` | Creates a wrapper for label actions. **Required for removable labels** |
| `.pf-m-outline` | `.pf-v5-c-label` | Modifies label for outline styles. |
| `.pf-m-compact` | `.pf-v5-c-label` | Modifies label for compact styles. |
| `.pf-m-overflow` | `.pf-v5-c-label` | Modifies label for overflow styles for use in a label group. |
| `.pf-m-add` | `.pf-v5-c-label` | Modifies label for add styles for use in a label group. |
| `.pf-m-blue` | `.pf-v5-c-label` | Modifies the label to have blue colored styling. |
| `.pf-m-green` | `.pf-v5-c-label` | Modifies the label to have green colored styling. |
| `.pf-m-orange` | `.pf-v5-c-label` | Modifies the label to have orange colored styling. |
| `.pf-m-red` | `.pf-v5-c-label` | Modifies the label to have red colored styling. |
| `.pf-m-purple` | `.pf-v5-c-label` | Modifies the label to have purple colored styling. |
| `.pf-m-cyan` | `.pf-v5-c-label` | Modifies the label to have cyan colored styling. |
| `.pf-m-gold` | `.pf-v5-c-label` | Modifies the label to have gold colored styling. |
| `.pf-m-editable` | `.pf-v5-c-label` | Modifies label for editable styles. |
| `.pf-m-editable-active` | `.pf-v5-c-label.pf-m-editable` | Modifies editable label for active styles. |
| `--pf-v5-c-label__text--MaxWidth` | `.pf-v5-c-label` | Modifiex the max width of the text before text will truncate. |

### Label group accessibility

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `role="list"` | `.pf-v5-c-label-group__list` | Indicates that the label group list is a list element. This role is redundant since `.pf-v5-c-label-group__list` is a `<ul>` but is required for screen readers to announce the list properly. **Required** |
| `aria-label="[button label text]"` | `.pf-v5-c-label-group__close > button` |  Provides an accessible name for a label group close button when an icon is used instead of text. Required when an icon is used with no supporting text. **Required** |
| `aria-labelledby="[id value of .pf-v5-c-label-group__close > button] [id value of .pf-v5-c-label-group__label]"` | `.pf-v5-c-label-group__close > button` | Provides an accessible name for the button. **Required** |
| `aria-label="[label text]"` | `.pf-v5-c-label-group__textarea` | Provides an accessible name for the textarea. **Required** |
| `row="1"` | `.pf-v5-c-label-group__textarea` | Indicates that the label group textarea is one row. **Required** |
| `tabindex="0"` | `.pf-v5-c-label-group__textarea` | Inserts the label group textarea into the tab order of the page so that it is focusable. **Required** |

### Label group usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-label-group` | `<div>` | Initiates the label group component. **Required.** |
| `.pf-v5-c-label-group__list` | `<ul>` | Initiates the container for a list of labels. **Required.** |
| `.pf-v5-c-label-group__list-item` | `<li>` | Initiates the list item inside of the label group. **Required.** |
| `.pf-v5-c-label-group__main` | `<div>` | Initiates the main element in the label group. **Required when label and list are present** |
| `.pf-v5-c-label-group__textarea` | `<textarea>` | Initiates the textarea element in the label group. **Required when label group is editable** |
| `.pf-v5-c-label-group__label` | `<span>` | Initiates the label to be used in the label group. |
| `.pf-v5-c-label-group__close` | `<div>` | Initiates the container used for the button to remove the label group. |
| `.pf-v5-c-button` | `.pf-v5-c-label-group__close <button>` | Initiates the button used to remove the label group. |
| `.pf-m-editable` | `.pf-v5-c-label-group` | Modifies the label group to support editable styling. |
| `.pf-m-category` | `.pf-v5-c-label-group` | Modifies the label group to support category styling. |
| `.pf-m-textarea` | `.pf-v5-c-label-group__list-item` | Modifies the label group list item to support textarea. |
