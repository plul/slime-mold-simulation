---
id: 'Slider'
section: components
cssPrefix: pf-v5-c-slider
---## Examples

### Discrete

```html
<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 62.5%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div class="pf-v5-c-slider__steps" aria-hidden="true">
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 0%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">0</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 12.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 25%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">2</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 37.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 50%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">4</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 62.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 75%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">6</div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 87.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 100%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">8</div>
      </div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="8"
      aria-valuenow="5"
      aria-label="Value"
      tabindex="0"
    ></div>
  </div>
</div>

```

### Continuous

```html
<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      tabindex="0"
    ></div>
  </div>
</div>

<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div class="pf-v5-c-slider__steps" aria-hidden="true">
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 0%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">0%</div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 100%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">100%</div>
      </div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      tabindex="0"
    ></div>
  </div>
</div>

```

### Value input

```html
<div
  class="pf-v5-c-slider"
  style="--pf-v5-c-slider--value: 62.5%; --pf-v5-c-slider__value--c-form-control--width-chars: 1;"
>
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div class="pf-v5-c-slider__steps" aria-hidden="true">
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 0%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">0</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 12.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 25%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">2</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 37.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 50%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">4</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 62.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 75%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">6</div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 87.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 100%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">8</div>
      </div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="8"
      aria-valuenow="5"
      aria-label="Value"
      tabindex="0"
    ></div>
  </div>
  <div class="pf-v5-c-slider__value">
    <span class="pf-v5-c-form-control">
      <input type="number" value="5" aria-label="Slider value input" />
    </span>
  </div>
</div>

<br />

<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div class="pf-v5-c-slider__steps" aria-hidden="true">
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 0%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">0%</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 25%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 50%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">50%</div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 75%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 100%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">100%</div>
      </div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      tabindex="0"
    ></div>
  </div>
  <div class="pf-v5-c-slider__value">
    <div class="pf-v5-c-input-group">
      <div class="pf-v5-c-input-group__item pf-m-fill">
        <span class="pf-v5-c-form-control">
          <input type="number" value="50" aria-label="Slider value input" />
        </span>
      </div>
      <div class="pf-v5-c-input-group__item pf-m-box">
        <span class="pf-v5-c-input-group__text">%</span>
      </div>
    </div>
  </div>
</div>

<br />

<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      tabindex="0"
    ></div>
  </div>
  <div class="pf-v5-c-slider__value">
    <div class="pf-v5-c-input-group">
      <div class="pf-v5-c-input-group__item pf-m-fill">
        <span class="pf-v5-c-form-control">
          <input type="number" value="50" aria-label="Slider value input" />
        </span>
      </div>
      <div class="pf-v5-c-input-group__item pf-m-box">
        <span class="pf-v5-c-input-group__text">%</span>
      </div>
    </div>
  </div>
</div>

```

### Thumb value input

```html
<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      tabindex="0"
    ></div>
    <div class="pf-v5-c-slider__value pf-m-floating">
      <div class="pf-v5-c-input-group">
        <div class="pf-v5-c-input-group__item pf-m-fill">
          <span class="pf-v5-c-form-control">
            <input type="number" value="50" aria-label="Slider value input" />
          </span>
        </div>
        <div class="pf-v5-c-input-group__item pf-m-box">
          <span class="pf-v5-c-input-group__text">%</span>
        </div>
      </div>
    </div>
  </div>
</div>

```

### Actions

```html
<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__actions">
    <button class="pf-v5-c-button pf-m-plain" type="button" aria-label="Minus">
      <i class="fas fa-fw fa-minus" aria-hidden="true"></i>
    </button>
  </div>
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      tabindex="0"
    ></div>
  </div>
  <div class="pf-v5-c-slider__actions">
    <button class="pf-v5-c-button pf-m-plain" type="button" aria-label="Plus">
      <i class="fas fa-fw fa-plus" aria-hidden="true"></i>
    </button>
  </div>
</div>

<br />
<br />

<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      tabindex="0"
    ></div>
    <div class="pf-v5-c-slider__value pf-m-floating">
      <div class="pf-v5-c-input-group">
        <div class="pf-v5-c-input-group__item pf-m-fill">
          <span class="pf-v5-c-form-control pf-m-disabled">
            <input
              disabled
              type="number"
              value="50"
              aria-label="Slider value input"
            />
          </span>
        </div>
        <div class="pf-v5-c-input-group__item pf-m-box pf-m-disabled">
          <span class="pf-v5-c-input-group__text">%</span>
        </div>
      </div>
    </div>
  </div>
  <div class="pf-v5-c-slider__actions">
    <button class="pf-v5-c-button pf-m-plain" type="button" aria-label="Locked">
      <i class="fas fa-fw fa-lock" aria-hidden="true"></i>
    </button>
  </div>
</div>

<br />
<br />

<div class="pf-v5-c-slider" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      tabindex="0"
    ></div>
    <div class="pf-v5-c-slider__value pf-m-floating">
      <div class="pf-v5-c-input-group">
        <div class="pf-v5-c-input-group__item pf-m-fill">
          <span class="pf-v5-c-form-control">
            <input type="number" value="50" aria-label="Slider value input" />
          </span>
        </div>
        <div class="pf-v5-c-input-group__item pf-m-box">
          <span class="pf-v5-c-input-group__text">%</span>
        </div>
      </div>
    </div>
  </div>
  <div class="pf-v5-c-slider__actions">
    <button class="pf-v5-c-button pf-m-plain" type="button" aria-label="Lock">
      <i class="fas fa-fw fa-lock-open" aria-hidden="true"></i>
    </button>
  </div>
</div>

```

### Disabled

```html
<div
  class="pf-v5-c-slider pf-m-disabled"
  style="--pf-v5-c-slider--value: 62.5%;"
>
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div class="pf-v5-c-slider__steps" aria-hidden="true">
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 0%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">0</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 12.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 25%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">2</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 37.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 50%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">4</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 62.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 75%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">6</div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 87.5%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 100%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">8</div>
      </div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="8"
      aria-valuenow="5"
      aria-label="Value"
      aria-disabled="true"
    ></div>
  </div>
</div>

<br />
<br />

<div class="pf-v5-c-slider pf-m-disabled" style="--pf-v5-c-slider--value: 50%;">
  <div class="pf-v5-c-slider__main">
    <div class="pf-v5-c-slider__rail">
      <div class="pf-v5-c-slider__rail-track"></div>
    </div>
    <div class="pf-v5-c-slider__steps" aria-hidden="true">
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 0%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">0%</div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 25%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step pf-m-active"
        style="--pf-v5-c-slider__step--Left: 50%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">50%</div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 75%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
      </div>
      <div
        class="pf-v5-c-slider__step"
        style="--pf-v5-c-slider__step--Left: 100%;"
      >
        <div class="pf-v5-c-slider__step-tick"></div>
        <div class="pf-v5-c-slider__step-label">100%</div>
      </div>
    </div>
    <div
      class="pf-v5-c-slider__thumb"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow="50"
      aria-label="Value"
      aria-disabled="true"
    ></div>
  </div>
  <div class="pf-v5-c-slider__value">
    <div class="pf-v5-c-input-group">
      <div class="pf-v5-c-input-group__item pf-m-fill">
        <span class="pf-v5-c-form-control pf-m-disabled">
          <input
            disabled
            type="number"
            value="50"
            aria-label="Slider value input"
          />
        </span>
      </div>
      <div class="pf-v5-c-input-group__item pf-m-box pf-m-disabled">
        <span class="pf-v5-c-input-group__text">%</span>
      </div>
    </div>
  </div>
</div>

```

## Documentation

### Accessibility

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `role="slider"` | `.pf-v5-c-slider__thumb` | Identifies the element as a slider. **Required** |
| `tabindex="0"` | `.pf-v5-c-slider__thumb` | Includes the slider thumb in the page tab sequence. **Note:** only for use with non-disabled slider. **Required** |
| `aria-disabled="true"` | `.pf-v5-c-slider.pf-m-disabled .pf-v5-c-slider__thumb` | Indicates that the slider thumb is disabled. **Required** |
| `aria-valuemin="[value]"` | `.pf-v5-c-slider__thumb` | Specifies the minimum value of the slider. **Required** |
| `aria-valuemax="[value]"` | `.pf-v5-c-slider__thumb` | Specifies the maximum value of the slider. **Required** |
| `aria-valuenow="[value]"` | `.pf-v5-c-slider__thumb` | Specifies the current value of the slider. **Required** |

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-slider` | `<div>` | Initiates the slider component. **Required** |
| `.pf-v5-c-slider__main` | `<div>` | Initiates the slider main element. **Required** |
| `.pf-v5-c-slider__rail` | `<div>` | Initiates the slider rail. **Required** |
| `.pf-v5-c-slider__rail-track` | `<div>` | Initiates the slider rail track. **Required** |
| `.pf-v5-c-slider__steps` | `<div>` | Initiates the slider steps. |
| `.pf-v5-c-slider__step` | `<div>` | Initiates a slider step. |
| `.pf-v5-c-slider__step-tick` | `<div>` | Initiates a slider step tick. |
| `.pf-v5-c-slider__step-label` | `<div>` | Initiates a slider step label. |
| `.pf-v5-c-slider__thumb` | `<div>` | Initiates the slider thumb. **Required** |
| `.pf-v5-c-slider__value` | `<div>` | Initiates the slider value. |
| `.pf-v5-c-slider__actions` | `<div>` | Initiates the slider actions. |
| `.pf-m-disabled` | `.pf-v5-c-slider` | Modifies the slider for the disabled state. |
| `.pf-m-floating` | `.pf-v5-c-slider__thumb` | Modifies the slider value to float above the thumb. |
| `--pf-v5-c-slider--value` | `.pf-v5-c-slider` | Applies appropriate slider styles based on the current value. **Required** |
