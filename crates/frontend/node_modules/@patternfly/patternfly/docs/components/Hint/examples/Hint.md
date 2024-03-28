---
id: 'Hint'
section: components
cssPrefix: pf-v5-c-hint
---## Examples

### Hint with title

```html
<div class="pf-v5-c-hint">
  <div class="pf-v5-c-hint__actions">
    <div class="pf-v5-c-dropdown">
      <button
        class="pf-v5-c-dropdown__toggle pf-m-plain"
        id="hint-with-title-dropdown-kebab-button"
        aria-expanded="false"
        type="button"
        aria-label="Actions"
      >
        <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
      </button>
      <ul
        class="pf-v5-c-dropdown__menu"
        aria-labelledby="hint-with-title-dropdown-kebab-button"
        hidden
        role="menu"
      >
        <li role="none">
          <a class="pf-v5-c-dropdown__menu-item" role="menuitem" href="#">Link</a>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            type="button"
          >Action</button>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-dropdown__menu-item pf-m-disabled"
            role="menuitem"
            href="#"
            aria-disabled="true"
            tabindex="-1"
          >Disabled link</a>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            type="button"
            disabled
          >Disabled action</button>
        </li>
        <li class="pf-v5-c-divider" role="separator"></li>
        <li role="none">
          <a
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            href="#"
          >Separated link</a>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-hint__title">Do more with Find it Fix it capabilities</div>
  <div
    class="pf-v5-c-hint__body"
  >Upgrade to Red Hat Smart Management to remediate all your systems across regions and geographies.</div>
</div>

<br />

<div class="pf-v5-c-hint">
  <div class="pf-v5-c-hint__actions">
    <div class="pf-v5-c-dropdown">
      <button
        class="pf-v5-c-dropdown__toggle pf-m-plain"
        id="hint-with-title-with-footer-dropdown-kebab-button"
        aria-expanded="false"
        type="button"
        aria-label="Actions"
      >
        <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
      </button>
      <ul
        class="pf-v5-c-dropdown__menu"
        aria-labelledby="hint-with-title-with-footer-dropdown-kebab-button"
        hidden
        role="menu"
      >
        <li role="none">
          <a class="pf-v5-c-dropdown__menu-item" role="menuitem" href="#">Link</a>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            type="button"
          >Action</button>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-dropdown__menu-item pf-m-disabled"
            role="menuitem"
            href="#"
            aria-disabled="true"
            tabindex="-1"
          >Disabled link</a>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            type="button"
            disabled
          >Disabled action</button>
        </li>
        <li class="pf-v5-c-divider" role="separator"></li>
        <li role="none">
          <a
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            href="#"
          >Separated link</a>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-hint__title">Do more with Find it Fix it capabilities</div>
  <div
    class="pf-v5-c-hint__body"
  >Upgrade to Red Hat Smart Management to remediate all your systems across regions and geographies.</div>
  <div class="pf-v5-c-hint__footer">
    <button
      class="pf-v5-c-button pf-m-link pf-m-inline"
      type="button"
    >Try it for 90 days</button>
  </div>
</div>

```

### Default with no title

```html
<div class="pf-v5-c-hint">
  <div class="pf-v5-c-hint__body">
    Welcome to the new documentation experience.
    <a href="#">Learn more about the improved features</a>.
  </div>
</div>

<br />

<div class="pf-v5-c-hint">
  <div class="pf-v5-c-hint__actions">
    <div class="pf-v5-c-dropdown">
      <button
        class="pf-v5-c-dropdown__toggle pf-m-plain"
        id="hint-with-no-title-dropdown-kebab-button"
        aria-expanded="false"
        type="button"
        aria-label="Actions"
      >
        <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
      </button>
      <ul
        class="pf-v5-c-dropdown__menu"
        aria-labelledby="hint-with-no-title-dropdown-kebab-button"
        hidden
        role="menu"
      >
        <li role="none">
          <a class="pf-v5-c-dropdown__menu-item" role="menuitem" href="#">Link</a>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            type="button"
          >Action</button>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-dropdown__menu-item pf-m-disabled"
            role="menuitem"
            href="#"
            aria-disabled="true"
            tabindex="-1"
          >Disabled link</a>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            type="button"
            disabled
          >Disabled action</button>
        </li>
        <li class="pf-v5-c-divider" role="separator"></li>
        <li role="none">
          <a
            class="pf-v5-c-dropdown__menu-item"
            role="menuitem"
            href="#"
          >Separated link</a>
        </li>
      </ul>
    </div>
  </div>
  <div
    class="pf-v5-c-hint__body"
  >Upgrade to Red Hat Smart Management to remediate all your systems across regions and geographies.</div>
  <div class="pf-v5-c-hint__footer">
    <button
      class="pf-v5-c-button pf-m-link pf-m-inline"
      type="button"
    >Try it for 90 days</button>
  </div>
</div>

```

## Documentation

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-hint` | `<div>` | Initiates the hint component. **Required** |
| `.pf-v5-c-hint__title` | `<div>` | Initiates the hint title element. |
| `.pf-v5-c-hint__body` | `<div>` | Initiates the hint body element. |
| `.pf-v5-c-hint__footer` | `<div>` | Initiates the hint footer element. |
| `.pf-v5-c-hint__actions` | `<div>` | Initiates the hint actions element. |
