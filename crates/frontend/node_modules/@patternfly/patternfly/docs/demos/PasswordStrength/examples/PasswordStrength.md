---
id: 'Password strength'
section: components
---## Examples

### Initial state

```html
<form class="pf-v5-c-form" novalidate>
  <div class="pf-v5-c-form__group">
    <div class="pf-v5-c-form__group-label pf-m-info">
      <div class="pf-v5-c-form__group-label-main"><label
          class="pf-v5-c-form__label"
          for="password-strength-demo--initial-password"
        >
          <span class="pf-v5-c-form__label-text">Password</span>&nbsp;<span class="pf-v5-c-form__label-required" aria-hidden="true">&#42;</span></label>&nbsp;<span
          class="pf-v5-c-form__group-label-help"
          aria-label="More information for password field"
          aria-describedby="password-strength-demo--initial-password"
          role="button"
          type="button"
          tabindex="0"
        ><i class="pf-v5-pficon pf-v5-pficon-help" aria-hidden="true"></i></span>
      </div>
      <div class="pf-v5-c-form__group-label-info"></div>
    </div>
    <div class="pf-v5-c-form__group-control">
      <div class="pf-v5-c-input-group">
        <div class="pf-v5-c-input-group__item pf-m-fill">
          <span class="pf-v5-c-form-control pf-m-required">
            <input
              required
              type="text"
              id="password-strength-demo--initial-password"
              name="password-strength-demo--initial-password"
              aria-label="Password input"
              value
              placeholder="Password"
            />
          </span>
        </div>
        <div class="pf-v5-c-input-group__item">
          <button
            class="pf-v5-c-button pf-m-control"
            type="button"
            aria-label="Show password"
          >
            <i class="fas fa-eye-slash" aria-hidden="true"></i>
          </button>
        </div>
      </div>
      <div class="pf-v5-c-form__helper-text" aria-live="polite">
        <div class="pf-v5-c-helper-text">
          <div
            class="pf-v5-c-helper-text__item pf-m-indeterminate pf-m-dynamic"
          >
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-minus" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Must be at least 14 characters</span>
          </div>
          <div
            class="pf-v5-c-helper-text__item pf-m-indeterminate pf-m-dynamic"
          >
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-minus" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Cannot contain the word "redhat"</span>
          </div>
          <div
            class="pf-v5-c-helper-text__item pf-m-indeterminate pf-m-dynamic"
          >
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-minus" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Must include at least 3 of the following: lowercase letters, uppercase letters, numbers, symbols</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</form>

```

### Invalid password

```html
<form class="pf-v5-c-form" novalidate>
  <div class="pf-v5-c-form__group">
    <div class="pf-v5-c-form__group-label pf-m-info">
      <div class="pf-v5-c-form__group-label-main"><label
          class="pf-v5-c-form__label"
          for="password-strength-demo--invalid-password"
        >
          <span class="pf-v5-c-form__label-text">Password</span>&nbsp;<span class="pf-v5-c-form__label-required" aria-hidden="true">&#42;</span></label>&nbsp;<span
          class="pf-v5-c-form__group-label-help"
          aria-label="More information for password field"
          aria-describedby="password-strength-demo--invalid-password"
          role="button"
          type="button"
          tabindex="0"
        ><i class="pf-v5-pficon pf-v5-pficon-help" aria-hidden="true"></i></span>
      </div>
      <div class="pf-v5-c-form__group-label-info"></div>
    </div>
    <div class="pf-v5-c-form__group-control">
      <div class="pf-v5-c-input-group">
        <div class="pf-v5-c-input-group__item pf-m-fill">
          <span class="pf-v5-c-form-control pf-m-required">
            <input
              required
              type="text"
              id="password-strength-demo--invalid-password"
              name="password-strength-demo--invalid-password"
              aria-label="Password input"
              value="Marie$RedHat78"
              placeholder="Password"
            />
          </span>
        </div>
        <div class="pf-v5-c-input-group__item">
          <button
            class="pf-v5-c-button pf-m-control"
            type="button"
            aria-label="Show password"
          >
            <i class="fas fa-eye-slash" aria-hidden="true"></i>
          </button>
        </div>
      </div>
      <div class="pf-v5-c-form__helper-text" aria-live="polite">
        <div class="pf-v5-c-helper-text">
          <div class="pf-v5-c-helper-text__item pf-m-success pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Must be at least 14 characters</span>
          </div>
          <div class="pf-v5-c-helper-text__item pf-m-error pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-exclamation-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Cannot contain the word "redhat"</span>
          </div>
          <div class="pf-v5-c-helper-text__item pf-m-success pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Must include at least 3 of the following: lowercase letters, uppercase letters, numbers, symbols</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</form>

```

### Valid, weak password

```html
<form class="pf-v5-c-form" novalidate>
  <div class="pf-v5-c-form__group">
    <div class="pf-v5-c-form__group-label pf-m-info">
      <div class="pf-v5-c-form__group-label-main"><label
          class="pf-v5-c-form__label"
          for="password-strength-demo--weak-password"
        >
          <span class="pf-v5-c-form__label-text">Password</span>&nbsp;<span class="pf-v5-c-form__label-required" aria-hidden="true">&#42;</span></label>&nbsp;<span
          class="pf-v5-c-form__group-label-help"
          aria-label="More information for password field"
          aria-describedby="password-strength-demo--weak-password"
          role="button"
          type="button"
          tabindex="0"
        ><i class="pf-v5-pficon pf-v5-pficon-help" aria-hidden="true"></i></span>
      </div>
      <div class="pf-v5-c-form__group-label-info">
        <div class="pf-v5-c-helper-text">
          <div class="pf-v5-c-helper-text__item pf-m-error">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-exclamation-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-helper-text__item-text">Weak</span>
          </div>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-form__group-control">
      <div class="pf-v5-c-input-group">
        <div class="pf-v5-c-input-group__item pf-m-fill">
          <span class="pf-v5-c-form-control pf-m-required">
            <input
              required
              type="text"
              id="password-strength-demo--weak-password"
              name="password-strength-demo--weak-password"
              aria-label="Password input"
              value="Marie$Can3Read"
              placeholder="Password"
            />
          </span>
        </div>
        <div class="pf-v5-c-input-group__item">
          <button
            class="pf-v5-c-button pf-m-control"
            type="button"
            aria-label="Show password"
          >
            <i class="fas fa-eye-slash" aria-hidden="true"></i>
          </button>
        </div>
      </div>
      <div class="pf-v5-c-form__helper-text" aria-live="polite">
        <div class="pf-v5-c-helper-text">
          <div class="pf-v5-c-helper-text__item pf-m-success pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Must be at least 14 characters</span>
          </div>
          <div class="pf-v5-c-helper-text__item pf-m-success pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Cannot contain the word "redhat"</span>
          </div>
          <div class="pf-v5-c-helper-text__item pf-m-success pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Must include at least 3 of the following: lowercase letters, uppercase letters, numbers, symbols</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</form>

```

### Valid, strong password

```html
<form class="pf-v5-c-form" novalidate>
  <div class="pf-v5-c-form__group">
    <div class="pf-v5-c-form__group-label pf-m-info">
      <div class="pf-v5-c-form__group-label-main"><label
          class="pf-v5-c-form__label"
          for="password-strength-demo--strong-password"
        >
          <span class="pf-v5-c-form__label-text">Password</span>&nbsp;<span class="pf-v5-c-form__label-required" aria-hidden="true">&#42;</span></label>&nbsp;<span
          class="pf-v5-c-form__group-label-help"
          aria-label="More information for password field"
          aria-describedby="password-strength-demo--strong-password"
          role="button"
          type="button"
          tabindex="0"
        ><i class="pf-v5-pficon pf-v5-pficon-help" aria-hidden="true"></i></span>
      </div>
      <div class="pf-v5-c-form__group-label-info">
        <div class="pf-v5-c-helper-text">
          <div class="pf-v5-c-helper-text__item pf-m-success">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-c-helper-text__item-text">Strong</span>
          </div>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-form__group-control">
      <div class="pf-v5-c-input-group">
        <div class="pf-v5-c-input-group__item pf-m-fill">
          <span class="pf-v5-c-form-control pf-m-required">
            <input
              required
              type="text"
              id="password-strength-demo--strong-password"
              name="password-strength-demo--strong-password"
              aria-label="Password input"
              value="Marie$Can8Read3Pass@Word"
              placeholder="Password"
            />
          </span>
        </div>
        <div class="pf-v5-c-input-group__item">
          <button
            class="pf-v5-c-button pf-m-control"
            type="button"
            aria-label="Show password"
          >
            <i class="fas fa-eye-slash" aria-hidden="true"></i>
          </button>
        </div>
      </div>
      <div class="pf-v5-c-form__helper-text" aria-live="polite">
        <div class="pf-v5-c-helper-text">
          <div class="pf-v5-c-helper-text__item pf-m-success pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Must be at least 14 characters</span>
          </div>
          <div class="pf-v5-c-helper-text__item pf-m-success pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Cannot contain the word "redhat"</span>
          </div>
          <div class="pf-v5-c-helper-text__item pf-m-success pf-m-dynamic">
            <span class="pf-v5-c-helper-text__item-icon">
              <i class="fas fa-fw fa-check-circle" aria-hidden="true"></i>
            </span>
            <span
              class="pf-v5-c-helper-text__item-text"
            >Must include at least 3 of the following: lowercase letters, uppercase letters, numbers, symbols</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</form>

```

## Documentation

This demo implements a password strength meter. It uses multiple helper text items in the form helper text area below the input field to indicate whether the password meets validity criteria. Once the validity criteria are met, it also places helper text in the info area above the input field to indicate the strength of the password.
