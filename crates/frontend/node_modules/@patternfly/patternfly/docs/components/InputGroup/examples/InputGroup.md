---
id: Input group
section: components
cssPrefix: pf-v5-c-input-group
---### Overview

Use the input group to extend form controls by adding text, buttons, selects, etc. The input group handles border overlap.

## Examples

### Variations

```html
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item">
    <button
      class="pf-v5-c-button pf-m-control"
      type="button"
      id="textAreaButton1"
    >Button</button>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control pf-m-resize-both">
      <textarea
        name="textarea1"
        id="textarea1"
        aria-label="Textarea with buttons"
        aria-describedby="textAreaButton1"
      ></textarea>
    </span>
  </div>
  <div class="pf-v5-c-input-group__item">
    <button class="pf-v5-c-button pf-m-control" type="button">Button</button>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control pf-m-resize-both">
      <textarea
        name="textarea2"
        id="textarea2"
        aria-label="Textarea with button"
        aria-describedby="textAreaButton2"
      ></textarea>
    </span>
  </div>
  <div class="pf-v5-c-input-group__item">
    <button
      class="pf-v5-c-button pf-m-control"
      type="button"
      id="textAreaButton2"
    >Button</button>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item">
    <button
      class="pf-v5-c-button pf-m-control"
      type="button"
      id="textAreaButton3"
    >Button</button>
  </div>
  <div class="pf-v5-c-input-group__item">
    <button class="pf-v5-c-button pf-m-control" type="button">Button</button>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control pf-m-resize-both">
      <textarea
        name="textarea3"
        id="textarea3"
        aria-label="Textarea with buttons"
        aria-describedby="textAreaButton3"
      ></textarea>
    </span>
  </div>
  <div class="pf-v5-c-input-group__item">
    <button class="pf-v5-c-button pf-m-control" type="button">Button</button>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item">
    <div class="pf-v5-c-select" style="width: 100px;">
      <span id="select-example-collapsed1-label" hidden>Choose one</span>

      <button
        class="pf-v5-c-select__toggle"
        type="button"
        id="select-example-collapsed1-toggle"
        aria-haspopup="true"
        aria-expanded="false"
        aria-labelledby="select-example-collapsed1-label select-example-collapsed1-toggle"
      >
        <div class="pf-v5-c-select__toggle-wrapper">
          <span class="pf-v5-c-select__toggle-text">Select</span>
        </div>
        <span class="pf-v5-c-select__toggle-arrow">
          <i class="fas fa-caret-down" aria-hidden="true"></i>
        </span>
      </button>

      <ul
        class="pf-v5-c-select__menu"
        role="listbox"
        aria-labelledby="select-example-collapsed1-label"
        hidden
        style="width: 100px;"
      >
        <li role="presentation">
          <button class="pf-v5-c-select__menu-item" role="option">Running</button>
        </li>
        <li role="presentation">
          <button
            class="pf-v5-c-select__menu-item pf-m-selected"
            role="option"
            aria-selected="true"
          >
            Stopped
            <span class="pf-v5-c-select__menu-item-icon">
              <i class="fas fa-check" aria-hidden="true"></i>
            </span>
          </button>
        </li>
        <li role="presentation">
          <button class="pf-v5-c-select__menu-item" role="option">Down</button>
        </li>
        <li role="presentation">
          <button class="pf-v5-c-select__menu-item" role="option">Degraded</button>
        </li>
        <li role="presentation">
          <button
            class="pf-v5-c-select__menu-item"
            role="option"
          >Needs maintenance</button>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control">
      <input
        type="text"
        id="textInput4"
        name="textInput4"
        aria-label="Input with select and button"
        aria-describedby="inputSelectButton1"
      />
    </span>
  </div>
  <div class="pf-v5-c-input-group__item">
    <button
      class="pf-v5-c-button pf-m-control"
      type="button"
      id="inputSelectButton1"
    >Button</button>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item pf-m-box">
    <span class="pf-v5-c-input-group__text">
      <i class="fas fa-dollar-sign" aria-hidden="true"></i>
    </span>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control">
      <input
        type="number"
        id="textInput5"
        name="textInput5"
        aria-label=" Dollar amount input example"
      />
    </span>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-box">
    <span class="pf-v5-c-input-group__text">.00</span>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control">
      <input
        type="email"
        id="textInput6"
        name="textInput6"
        aria-label="Email input field"
        aria-describedby="email-example"
      />
    </span>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-box">
    <span class="pf-v5-c-input-group__text" id="email-example">@example.com</span>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item pf-m-box">
    <span class="pf-v5-c-input-group__text">
      <i class="fas fa-at" aria-hidden="true"></i>
    </span>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control pf-m-required pf-m-error">
      <input
        required
        type="email"
        id="textInput7"
        name="textInput7"
        aria-invalid="true"
        aria-label="Error state username example"
      />
      <span class="pf-v5-c-form-control__utilities">
        <span class="pf-v5-c-form-control__icon pf-m-status">
          <i class="fas fa-exclamation-circle" aria-hidden="true"></i>
        </span>
      </span>
    </span>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control">
      <input
        type="text"
        id="textInput13"
        name="textInput13"
        aria-label="Input example with popover"
      />
    </span>
  </div>
  <div class="pf-v5-c-input-group__item">
    <button
      class="pf-v5-c-button pf-m-control"
      type="button"
      aria-label="Popover for input"
    >
      <i class="fas fa-question-circle" aria-hidden="true"></i>
    </button>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control">
      <input
        type="text"
        id="textInput12"
        name="textInput12"
        aria-label="Input example with popover"
      />
    </span>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-plain">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-label="Popover for input"
    >
      <i class="fas fa-question-circle" aria-hidden="true"></i>
    </button>
  </div>
</div>
<br />
<div class="pf-v5-c-input-group">
  <div class="pf-v5-c-input-group__item pf-m-fill">
    <span class="pf-v5-c-form-control">
      <input
        type="number"
        id="textInput14"
        name="textInput14"
        aria-label="Input example with plain unit"
      />
    </span>
  </div>
  <div class="pf-v5-c-input-group__item pf-m-box pf-m-plain">
    <span class="pf-v5-c-input-group__text">%</span>
  </div>
</div>

```

## Documentation

### Accessibility

When using the `.pf-v5-c-input-group` always ensure labels are used outside the input group with the `.pf-v5-screen-reader` class applied. You can also make use of the `aria-describedby`, `aria-label`, or `aria-labelledby` attributes. For more information on accessibility and forms see the [form component](/components/form).

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `aria-describedby` | `.pf-v5-c-form-control` |  When using `.pf-v5-c-input-group__text` or `.pf-v5-c-input-group__action` make use of this on the input field. |

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-input-group` | `<div>` |  Initiates the input group. **Required** |
| `.pf-v5-c-input-group__item` | `<div>` |  Initiates the input group item. |
| `.pf-v5-c-input-group__text` | `<span>` |  Initiates input group text. This should be used within `.pf-v5-c-input-group__item` to contain text. |
| `.pf-m-plain` | `.pf-v5-c-input-group__item` | Removes the border from the input group element. |
| `.pf-m-box` | `.pf-v5-c-input-group__item` | Adds appropriate styling for items that are not form controls. |
| `.pf-m-fill` | `.pf-v5-c-input-group__item` | Allows the input group element to stretch to fill available space. |
| `.pf-m-disabled` | `.pf-v5-c-input-group__item` | Adds disabled styling to match a disabled input within the input group. |
